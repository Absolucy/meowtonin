// SPDX-License-Identifier: 0BSD
use crate::{ByondValue, byond, sys::ByondValueType as InternalByondValueType};
use constcat::concat_slices;
use std::{
	borrow::Cow,
	fmt::{self, Display},
	ops::Deref,
};

/// Simple helper macro that does a "fast" typecheck when the
/// `fast-typechecking` feature is enabled, and uses the actual byondapi
/// function to check otherwise.
macro_rules! check_byondvalue_type {
	($value:ident, $type:expr, $api:ident) => {
		if cfg!(feature = "fast-typechecking") {
			$value.get_type() == $type
		} else {
			unsafe { byond().$api(&$value.0) }
		}
	};
}

impl ByondValue {
	/// Determines if the [ByondValue] is a null value.
	///
	/// # Returns
	/// `true` if the value is null, `false` otherwise.
	pub fn is_null(&self) -> bool {
		check_byondvalue_type!(self, ByondValueType::Null, ByondValue_IsNull)
	}

	/// Checks if the [ByondValue] is a number.
	///
	/// # Returns
	/// `true` if the value is a number, `false` otherwise.
	pub fn is_number(&self) -> bool {
		check_byondvalue_type!(self, ByondValueType::Number, ByondValue_IsNum)
	}

	/// Checks if the [ByondValue] is a string.
	///
	/// # Returns
	/// `true` if the value is a string, `false` otherwise.
	pub fn is_string(&self) -> bool {
		check_byondvalue_type!(self, ByondValueType::String, ByondValue_IsStr)
	}

	/// Determines if the [ByondValue] represents a list.
	///
	/// # Returns
	/// `true` if the value is a list, `false` otherwise.
	pub fn is_list(&self) -> bool {
		// no fast typechecking here, a LOT of things are considered a list
		unsafe { byond().ByondValue_IsList(&self.0) }
	}

	/// Evaluates whether the [ByondValue] is considered "true" or not.
	///
	/// # Returns
	/// `true` if the value is logically true, `false` otherwise.
	pub fn is_true(&self) -> bool {
		unsafe { byond().ByondValue_IsTrue(&self.0) }
	}

	/// Evaluates whether the [ByondValue] is a reference (object) type or
	/// not. Does not check validity.
	///
	/// # Returns
	/// `true` if the value is a reference, `false` otherwise.
	pub fn is_ref(&self) -> bool {
		if cfg!(feature = "fast-typechecking") {
			self.get_type().is_ref_counted()
		} else {
			unsafe { byond().ByondValue_GetRef(&self.0) != 0 }
		}
	}
}

macro_rules! define_type_ids {
	($($name:ident = $id:expr)*) => {
		#[rustfmt::skip]
		#[allow(non_upper_case_globals)]
		impl ByondValueType {
			$(pub const $name: Self = Self($id);)*
		}
	}
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ByondValueType(pub InternalByondValueType);

define_type_ids! {
	Null = 0x00
	Turf = 0x01
	Obj = 0x02
	Mob = 0x03
	Area = 0x04
	Client = 0x05
	String = 0x06

	MobTypepath = 0x08
	ObjTypepath = 0x09
	TurfTypepath = 0x0A
	AreaTypepath = 0x0B
	Resource = 0x0C
	Image = 0x0D
	World = 0x0E

	// Lists
	List = 0x0F
	ArgList = 0x10
	MobContents = 0x17
	TurfContents = 0x18
	AreaContents = 0x19
	WorldContents = 0x1A
	ObjContents = 0x1C

	DatumTypepath = 0x20
	Datum = 0x21
	SaveFile = 0x23
	ProcRef = 0x26
	File = 0x27
	Number = 0x2A
	MobVars = 0x2C
	ObjVars = 0x2D
	TurfVars = 0x2E
	AreaVars = 0x2F
	ClientVars = 0x30
	Vars = 0x31
	MobOverlays = 0x32
	MobUnderlays = 0x33
	ObjOverlays = 0x34
	ObjUnderlays = 0x35
	TurfOverlays = 0x36
	TurfUnderlays = 0x37
	AreaOverlays = 0x38
	AreaUnderlays = 0x39
	Appearance = 0x3A
	Pointer = 0x3C
	ImageOverlays = 0x40
	ImageUnderlays = 0x41
	ImageVars = 0x42
	BinaryObject = 0x45
	TurfVisContents = 0x4B
	ObjVisContents = 0x4C
	MobVisContents = 0x4D
	TurfVisLocs = 0x4E
	ObjVisLocs = 0x4F
	MobVisLocs = 0x50
	WorldVars = 0x51
	GlobalVars = 0x52
	Filters = 0x53
	ImageVisContents = 0x54
	Alist = 0x55
	PixLoc = 0x56
	Vector = 0x57
	Callee = 0x58
}

impl ByondValueType {
	/// Types that refer to a list of contents
	pub const CONTENTS_TYPES: &[Self] = &[
		Self::MobContents,
		Self::TurfContents,
		Self::AreaContents,
		Self::WorldContents,
		Self::ObjContents,
	];

	/// Types that refer to a list of vars
	pub const VARS_TYPES: &[Self] = &[
		Self::MobVars,
		Self::ObjVars,
		Self::TurfVars,
		Self::AreaVars,
		Self::ClientVars,
		Self::Vars,
		Self::ImageVars,
		Self::WorldVars,
		Self::GlobalVars,
	];

	/// Types that refer to a list of static appearances
	pub const APPEARANCE_LIST_TYPES: &[Self] = &[
		Self::MobOverlays,
		Self::MobUnderlays,
		Self::ObjOverlays,
		Self::ObjUnderlays,
		Self::TurfOverlays,
		Self::TurfUnderlays,
		Self::AreaOverlays,
		Self::AreaUnderlays,
		Self::ImageOverlays,
		Self::ImageUnderlays,
	];

	/// Types that refer to a list of vis_contents
	pub const VIS_CONTENTS_TYPES: &[Self] = &[
		Self::TurfVisContents,
		Self::ObjVisContents,
		Self::MobVisContents,
		Self::ImageVisContents,
	];

	/// Types that refer to a list of vis_locs
	pub const VIS_LOCS_TYPES: &[Self] = &[Self::TurfVisLocs, Self::ObjVisLocs, Self::MobVisLocs];

	/// All types that are considered lists
	pub const ALL_LIST_TYPES: &[Self] = concat_slices!([ByondValueType]:
		ByondValueType::CONTENTS_TYPES,
		ByondValueType::VARS_TYPES,
		ByondValueType::APPEARANCE_LIST_TYPES,
		ByondValueType::VIS_CONTENTS_TYPES,
		ByondValueType::VIS_LOCS_TYPES,
		&[ByondValueType::List, ByondValueType::Alist, ByondValueType::ArgList],
	);

	/// All types that are considered datums
	pub const DATUM_TYPES: &[Self] = &[
		Self::Turf,
		Self::Obj,
		Self::Mob,
		Self::Area,
		Self::Image,
		Self::Datum,
	];

	/// All types that are tpyepaths.
	pub const PATH_TYPES: &[Self] = &[
		Self::MobTypepath,
		Self::ObjTypepath,
		Self::TurfTypepath,
		Self::AreaTypepath,
		Self::DatumTypepath,
	];

	/// Types that can be indexed by arbitrary strings
	pub const STRING_INDEXABLE_TYPES: &[Self] = concat_slices!([ByondValueType]:
		ByondValueType::VARS_TYPES,
		ByondValueType::DATUM_TYPES,
		&[
			ByondValueType::Client,
			ByondValueType::List,
			ByondValueType::Alist,
			ByondValueType::ArgList,
			ByondValueType::Appearance,
			ByondValueType::World
		],
	);

	/// Types that have procs that can be called (World is an exception because
	/// `global` has that tag, but cannot have procs defined)
	pub const PROC_HAVING_TYPES: &[Self] = concat_slices!([ByondValueType]:
		ByondValueType::ALL_LIST_TYPES,
		ByondValueType::DATUM_TYPES,
		&[
			ByondValueType::Client,
			ByondValueType::List,
			ByondValueType::Alist,
			ByondValueType::ArgList
		],
	);

	/// Types that can have procs defined by the user (World is an exception
	/// because you can define procs for `world`, but not `global`, which both
	/// have that tag)
	pub const PROC_DEFINABLE_TYPES: &[Self] =
		concat_slices!([ByondValueType]: ByondValueType::DATUM_TYPES, &[ByondValueType::Client]);

	/// Types that can be indexed in some way
	pub const INDEXABLE_TYPES: &[Self] = concat_slices!([ByondValueType]: ByondValueType::PROC_HAVING_TYPES, &[ByondValueType::Appearance, ByondValueType::World]);

	#[inline]
	pub fn can_index_at_all(&self) -> bool {
		Self::INDEXABLE_TYPES.contains(self)
	}

	#[inline]
	pub fn can_index_by_number(&self) -> bool {
		Self::ALL_LIST_TYPES.contains(self)
	}

	#[inline]
	pub fn can_index_by_string(&self) -> bool {
		Self::STRING_INDEXABLE_TYPES.contains(self)
	}

	#[inline]
	pub fn can_index_by_anything(&self) -> bool {
		matches!(*self, Self::List | Self::Alist)
	}

	/// Returns a simple string representation of the type.
	#[must_use]
	pub fn name(&self) -> Cow<'static, str> {
		match *self {
			Self::Null => Cow::Borrowed("null"),
			Self::Turf => Cow::Borrowed("turf"),
			Self::Obj => Cow::Borrowed("obj"),
			Self::Mob => Cow::Borrowed("mob"),
			Self::Area => Cow::Borrowed("area"),
			Self::Client => Cow::Borrowed("client"),
			Self::String => Cow::Borrowed("string"),
			Self::MobTypepath => Cow::Borrowed("mob typepath"),
			Self::ObjTypepath => Cow::Borrowed("obj typepath"),
			Self::TurfTypepath => Cow::Borrowed("turf typepath"),
			Self::AreaTypepath => Cow::Borrowed("area typepath"),
			Self::Resource => Cow::Borrowed("resource"),
			Self::Image => Cow::Borrowed("image"),
			Self::World => Cow::Borrowed("world"),
			Self::List => Cow::Borrowed("list"),
			Self::ArgList => Cow::Borrowed("arg list"),
			Self::MobContents => Cow::Borrowed("mob contents"),
			Self::TurfContents => Cow::Borrowed("turf contents"),
			Self::AreaContents => Cow::Borrowed("area contents"),
			Self::WorldContents => Cow::Borrowed("world contents"),
			Self::ObjContents => Cow::Borrowed("obj contents"),
			Self::DatumTypepath => Cow::Borrowed("datum typepath"),
			Self::ProcRef => Cow::Borrowed("proc reference"),
			Self::File => Cow::Borrowed("file"),
			Self::MobVars => Cow::Borrowed("mob vars"),
			Self::ObjVars => Cow::Borrowed("obj vars"),
			Self::TurfVars => Cow::Borrowed("turf vars"),
			Self::AreaVars => Cow::Borrowed("area vars"),
			Self::ClientVars => Cow::Borrowed("client vars"),
			Self::Vars => Cow::Borrowed("vars"),
			Self::MobOverlays => Cow::Borrowed("mob overlays"),
			Self::MobUnderlays => Cow::Borrowed("mob underlays"),
			Self::ObjOverlays => Cow::Borrowed("obj overlays"),
			Self::ObjUnderlays => Cow::Borrowed("obj underlays"),
			Self::TurfOverlays => Cow::Borrowed("turf overlays"),
			Self::TurfUnderlays => Cow::Borrowed("turf underlays"),
			Self::AreaOverlays => Cow::Borrowed("area overlays"),
			Self::AreaUnderlays => Cow::Borrowed("area underlays"),
			Self::ImageOverlays => Cow::Borrowed("image overlays"),
			Self::ImageUnderlays => Cow::Borrowed("image underlays"),
			Self::ImageVars => Cow::Borrowed("image vars"),
			Self::BinaryObject => Cow::Borrowed("binary object"),
			Self::TurfVisContents => Cow::Borrowed("turf vis_contents"),
			Self::ObjVisContents => Cow::Borrowed("obj vis_contents"),
			Self::MobVisContents => Cow::Borrowed("mob vis_contents"),
			Self::TurfVisLocs => Cow::Borrowed("turf vis_locs"),
			Self::ObjVisLocs => Cow::Borrowed("obj vis_locs"),
			Self::MobVisLocs => Cow::Borrowed("mob vis_locs"),
			Self::WorldVars => Cow::Borrowed("world vars"),
			Self::GlobalVars => Cow::Borrowed("global vars"),
			Self::Filters => Cow::Borrowed("filter(s)"),
			Self::ImageVisContents => Cow::Borrowed("image vis_contents"),
			Self::Datum => Cow::Borrowed("datum"),
			Self::SaveFile => Cow::Borrowed("savefile"),
			Self::Number => Cow::Borrowed("number"),
			Self::Appearance => Cow::Borrowed("appearance"),
			Self::Pointer => Cow::Borrowed("pointer"),
			Self::Alist => Cow::Borrowed("alist"),
			Self::PixLoc => Cow::Borrowed("pixloc"),
			Self::Vector => Cow::Borrowed("vector"),
			Self::Callee => Cow::Borrowed("callee"),
			other => Cow::Owned(format!("unknown ({other})")),
		}
	}

	/// Returns if this type is reference counted or not.
	///
	/// If you're checking to see if you should call [`ByondValue::inc_ref()`],
	/// [`ByondValue::dec_ref()`], or [`ByondValue::dec_temp_ref()`], use
	/// [`should_ref_count()`](Self::should_ref_count) instead.
	///
	/// # Returns
	/// `true` if the value is reference counted, `false` otherwise.
	#[inline]
	pub const fn is_ref_counted(&self) -> bool {
		!matches!(
			*self,
			Self::Null
				| Self::Number
				| Self::MobTypepath
				| Self::ObjTypepath
				| Self::TurfTypepath
				| Self::AreaTypepath
				| Self::DatumTypepath
				| Self::Turf
		)
	}

	/// Returns if this type SHOULD be reference counted.
	///
	/// The difference between this and
	/// [`is_ref_counted()`](Self::is_ref_counted) is that this also checks to
	/// see if this type SHOULDN'T be refcounted, even if it is technically a
	/// reference.
	///
	/// # Returns
	/// `true` if the value should be reference counted, `false` otherwise.
	///
	/// Currently, this only returns `false` for [`NULL`](Self::NULL),
	/// [`NUMBER`](Self::NUMBER), and [`WORLD`](Self::WORLD).
	#[inline]
	pub const fn should_ref_count(self) -> bool {
		// we have to compare the inner values for the world check to keep this const.
		// it's dumb, I know.
		self.is_ref_counted() && self.0 != Self::World.0
	}
}

impl Display for ByondValueType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}

impl AsRef<InternalByondValueType> for ByondValueType {
	fn as_ref(&self) -> &InternalByondValueType {
		&self.0
	}
}

impl Deref for ByondValueType {
	type Target = InternalByondValueType;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl PartialEq<InternalByondValueType> for ByondValueType {
	fn eq(&self, other: &InternalByondValueType) -> bool {
		self.0 == *other
	}
}

impl PartialEq<ByondValueType> for InternalByondValueType {
	fn eq(&self, other: &ByondValueType) -> bool {
		*self == other.0
	}
}

impl From<InternalByondValueType> for ByondValueType {
	fn from(value: InternalByondValueType) -> Self {
		Self(value)
	}
}

impl From<ByondValueType> for InternalByondValueType {
	fn from(value: ByondValueType) -> Self {
		value.0
	}
}
