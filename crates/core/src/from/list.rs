// SPDX-License-Identifier: 0BSD
use crate::{ByondResult, ByondValue, FromByond};
use std::{
	collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
	hash::{BuildHasher, Hash},
};

impl<Value> FromByond for Vec<Value>
where
	Value: FromByond,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value
			.values()?
			.map(|value| Value::from_byond(&value))
			.collect()
	}
}

impl<Key, Value> FromByond for Vec<(Key, Value)>
where
	Key: FromByond,
	Value: FromByond,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value
			.iter()?
			.map(|(key, value)| Ok((Key::from_byond(&key)?, Value::from_byond(&value)?)))
			.collect()
	}
}

impl<Value> FromByond for VecDeque<Value>
where
	Value: FromByond,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value
			.values()?
			.map(|value| Value::from_byond(&value))
			.collect()
	}
}

impl<Key, Value> FromByond for VecDeque<(Key, Value)>
where
	Key: FromByond,
	Value: FromByond,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value
			.iter()?
			.map(|(key, value)| Ok((Key::from_byond(&key)?, Value::from_byond(&value)?)))
			.collect()
	}
}

impl<Key, Value, Hasher> FromByond for HashMap<Key, Value, Hasher>
where
	Key: FromByond + Hash + Eq,
	Value: FromByond,
	Hasher: BuildHasher + Default,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let mut out =
			HashMap::with_capacity_and_hasher(value.length::<usize>()?, Hasher::default());
		for (key, value) in value.iter()? {
			out.insert(Key::from_byond(&key)?, Value::from_byond(&value)?);
		}
		Ok(out)
	}
}

impl<Value, Hasher> FromByond for HashSet<Value, Hasher>
where
	Value: FromByond + Hash + Eq,
	Hasher: BuildHasher + Default,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let mut out =
			HashSet::with_capacity_and_hasher(value.length::<usize>()?, Hasher::default());
		for key in value.values()? {
			out.insert(Value::from_byond(&key)?);
		}
		Ok(out)
	}
}

impl<Key, Value> FromByond for BTreeMap<Key, Value>
where
	Key: FromByond + Ord,
	Value: FromByond,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let mut out = BTreeMap::new();
		for (key, value) in value.iter()? {
			out.insert(Key::from_byond(&key)?, Value::from_byond(&value)?);
		}
		Ok(out)
	}
}

impl<Value> FromByond for BTreeSet<Value>
where
	Value: FromByond + Eq + Ord,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let mut out = BTreeSet::new();
		for value in value.values()? {
			out.insert(Value::from_byond(&value)?);
		}
		Ok(out)
	}
}
