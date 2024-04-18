// SPDX-License-Identifier: 0BSD
use crate::{byond, ByondError, ByondResult, ByondValue, FromByond, ToByond};
use std::mem::MaybeUninit;

impl ByondValue {
	#[inline]
	pub fn new_list() -> ByondResult<Self> {
		unsafe {
			let mut value = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_CreateList(value.as_mut_ptr()))?;
			Ok(Self(value.assume_init()).persist())
		}
	}

	pub fn read_list(&self) -> ByondResult<Vec<Self>> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		unsafe {
			let mut buffer = Vec::<ByondValue>::new();
			let mut needed_len = 0;
			if byond().Byond_ReadList(&self.0, buffer.as_mut_ptr().cast(), &mut needed_len) {
				// Safety: if this returns true, then the buffer was large enough, and thus
				// needed_len <= capacity.
				buffer.set_len(needed_len);
				return Ok(crate::inc_ref_list_contents(buffer));
			}
			buffer.reserve(needed_len.saturating_sub(buffer.len()));
			map_byond_error!(byond().Byond_ReadList(
				&self.0,
				buffer.as_mut_ptr().cast(),
				&mut needed_len
			))?;
			// Safety: needed_len is always <= capacity here,
			// unless BYOND did a really bad fucky wucky.
			buffer.set_len(needed_len);
			Ok(crate::inc_ref_list_contents(buffer))
		}
	}

	pub fn read_assoc_list(&self) -> ByondResult<Vec<[Self; 2]>> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		unsafe {
			let mut buffer = Vec::<ByondValue>::new();
			let mut needed_len = 0;
			if byond().Byond_ReadListAssoc(&self.0, buffer.as_mut_ptr().cast(), &mut needed_len) {
				// Safety: if this returns true, then the buffer was large enough, and thus
				// needed_len <= capacity.
				buffer.set_len(needed_len);
				// Safety: with assoc lists, len should always be a multiple of 2.
				return Ok(crate::inc_ref_assoc_list_contents(stupid_assoc_cast(
					buffer,
				)));
			}
			buffer.reserve(needed_len.saturating_sub(buffer.len()));
			map_byond_error!(byond().Byond_ReadListAssoc(
				&self.0,
				buffer.as_mut_ptr().cast(),
				&mut needed_len
			))?;
			// Safety: needed_len is always <= capacity here,
			// unless BYOND did a really bad fucky wucky.
			buffer.set_len(needed_len);
			// Safety: with assoc lists, len should always be a multiple of 2.
			Ok(crate::inc_ref_assoc_list_contents(stupid_assoc_cast(
				buffer,
			)))
		}
	}

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
			let result = Self(result.assume_init()).persist();
			Value::from_byond(&result)
		}
	}

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
		self.call("Add", [[value]])?; // byond moment
		Ok(())
	}

	/// Pops a value from a list
	pub fn pop_list(&mut self) -> ByondResult<Option<ByondValue>> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		let len = self.length::<usize>()?;
		if len == 0 {
			return Ok(None);
		}
		let value = self.read_list_index(&len)?;
		self.call("Remove", [&value])?;
		Ok(Some(value))
	}

	/// Iterates through the assoc values of the list if this value is a list,
	/// if the value isn't a list then it returns an error. Non assoc lists will
	/// have the second field of the tuple be null (key, value) for proper assoc
	/// lists
	pub fn iter(&self) -> ByondResult<impl Iterator<Item = (ByondValue, ByondValue)> + '_> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		let len = self.length::<usize>()?;
		Ok(ListIterator {
			value: self,
			len,
			ctr: 1,
		})
	}

	pub fn values(&self) -> ByondResult<impl Iterator<Item = ByondValue> + '_> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		let len = self.length::<usize>()?;
		Ok(ValueIterator {
			value: self,
			len,
			ctr: 1,
		})
	}
}

struct ValueIterator<'a> {
	value: &'a ByondValue,
	len: usize,
	ctr: usize,
}
impl<'a> Iterator for ValueIterator<'a> {
	type Item = ByondValue;
	fn next(&mut self) -> Option<Self::Item> {
		if self.ctr <= self.len {
			let value = self
				.value
				.read_list_index(&ByondValue::new_num(self.ctr as f32))
				.ok()?;
			self.ctr += 1;
			Some(value)
		} else {
			None
		}
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(self.len))
	}
}

struct ListIterator<'a> {
	value: &'a ByondValue,
	len: usize,
	ctr: usize,
}
impl<'a> Iterator for ListIterator<'a> {
	type Item = (ByondValue, ByondValue);
	fn next(&mut self) -> Option<Self::Item> {
		if self.ctr <= self.len {
			let key = self
				.value
				.read_list_index(&ByondValue::new_num(self.ctr as f32))
				.ok()?;
			let value = self.value.read_list_index(&key).ok()?;
			self.ctr += 1;
			Some((key, value))
		} else {
			None
		}
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(self.len))
	}
}

// Safety: `list` should always have a length that is a multiple of 2.
unsafe fn stupid_assoc_cast(list: Vec<ByondValue>) -> Vec<[ByondValue; 2]> {
	use crate::sys::CByondValue;

	assert_eq!(
		std::mem::size_of::<CByondValue>() * 2,
		std::mem::size_of::<[ByondValue; 2]>()
	);
	if list.len() % 2 != 0 {
		std::hint::unreachable_unchecked();
	}
	let stupid: Vec<CByondValue> = list.into_iter().map(|x| x.0).collect();
	let assoc_list: Vec<[CByondValue; 2]> = bytemuck::try_cast_vec(stupid).unwrap_unchecked();
	assoc_list
		.into_iter()
		.map(|[a, b]| [ByondValue(a), ByondValue(b)])
		.collect()
}
