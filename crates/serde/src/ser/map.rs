// SPDX-License-Identifier: 0BSD
use super::ByondSerializer;
use crate::error::SerializeError;
use meowtonin::{byondval, ByondResult, ByondValue, ToByond};
use serde::ser::{Serialize, SerializeMap, SerializeStruct, SerializeStructVariant};

pub(crate) struct ByondMapSerializer<'a> {
	pub serializer: &'a mut ByondSerializer,
	pub map: ByondValue,
	pub variant: Option<&'static str>,
	pub key: ByondValue,
}

impl<'a> ByondMapSerializer<'a> {
	pub fn new(
		serializer: &'a mut ByondSerializer,
		variant: Option<&'static str>,
	) -> ByondResult<Self> {
		Ok(Self {
			serializer,
			map: ByondValue::new_list()?,
			variant,
			key: ByondValue::null(),
		})
	}
}

impl SerializeMap for ByondMapSerializer<'_> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		self.key = key.serialize(&mut *self.serializer)?;
		Ok(())
	}

	fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		let key = std::mem::take(&mut self.key);
		let value = value.serialize(&mut *self.serializer)?;
		self.map
			.write_list_index(&key, &value)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.map)
	}
}

impl SerializeStruct for ByondMapSerializer<'_> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		let key = key.to_byond()?;
		let value = value.serialize(&mut *self.serializer)?;
		self.map
			.write_list_index(&key, &value)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.map)
	}
}

impl SerializeStructVariant for ByondMapSerializer<'_> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		let key = key.to_byond()?;
		let value = value.serialize(&mut *self.serializer)?;
		self.map
			.write_list_index(&key, &value)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		let mut list = ByondValue::new_list()?;
		if let Some(variant) = self.variant.map(|variant| byondval!(variant)) {
			list.write_list_index(&variant, &self.map)?;
		}
		Ok(list)
	}
}
