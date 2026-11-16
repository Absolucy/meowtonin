// SPDX-License-Identifier: 0BSD
use super::ByondSerializer;
use crate::error::SerializeError;
use meowtonin::ByondValue;
use serde::ser::{Serialize, SerializeTupleVariant};

pub(crate) struct ByondVariantSerializer<'a> {
	pub serializer: &'a mut ByondSerializer,
	pub variant: &'static str,
	pub items: Vec<ByondValue>,
}

impl<'a> ByondVariantSerializer<'a> {
	pub fn new(serializer: &'a mut ByondSerializer, variant: &'static str, len: usize) -> Self {
		Self {
			serializer,
			variant,
			items: Vec::with_capacity(len),
		}
	}
}

impl SerializeTupleVariant for ByondVariantSerializer<'_> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize + ?Sized,
	{
		self.items.push(value.serialize(&mut *self.serializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		let mut sequence = ByondValue::new_list()?;
		sequence.write_list(self.items)?;
		let mut list = ByondValue::new_list()?;
		list.write_list_index(self.variant, &sequence)?;
		Ok(list)
	}
}
