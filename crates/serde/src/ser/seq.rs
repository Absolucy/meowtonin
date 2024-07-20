// SPDX-License-Identifier: 0BSD
use super::ByondSerializer;
use crate::error::SerializeError;
use meowtonin::ByondValue;
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, SerializeTupleStruct};

pub(crate) struct ByondSeqSerializer<'a> {
	serializer: &'a mut ByondSerializer,
	list: ByondValue,
}

impl<'a> ByondSeqSerializer<'a> {
	pub fn new(serializer: &'a mut ByondSerializer) -> Result<Self, SerializeError> {
		Ok(Self {
			serializer,
			list: ByondValue::new_list()?,
		})
	}
}

impl<'a> SerializeSeq for ByondSeqSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		self.list
			.push_list(value.serialize(&mut *self.serializer)?)?;
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.list)
	}
}

impl<'a> SerializeTuple for ByondSeqSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		self.list
			.push_list(value.serialize(&mut *self.serializer)?)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.list)
	}
}

impl<'a> SerializeTupleStruct for ByondSeqSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		self.list
			.push_list(value.serialize(&mut *self.serializer)?)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.list)
	}
}
