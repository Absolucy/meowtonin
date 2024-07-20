// SPDX-License-Identifier: 0BSD
pub mod de;
pub mod error;
pub mod ser;
pub mod value;

use meowtonin::ByondValue;

pub use crate::value::ByondSerde;

pub fn serialize<Value>(v: &Value) -> Result<ByondValue, error::SerializeError>
where
	Value: serde::Serialize,
{
	v.serialize(&mut ser::ByondSerializer)
}

pub fn deserialize<'de, Value>(value: ByondValue) -> Result<Value, error::DeserializeError>
where
	Value: serde::Deserialize<'de>,
{
	Value::deserialize(&de::ByondDeserializer { value })
}
