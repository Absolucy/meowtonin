// SPDX-License-Identifier: 0BSD
use super::ByondSerializer;
use crate::error::SerializeError;
use meowtonin::ByondValue;
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, SerializeTupleStruct};

pub(crate) struct ByondSeqSerializer<'a> {
	serializer: &'a mut ByondSerializer,
	items: Vec<ByondValue>,
}

impl<'a> ByondSeqSerializer<'a> {
	pub fn new(serializer: &'a mut ByondSerializer, len: Option<usize>) -> Self {
		Self {
			serializer,
			items: Vec::with_capacity(len.unwrap_or(0)),
		}
	}

	fn push<T>(&mut self, value: &T) -> Result<(), SerializeError>
	where
		T: Serialize + ?Sized,
	{
		self.items.push(value.serialize(&mut *self.serializer)?);
		Ok(())
	}

	fn finish(self) -> Result<ByondValue, SerializeError> {
		let mut list = ByondValue::new_list()?;
		list.write_list(self.items)?;
		Ok(list)
	}
}

impl SerializeSeq for ByondSeqSerializer<'_> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		self.push(value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.finish()
	}
}

impl SerializeTuple for ByondSeqSerializer<'_> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		self.push(value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.finish()
	}
}

impl SerializeTupleStruct for ByondSeqSerializer<'_> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		self.push(value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.finish()
	}
}
