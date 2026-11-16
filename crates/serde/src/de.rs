// SPDX-License-Identifier: 0BSD
use crate::error::DeserializeError;
use meowtonin::{ByondValue, ByondValueType, FromByond};
use serde::de::{
	self, DeserializeSeed, Deserializer, EnumAccess, MapAccess, SeqAccess, VariantAccess,
};

pub(crate) struct ByondDeserializer {
	pub(crate) value: ByondValue,
}

fn visit_number<'de, V>(num: f32, visitor: V) -> Result<V::Value, DeserializeError>
where
	V: de::Visitor<'de>,
{
	if num.fract() != 0.0 {
		return visitor.visit_f32(num);
	}
	if (0.0..u64::MAX as f32).contains(&num) {
		visitor.visit_u64(num as u64)
	} else if (i64::MIN as f32..0.0).contains(&num) {
		visitor.visit_i64(num as i64)
	} else {
		visitor.visit_f32(num)
	}
}

impl<'de> Deserializer<'de> for ByondDeserializer {
	type Error = DeserializeError;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let val = self.value.get_type();
		match val {
			ByondValueType::Null => visitor.visit_unit(),
			ByondValueType::String => self.deserialize_string(visitor),
			ByondValueType::Number => visit_number(self.value.get_number()?, visitor),
			ByondValueType::Pointer => Err(DeserializeError::Unexpected(
				"serializable value (null, str, num, list, string-ish)".into(),
				"pointer".into(),
			)),
			ByondValueType::List => {
				let pairs = self.value.read_assoc_list()?;
				if meowtonin::misc::is_likely_assoc(&pairs) {
					visitor.visit_map(ByondMapAccess::new(pairs))
				} else {
					let items = pairs.into_iter().map(|[key, _]| key).collect();
					visitor.visit_seq(ByondSeqAccess::new(items))
				}
			}
			_ if self.value.is_list() => {
				if self.value.is_likely_assoc()? {
					self.deserialize_map(visitor)
				} else {
					self.deserialize_seq(visitor)
				}
			}
			_ => match self.value.get_string() {
				Ok(string) if !string.trim().is_empty() => visitor.visit_string(string),
				_ => Err(DeserializeError::Unexpected(
					"serializable value (null, str, num, list, string-ish)".into(),
					val.name(),
				)),
			},
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_bool(bool::from_byond(self.value)?)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_i8(i8::from_byond(self.value)?)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_i16(i16::from_byond(self.value)?)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_i32(i32::from_byond(self.value)?)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_i64(i64::from_byond(self.value)?)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_u8(u8::from_byond(self.value)?)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_u16(u16::from_byond(self.value)?)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_u32(u32::from_byond(self.value)?)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_u64(u64::from_byond(self.value)?)
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_f32(f32::from_byond(self.value)?)
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_f64(f64::from_byond(self.value)?)
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let string = String::from_byond(self.value)?;
		let char = string.chars().next().ok_or(DeserializeError::EndOfArray)?;
		visitor.visit_char(char)
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_str(&String::from_byond(self.value)?)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_string(String::from_byond(self.value)?)
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_byte_buf(Vec::<u8>::from_byond(self.value)?)
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_bytes(visitor)
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		if self.value.is_null() {
			visitor.visit_none()
		} else {
			visitor.visit_some(self)
		}
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_unit()
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_unit()
	}

	fn deserialize_unit_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_unit()
	}

	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_seq(ByondSeqAccess::new(self.value.read_list()?))
	}

	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_tuple(len, visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let pairs = self
			.value
			.iter()?
			.map(|(key, value)| [key, value])
			.collect();
		visitor.visit_map(ByondMapAccess::new(pairs))
	}

	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_map(visitor)
	}

	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let kind = self.value.get_type();
		if kind == ByondValueType::String {
			return visitor.visit_enum(ByondEnumAccess {
				variant: self.value,
				contents: ByondValue::NULL,
			});
		}
		if !self.value.is_list() {
			return Err(DeserializeError::Unexpected(
				"enum variant name or list(variant = contents)".into(),
				kind.name(),
			));
		}
		let mut pairs = self.value.iter()?;
		let (Some((variant, contents)), None) = (pairs.next(), pairs.next()) else {
			return Err(DeserializeError::Unexpected(
				"list(variant = contents) with exactly one entry".into(),
				"a list with zero or several entries".into(),
			));
		};
		visitor.visit_enum(ByondEnumAccess { variant, contents })
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_string(self.value.get_string()?)
	}
}

struct ByondSeqAccess {
	items: std::vec::IntoIter<ByondValue>,
}

impl ByondSeqAccess {
	fn new(items: Vec<ByondValue>) -> Self {
		Self {
			items: items.into_iter(),
		}
	}
}

impl<'de> SeqAccess<'de> for ByondSeqAccess {
	type Error = DeserializeError;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self.items.next() {
			Some(value) => seed.deserialize(ByondDeserializer { value }).map(Some),
			None => Ok(None),
		}
	}
}

struct ByondMapAccess {
	pairs: std::vec::IntoIter<[ByondValue; 2]>,
	value: Option<ByondValue>,
}

impl ByondMapAccess {
	fn new(pairs: Vec<[ByondValue; 2]>) -> Self {
		Self {
			pairs: pairs.into_iter(),
			value: None,
		}
	}
}

impl<'de> MapAccess<'de> for ByondMapAccess {
	type Error = DeserializeError;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: DeserializeSeed<'de>,
	{
		let Some([key, value]) = self.pairs.next() else {
			return Ok(None);
		};
		self.value = Some(value);
		seed.deserialize(ByondDeserializer { value: key }).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		let value = self.value.take().ok_or(DeserializeError::EndOfArray)?;
		seed.deserialize(ByondDeserializer { value })
	}
}

struct ByondEnumAccess {
	variant: ByondValue,
	contents: ByondValue,
}

impl<'de> EnumAccess<'de> for ByondEnumAccess {
	type Error = DeserializeError;
	type Variant = ByondDeserializer;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		let variant = seed.deserialize(ByondDeserializer {
			value: self.variant,
		})?;
		Ok((variant, ByondDeserializer {
			value: self.contents,
		}))
	}
}

impl<'de> VariantAccess<'de> for ByondDeserializer {
	type Error = DeserializeError;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		seed.deserialize(self)
	}

	fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_map(visitor)
	}
}
