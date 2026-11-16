// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, ToByond};
use std::{
	borrow::Cow,
	collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
	hash::Hash,
};

impl<const LENGTH: usize, Value> ToByond for [Value; LENGTH]
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let list = self
			.iter()
			.map(|value| value.to_byond())
			.collect::<ByondResult<Vec<ByondValue>>>()?;
		let mut value = ByondValue::new_list()?;
		value.write_list(list)?;
		Ok(value)
	}
}

impl<Value> ToByond for &[Value]
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let list = self
			.iter()
			.map(|value| value.to_byond())
			.collect::<ByondResult<Vec<ByondValue>>>()?;
		let mut value = ByondValue::new_list()?;
		value.write_list(list)?;
		Ok(value)
	}
}

impl<const LENGTH: usize, Value> ToByond for &[Value; LENGTH]
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let list = self
			.iter()
			.map(|value| value.to_byond())
			.collect::<ByondResult<Vec<ByondValue>>>()?;
		let mut value = ByondValue::new_list()?;
		value.write_list(list)?;
		Ok(value)
	}
}

impl<Value> ToByond for Vec<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let list = self
			.iter()
			.map(|value| value.to_byond())
			.collect::<ByondResult<Vec<ByondValue>>>()?;
		let mut value = ByondValue::new_list()?;
		value.write_list(list)?;
		Ok(value)
	}
}

fn assoc_list_from<'a, Key, Value>(
	pairs: impl IntoIterator<Item = (&'a Key, &'a Value)>,
) -> ByondResult<ByondValue>
where
	Key: ToByond + 'a,
	Value: ToByond + 'a,
{
	let mut list = ByondValue::new_list()?;
	for (key, value) in pairs {
		let key = key.to_byond()?;
		if key.is_number() {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed(
					"string or object map key (a /list can't have number keys)",
				),
				got: Cow::Borrowed("number"),
			});
		}
		let value = value.to_byond()?;
		list.write_list_index(&key, &value)?;
	}
	Ok(list)
}

impl<Key, Value> ToByond for Vec<(Key, Value)>
where
	Key: ToByond,
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		assoc_list_from(self.iter().map(|(key, value)| (key, value)))
	}
}

impl<Value> ToByond for VecDeque<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let list = self
			.iter()
			.map(|value| value.to_byond())
			.collect::<ByondResult<Vec<ByondValue>>>()?;
		let mut value = ByondValue::new_list()?;
		value.write_list(list)?;
		Ok(value)
	}
}

impl<Key, Value> ToByond for VecDeque<(Key, Value)>
where
	Key: ToByond,
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		assoc_list_from(self.iter().map(|(key, value)| (key, value)))
	}
}

impl<Key, Value, Hasher> ToByond for HashMap<Key, Value, Hasher>
where
	Key: ToByond + Hash,
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		assoc_list_from(self)
	}
}

impl<Key, Hasher> ToByond for HashSet<Key, Hasher>
where
	Key: ToByond + Eq + Hash,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let list = self
			.iter()
			.map(|key| key.to_byond())
			.collect::<ByondResult<Vec<ByondValue>>>()?;
		let mut value = ByondValue::new_list()?;
		value.write_list(list)?;
		Ok(value)
	}
}

impl<Key, Value> ToByond for BTreeMap<Key, Value>
where
	Key: ToByond + Ord,
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		assoc_list_from(self)
	}
}

impl<Key> ToByond for BTreeSet<Key>
where
	Key: ToByond + Eq + Ord,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let list = self
			.iter()
			.map(|key| key.to_byond())
			.collect::<ByondResult<Vec<ByondValue>>>()?;
		let mut value = ByondValue::new_list()?;
		value.write_list(list)?;
		Ok(value)
	}
}
