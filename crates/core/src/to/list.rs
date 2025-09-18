// SPDX-License-Identifier: 0BSD
use crate::{ByondResult, ByondValue, ToByond};
use std::{
	collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
	hash::Hash,
};

impl<const LENGTH: usize, Value> ToByond for [Value; LENGTH]
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond ([Value; LENGTH])");
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
		tracy::zone!("to_byond (&[Value])");
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
		tracy::zone!("to_byond (&[Value; LENGTH])");
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
		tracy::zone!("to_byond (Vec<Value>)");
		let list = self
			.iter()
			.map(|value| value.to_byond())
			.collect::<ByondResult<Vec<ByondValue>>>()?;
		let mut value = ByondValue::new_list()?;
		value.write_list(list)?;
		Ok(value)
	}
}

impl<Key, Value> ToByond for Vec<(Key, Value)>
where
	Key: ToByond,
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (Vec<(Key, Value)>)");
		let mut list = ByondValue::new_list()?;
		for (key, value) in self {
			let key = key.to_byond()?;
			let value = value.to_byond()?;
			list.write_list_index(&key, &value)?;
		}
		Ok(list)
	}
}

impl<Value> ToByond for VecDeque<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (VecDeque<Value>)");
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
		tracy::zone!("to_byond (VecDeque<(Key, Value)>)");
		let mut list = ByondValue::new_list()?;
		for (key, value) in self {
			let key = key.to_byond()?;
			let value = value.to_byond()?;
			list.write_list_index(&key, &value)?;
		}
		Ok(list)
	}
}

impl<Key, Value, Hasher> ToByond for HashMap<Key, Value, Hasher>
where
	Key: ToByond + Hash,
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (HashMap<Key, Value, Hasher>)");
		let mut list = ByondValue::new_list()?;
		for (key, value) in self {
			let key = key.to_byond()?;
			let value = value.to_byond()?;
			list.write_list_index(&key, &value)?;
		}
		Ok(list)
	}
}

impl<Key, Hasher> ToByond for HashSet<Key, Hasher>
where
	Key: ToByond + Eq + Hash,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (HashSet<Key, Hasher>)");
		let mut list = ByondValue::new_list()?;
		for key in self {
			let key = key.to_byond()?;
			list.push_list(key)?;
		}
		Ok(list)
	}
}

impl<Key, Value> ToByond for BTreeMap<Key, Value>
where
	Key: ToByond + Ord,
	Value: ToByond + Ord,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (BTreeMap<Key, Value>)");
		let mut list = ByondValue::new_list()?;
		for (key, value) in self {
			let key = key.to_byond()?;
			let value = value.to_byond()?;
			list.write_list_index(&key, &value)?;
		}
		Ok(list)
	}
}

impl<Key> ToByond for BTreeSet<Key>
where
	Key: ToByond + Eq + Ord,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (BTreeSet<Key>)");
		let mut list = ByondValue::new_list()?;
		for key in self {
			let key = key.to_byond()?;
			list.push_list(key)?;
		}
		Ok(list)
	}
}
