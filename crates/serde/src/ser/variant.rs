// SPDX-License-Identifier: 0BSD
use super::ByondSerializer;
use crate::error::SerializeError;
use meowtonin::{ByondValue, ToByond};
use serde::ser::{Serialize, SerializeTupleVariant};

pub(crate) struct ByondVariantSerializer<'a> {
	pub serializer: &'a mut ByondSerializer,
	pub variant: &'static str,
	pub sequence: ByondValue,
}

impl<'a> ByondVariantSerializer<'a> {
	pub fn new(
		serializer: &'a mut ByondSerializer,
		variant: &'static str,
	) -> Result<Self, SerializeError> {
		Ok(Self {
			serializer,
			variant,
			sequence: ByondValue::new_list()?,
		})
	}
}

impl<'a> SerializeTupleVariant for ByondVariantSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.sequence
			.push_list(value.serialize(&mut *self.serializer)?)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		let mut list = ByondValue::new_list()?;
		list.write_list_index(self.variant.to_byond()?, self.sequence.to_byond()?)?;
		Ok(list)
	}
}
