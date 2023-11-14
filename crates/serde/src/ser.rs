// SPDX-License-Identifier: 0BSD
mod map;
mod seq;
mod variant;

use crate::error::SerializeError;
use meowtonin::{ByondValue, ToByond};
use serde::ser::{Serialize, Serializer};

pub(crate) struct ByondSerializer;

impl<'a> Serializer for &'a mut ByondSerializer {
	type Ok = ByondValue;
	type Error = SerializeError;

	type SerializeSeq = seq::ByondSeqSerializer<'a>;
	type SerializeTuple = seq::ByondSeqSerializer<'a>;
	type SerializeTupleStruct = seq::ByondSeqSerializer<'a>;
	type SerializeTupleVariant = variant::ByondVariantSerializer<'a>;
	type SerializeMap = map::ByondMapSerializer<'a>;
	type SerializeStruct = map::ByondMapSerializer<'a>;
	type SerializeStructVariant = map::ByondMapSerializer<'a>;

	#[inline]
	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		(v as i32).to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		(v as u32).to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		(v as f32).to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		v.to_string().to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		v.to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		v.to_vec().to_byond().map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Ok(ByondValue::null())
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: Serialize,
	{
		value.serialize(self)
	}

	#[inline]
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Ok(ByondValue::null())
	}

	#[inline]
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}

	#[inline]
	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(variant)
	}

	#[inline]
	fn serialize_newtype_struct<T: ?Sized>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: Serialize,
	{
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: Serialize,
	{
		let mut list = ByondValue::new_list()?;
		let value = value.serialize(self)?;
		list.write_list_index(variant, &value)?;
		Ok(list)
	}

	#[inline]
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		seq::ByondSeqSerializer::new(self)
	}

	#[inline]
	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		seq::ByondSeqSerializer::new(self)
	}

	#[inline]
	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		seq::ByondSeqSerializer::new(self)
	}

	#[inline]
	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		variant::ByondVariantSerializer::new(self, variant)
	}

	#[inline]
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		map::ByondMapSerializer::new(self, None).map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		map::ByondMapSerializer::new(self, None).map_err(SerializeError::from)
	}

	#[inline]
	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		map::ByondMapSerializer::new(self, Some(variant)).map_err(SerializeError::from)
	}
}
