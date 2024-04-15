// SPDX-License-Identifier: 0BSD
use crate::sys::CByondXYZ;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct ByondXYZ(pub CByondXYZ);

impl ByondXYZ {
	#[inline]
	pub fn new(x: i16, y: i16, z: i16) -> Self {
		Self(CByondXYZ { x, y, z, junk: 0 })
	}

	#[inline(always)]
	pub fn x(&self) -> i16 {
		self.0.x
	}

	#[inline(always)]
	pub fn y(&self) -> i16 {
		self.0.y
	}

	#[inline(always)]
	pub fn z(&self) -> i16 {
		self.0.z
	}

	pub fn block_size(&self, other: &ByondXYZ) -> (u16, u16) {
		let dx = (self.x() as i32 - other.x() as i32).unsigned_abs() as u16 + 1;
		let dy = (self.y() as i32 - other.y() as i32).unsigned_abs() as u16 + 1;
		(dx, dy)
	}

	pub fn total_block_size(&self, other: &ByondXYZ) -> u16 {
		let (w, h) = self.block_size(other);
		w.saturating_mul(h)
	}

	pub fn distance(&self, other: &ByondXYZ) -> f64 {
		let dx = (self.x() - other.x()) as f64;
		let dy = (self.y() - other.y()) as f64;
		let dz = (self.z() - other.z()) as f64;

		(dx * dx + dy * dy + dz * dz).sqrt()
	}
}

impl PartialEq for ByondXYZ {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.0.x == other.0.x && self.0.y == other.0.y && self.0.z == other.0.z
	}
}

impl Eq for ByondXYZ {}

impl std::hash::Hash for ByondXYZ {
	#[inline]
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.0.x.hash(state);
		self.0.y.hash(state);
		self.0.z.hash(state);
	}
}

impl From<ByondXYZ> for CByondXYZ {
	#[inline(always)]
	fn from(value: ByondXYZ) -> Self {
		value.0
	}
}

impl From<CByondXYZ> for ByondXYZ {
	#[inline(always)]
	fn from(value: CByondXYZ) -> Self {
		Self(value)
	}
}

impl From<ByondXYZ> for (i16, i16, i16) {
	#[inline(always)]
	fn from(value: ByondXYZ) -> Self {
		(value.x(), value.y(), value.z())
	}
}

impl From<(i16, i16, i16)> for ByondXYZ {
	#[inline(always)]
	fn from((x, y, z): (i16, i16, i16)) -> Self {
		Self::new(x, y, z)
	}
}

impl From<(u16, u16, u16)> for ByondXYZ {
	#[inline(always)]
	fn from((x, y, z): (u16, u16, u16)) -> Self {
		Self::new(x as i16, y as i16, z as i16)
	}
}

impl Add for ByondXYZ {
	type Output = ByondXYZ;

	fn add(self, other: ByondXYZ) -> ByondXYZ {
		ByondXYZ::new(
			self.x() + other.x(),
			self.y() + other.y(),
			self.z() + other.z(),
		)
	}
}

impl AddAssign for ByondXYZ {
	fn add_assign(&mut self, other: ByondXYZ) {
		self.0.x += other.0.x;
		self.0.y += other.0.y;
		self.0.z += other.0.z;
	}
}

impl Sub for ByondXYZ {
	type Output = ByondXYZ;

	fn sub(self, other: ByondXYZ) -> ByondXYZ {
		ByondXYZ::new(
			self.x() - other.x(),
			self.y() - other.y(),
			self.z() - other.z(),
		)
	}
}

impl SubAssign for ByondXYZ {
	fn sub_assign(&mut self, other: ByondXYZ) {
		self.0.x -= other.0.x;
		self.0.y -= other.0.y;
		self.0.z -= other.0.z;
	}
}
