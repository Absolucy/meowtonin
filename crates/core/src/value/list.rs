// SPDX-License-Identifier: 0BSD
use crate::{
	ByondError, ByondResult, ByondValue, ByondValueType, FromByond, ToByond, byond, sys::u4c,
};
use std::mem::MaybeUninit;

impl ByondValue {
	/// Creates an empty list (referenced). Equivalent to `list()`.
	pub fn new_list() -> ByondResult<Self> {
		unsafe {
			let mut value = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_CreateList(value.as_mut_ptr()))?;
			Ok(Self(unsafe { value.assume_init() }))
		}
	}

	/// Creates a new dimensional list (referenced).
	/// Equivalent to `new /list(sizes[0], sizes[1], ...)`.
	pub fn new_dimensional_list<Sizes>(sizes: Sizes) -> ByondResult<Self>
	where
		Sizes: AsRef<[u4c]>,
	{
		let sizes = sizes.as_ref();
		unsafe {
			let mut value = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_CreateDimensionalList(
				value.as_mut_ptr(),
				sizes.as_ptr(),
				sizes.len() as _
			))?;
			Ok(Self(unsafe { value.assume_init() }))
		}
	}

	/// Creates a list (referenced) with a given length.
	/// Equivalent to `new /list(len)`.
	pub fn new_list_len(length: u4c) -> ByondResult<Self> {
		unsafe {
			let mut value = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_CreateListLen(value.as_mut_ptr(), length))?;
			Ok(Self(unsafe { value.assume_init() }))
		}
	}

	/// Reads items (referenced) from a list.
	pub fn read_list(&self) -> ByondResult<Vec<Self>> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		crate::sync::on_main_thread(|| {
			let values = unsafe {
				crate::misc::with_buffer::<_, ByondValue, _, _>(
					None,
					|ptr, len| byond().Byond_ReadList(&self.0, ptr.cast(), len),
					|buffer| buffer,
				)
			}?;
			// only a plain /list hands its items over unreferenced
			if self.get_type() == ByondValueType::List {
				for value in &values {
					if value.get_type().is_ref_counted() {
						value.inc_ref();
					}
				}
			}
			Ok(values)
		})
	}

	/// Reads key,value pairs (referenced) from a `/list` or `/alist`.
	pub fn read_assoc_list(&self) -> ByondResult<Vec<[Self; 2]>> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		// off the main thread BYOND runs Byond_ReadList here instead
		crate::sync::on_main_thread(|| unsafe {
			// BYOND counts single items, the buffer counts pairs
			crate::misc::with_buffer::<_, [ByondValue; 2], _, _>(
				None,
				|ptr, pairs| {
					let mut items = *pairs * 2;
					let ok = byond().Byond_ReadListAssoc(&self.0, ptr.cast(), &mut items);
					*pairs = items.div_ceil(2);
					ok
				},
				|buffer| buffer,
			)
		})
	}

	/// Returns if this is likely an associative list or not.
	///
	/// A `/alist` or `vars` list always is. A `/list` is checked by its
	/// contents: a non-null value means it probably is, while a number key or a
	/// repeated key means it is a plain list. Every other pseudo-list
	/// (`contents` and friends) has no values, so it is not.
	///
	/// Do not rely on this being 100% accurate.
	pub fn is_likely_assoc(&self) -> ByondResult<bool> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		match self.get_type() {
			ByondValueType::Alist => Ok(true),
			ByondValueType::List => Ok(crate::misc::is_likely_assoc(&self.read_assoc_list()?)),
			kind if ByondValueType::VARS_TYPES.contains(&kind) => Ok(true),
			_ => Ok(false),
		}
	}

	/// Writes items to a list, in place of old contents.
	pub fn write_list<List>(&mut self, contents: List) -> ByondResult<()>
	where
		List: IntoIterator<Item = Self>,
	{
		let contents = contents.into_iter().collect::<Vec<_>>();
		map_byond_error!(byond().Byond_WriteList(
			&self.0,
			contents.as_ptr().cast(),
			contents.len() as _
		))
	}

	/// Reads an item (referenced) from a list.
	pub fn read_list_index<Idx, Value>(&self, idx: &Idx) -> ByondResult<Value>
	where
		Idx: ToByond,
		Value: FromByond,
	{
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		unsafe {
			let mut result = MaybeUninit::uninit();
			let idx = idx.to_byond()?;
			map_byond_error!(byond().Byond_ReadListIndex(&self.0, &idx.0, result.as_mut_ptr()))?;
			Value::from_byond(Self(unsafe { result.assume_init() }))
		}
	}

	/// Writes an item to a list.
	pub fn write_list_index<Idx, Value>(&mut self, idx: Idx, value: Value) -> ByondResult<()>
	where
		Idx: ToByond,
		Value: ToByond,
	{
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		let idx = idx.to_byond()?;
		let value = value.to_byond()?;
		map_byond_error!(byond().Byond_WriteListIndex(&self.0, &idx.0, &value.0))
	}

	/// Pushes a value into a list
	pub fn push_list(&mut self, value: ByondValue) -> ByondResult<()> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		self.call::<_, _, _, ()>("Add", [[value]])?; // byond moment
		Ok(())
	}

	/// Pops a value from a list
	pub fn pop_list(&mut self) -> ByondResult<Option<ByondValue>> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		// alist[n] is a key lookup,
		// so "last" is the last key in the alist's "sorted" order
		if self.get_type() == ByondValueType::Alist {
			// reads every key to pop one,
			// maybe redo this if someone pops big alists in a loop?
			return crate::sync::on_main_thread(|| {
				let Some(key) = self.read_list()?.pop() else {
					return Ok(None);
				};
				// not Remove(): that would split a key that is itself a list
				let removed = unsafe { byond().Byond_ListRemove(&self.0, &key.0, 1) };
				if removed == 0 {
					return Err(ByondError::get_last_byond_error());
				}
				Ok(Some(key))
			});
		}
		let len = self.length()?;
		if len == 0 {
			return Ok(None);
		}
		let value = self.read_list_index(&len)?;
		self.write_var("len", len - 1)?;
		Ok(Some(value))
	}

	/// Iterates through the assoc values of the list if this value is a list.
	///
	/// If the value isn't a list then it returns an error.
	///
	/// Non assoc lists will have the second field of the tuple be null (key,
	/// value) for proper assoc lists.
	///
	/// Pseudo-lists like `contents` have no values to look up, and BYOND raises
	/// a runtime error when asked for `contents[obj]`, so on those this returns
	/// an error. Use [`values`](Self::values) for them.
	pub fn iter(&self) -> ByondResult<impl Iterator<Item = (ByondValue, ByondValue)>> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		let pairs = match self.get_type() {
			ByondValueType::List | ByondValueType::Alist => self.read_assoc_list()?,
			// ReadListAssoc refuses these, and a number index here means a
			// position, so a number item has no value to look up
			_ => crate::sync::on_main_thread(|| {
				self.read_list()?
					.into_iter()
					.map(|key| {
						if key.is_number() {
							return Ok([key, ByondValue::NULL]);
						}
						let value = self.read_list_index(&key)?;
						Ok([key, value])
					})
					.collect::<ByondResult<Vec<_>>>()
			})?,
		};
		Ok(pairs.into_iter().map(|[key, value]| (key, value)))
	}

	pub fn values(&self) -> ByondResult<impl Iterator<Item = ByondValue>> {
		Ok(self.read_list()?.into_iter())
	}
}
