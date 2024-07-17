// Copyright (C) 2024  Nathan H. Keough
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! The extended CAN2.0 specification with a 29-bit identifier.

if_alloc! {
    use crate::alloc::{string::String, fmt::format};
}

use bitfield_struct::bitfield;

use crate::{
    conversion::Conversion,
    identifier::{Id, IsProtocol},
};

/// Bitfield representation of an extended 29-bit CAN identifier.
///
/// ### Repr: `u32`
///
/// | Field      | Size (bits) |
/// |------------|-------------|
/// | Padding    | 3           |
/// | Identifier | 29          |
#[bitfield(u32, order = Msb, conversion = false)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Can2B {
    #[bits(3)]
    __: u8,
    #[bits(29)]
    id_bits: u32,
}

impl IsProtocol for Can2B {}

pub type IdCan2B = Id<Can2B>;

impl IdCan2B {
    /// Returns the value of the identifier, which is truncated to 29-bits.
    #[inline]
    #[must_use]
    pub const fn id(self) -> u32 {
        self.0.id_bits()
    }
}

impl Conversion<u32> for IdCan2B {
    type Error = anyhow::Error;

    /// Creates a new 29-bit extended identifier from a 16-bit integer.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2B::from_bits(16711935);
    ///
    /// assert_eq!(0b00000_11111111_00000000_11111111, id_a.into_bits());
    /// ```
    #[inline]
    fn from_bits(bits: u32) -> Self {
        let id_bitfield = Can2B(bits);

        Self(id_bitfield)
    }

    /// Creates a new 29-bit extended identifier from a base-16 (hex) string slice.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2B::from_hex("00FF00FF");
    ///
    /// assert_eq!(0b00000_11111111_00000000_11111111, id_a.into_bits());
    /// ```
    #[inline]
    fn from_hex(hex_str: &str) -> Self {
        let bits = u32::from_str_radix(hex_str, 16).unwrap_or_default();
        let id_bitfield = Can2B(bits);

        Self(id_bitfield)
    }

    /// Creates a new 29-bit extended identifier from a 16-bit integer.
    ///
    /// # Errors
    /// - If value out of range for valid 11-bit identifiers
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2B::try_from_bits(16711935).unwrap();
    /// let id_b = IdCan2B::try_from_bits(536870912);
    ///
    /// assert_eq!(0b00000_11111111_00000000_11111111, id_a.into_bits());
    /// assert!(id_b.is_err());
    /// ```
    #[inline]
    fn try_from_bits(bits: u32) -> Result<Self, Self::Error> {
        if bits > 0x1FFF_FFFF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0..536870911 - got {}",
                bits
            ));
        }
        let id_bitfield = Can2B(bits);

        Ok(Self(id_bitfield))
    }

    /// Creates a new 29-bit extended identifier from a base-16 (hex) string slice.
    ///
    /// # Errors
    /// - If failed to parse input hexadecimal string slice.
    /// - If value out of range for valid 11-bit identifiers
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2B::try_from_hex("00FF00FF").unwrap();
    /// let id_b = IdCan2B::try_from_hex("20000000");
    ///
    /// assert_eq!(0b00000_11111111_00000000_11111111, id_a.into_bits());
    /// assert!(id_b.is_err());
    /// ```
    #[inline]
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let bits = u32::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x1FFF_FFFF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0x0000_0000..0x1FFF_FFFF - got {:#08X}",
                bits
            ));
        }
        let id_bitfield = Can2B(bits);

        Ok(Self(id_bitfield))
    }

    /// Creates a new 16-bit integer from the 29-bit extended identifier.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2B::from_bits(16711935);
    ///
    /// assert_eq!(16711935, id_a.into_bits());
    /// ```
    #[inline]
    fn into_bits(self) -> u32 {
        self.0 .0
    }

    /// Creates a new base-16 (hex) `String` from the 29-bit extended identifier.
    ///
    /// # Requires
    /// - `alloc`
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2B::from_bits(16711935);
    ///
    /// assert_eq!("00FF00FF", id_a.into_hex());
    /// ```
    #[inline]
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:08X}", self.0 .0))
    }
}

#[cfg(test)]
mod can2b_tests {

    use super::*;

    #[test]
    fn test_identifier_value() {
        let id_a = IdCan2B::from_bits(u32::MAX);

        assert_eq!(0x1FFFFFFF, id_a.id())
    }

    #[test]
    fn test_from_bits() {
        let id_a = IdCan2B::from_bits(16711935);

        assert_eq!(0b00000_11111111_00000000_11111111, id_a.0 .0)
    }

    #[test]
    fn test_from_hex() {
        let id_a = IdCan2B::from_hex("00FF00FF");

        assert_eq!(0b00000_11111111_00000000_11111111, id_a.0 .0)
    }

    #[test]
    fn test_try_from_bits() {
        let id_a = IdCan2B::try_from_bits(16711935).unwrap();
        let id_b = IdCan2B::try_from_bits(536870912);

        assert_eq!(0b00000_11111111_00000000_11111111, id_a.0 .0);
        assert!(id_b.is_err())
    }

    #[test]
    fn test_try_from_hex() {
        let id_a = IdCan2B::try_from_hex("00FF00FF").unwrap();
        let id_b = IdCan2B::try_from_hex("20000000");

        assert_eq!(0b00000_11111111_00000000_11111111, id_a.0 .0);
        assert!(id_b.is_err())
    }

    #[test]
    fn test_into_bits() {
        let id_a = IdCan2B::from_bits(16711935);

        assert_eq!(16711935, id_a.into_bits())
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_into_hex() {
        let id_a = IdCan2B::from_bits(16711935);

        assert_eq!("00FF00FF", id_a.into_hex())
    }
}
