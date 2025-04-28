// SPDX-License-Identifier: 0BSD
use crate::sys::{s2c, CByondPixLoc};
use std::hash::Hash;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ByondPixLoc(pub CByondPixLoc);

impl ByondPixLoc {
	/// A [ByondPixLoc] with x/y/z coordinates as 0.
	pub const ZERO: Self = Self::new(0.0, 0.0, 0);

	/// Initializes a new [ByondPixLoc] with the given X/Y/Z coordinates.
	#[inline]
	pub const fn new(x: f32, y: f32, z: s2c) -> Self {
		Self(CByondPixLoc { x, y, z, junk: 0 })
	}

	/// Returns the pixel X coordinate.
	#[inline]
	pub const fn x(&self) -> f32 {
		self.0.x
	}

	/// Returns the pixel Y coordinate.
	#[inline]
	pub const fn y(&self) -> f32 {
		self.0.y
	}

	/// Returns the Z coordinate.
	#[inline]
	pub const fn z(&self) -> i16 {
		self.0.z
	}
}

impl Default for ByondPixLoc {
	fn default() -> Self {
		Self::ZERO
	}
}

// the automatic PartialEq deriver would also compare the junk data, and we
// don't want that.
impl PartialEq for ByondPixLoc {
	fn eq(&self, other: &Self) -> bool {
		self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
	}
}

impl Hash for ByondPixLoc {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.x().to_bits().hash(state);
		self.y().to_bits().hash(state);
		self.z().hash(state);
	}
}

impl AsRef<CByondPixLoc> for ByondPixLoc {
	fn as_ref(&self) -> &CByondPixLoc {
		&self.0
	}
}

impl AsMut<CByondPixLoc> for ByondPixLoc {
	fn as_mut(&mut self) -> &mut CByondPixLoc {
		&mut self.0
	}
}
