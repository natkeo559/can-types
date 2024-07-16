if_alloc! {
    use crate::alloc::{string::String, fmt::format};
}

use bitfield_struct::bitfield;

use crate::{
    conversion::Conversion,
    identifier::{Id, IsProtocol},
};

/// Bitfield representation of a standard 11-bit CAN identifier.
///
/// ### Repr: `u16`
///
/// | Field                  | Size (bits) |
/// |------------------------|-------------|
/// | Padding bits (private) | 5           |
/// | Identifier bits        | 11          |
#[bitfield(u16, order = Msb, conversion = false)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Can2A {
    #[bits(5)]
    __: u8,
    #[bits(11)]
    id_bits: u16,
}

impl IsProtocol for Can2A {}

pub type IdCan2A = Id<Can2A>;

impl IdCan2A {
    /// Returns the value of the identifier, which is truncated to 11-bits.
    #[inline]
    #[must_use]
    pub const fn id(self) -> u16 {
        self.0.id_bits()
    }
}

impl Conversion<u16> for IdCan2A {
    type Error = anyhow::Error;

    /// Creates a new 11-bit standard identifier from a 16-bit integer.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2A::from_bits(15);
    ///
    /// assert_eq!(0b000_0000_1111, id_a.into_bits());
    /// ```
    #[inline]
    fn from_bits(bits: u16) -> Self {
        let id_bitfield = Can2A(bits);

        Self(id_bitfield)
    }

    /// Creates a new 11-bit standard identifier from a base-16 (hex) string slice.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2A::from_hex("00F");
    ///
    /// assert_eq!(0b000_0000_1111, id_a.into_bits());
    /// ```
    #[inline]
    fn from_hex(hex_str: &str) -> Self {
        let bits = u16::from_str_radix(hex_str, 16).unwrap_or_default();
        let id_bitfield = Can2A(bits);

        Self(id_bitfield)
    }

    /// Creates a new 11-bit standard identifier from a 16-bit integer.
    /// # Errors
    /// - If value out of range for valid 11-bit identifiers
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2A::try_from_bits(15).unwrap();
    /// let id_b = IdCan2A::try_from_bits(2048);
    ///
    /// assert_eq!(0b000_0000_1111, id_a.into_bits());
    /// assert!(id_b.is_err());
    /// ```
    #[inline]
    fn try_from_bits(bits: u16) -> Result<Self, Self::Error> {
        if bits > 0x7FF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0..2047 - got {}",
                bits
            ));
        }
        let id_bitfield = Can2A(bits);

        Ok(Self(id_bitfield))
    }

    /// Creates a new 11-bit standard identifier from a base-16 (hex) string slice.
    /// # Errors
    /// - If failed to parse input hexadecimal string slice.
    /// - If value out of range for valid 11-bit identifiers
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2A::try_from_hex("00F").unwrap();
    /// let id_b = IdCan2A::try_from_hex("FFF");
    ///
    /// assert_eq!(0b000_0000_1111, id_a.into_bits());
    /// assert!(id_b.is_err());
    /// ```
    #[inline]
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let bits = u16::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x7FF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0x000..0x7FF - got {:#03X}",
                bits
            ));
        }
        let id_bitfield = Can2A(bits);

        Ok(Self(id_bitfield))
    }

    /// Creates a new 16-bit integer from the 11-bit standard identifier.
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2A::from_bits(15);
    ///
    /// assert_eq!(15, id_a.into_bits());
    /// assert_eq!(0b000_0000_1111, id_a.into_bits());
    /// assert_eq!(0x00F, id_a.into_bits());
    /// ```
    #[inline]
    fn into_bits(self) -> u16 {
        self.0 .0
    }

    /// Creates a new base-16 (hex) `String` from the 11-bit standard identifier.
    ///
    /// # Requires
    /// - `alloc`
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::*;
    /// let id_a = IdCan2A::from_bits(15);
    ///
    /// assert_eq!("00F", id_a.into_hex());
    /// ```
    #[inline]
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:03X}", self.0 .0))
    }
}

#[cfg(test)]
mod can2a_tests {

    use super::*;

    #[test]
    fn test_identifier_value() {
        let id_a = IdCan2A::from_bits(u16::MAX);

        assert_eq!(0x7FF, id_a.id())
    }

    #[test]
    fn test_from_bits() {
        let id_a = IdCan2A::from_bits(15);

        assert_eq!(0b000_0000_1111, id_a.0 .0)
    }

    #[test]
    fn test_from_hex() {
        let id_a = IdCan2A::from_hex("00F");

        assert_eq!(0b000_0000_1111, id_a.0 .0)
    }

    #[test]
    fn test_try_from_bits() {
        let id_a = IdCan2A::try_from_bits(15).unwrap();
        let id_b = IdCan2A::try_from_bits(2048);

        assert_eq!(0b000_0000_1111, id_a.0 .0);
        assert!(id_b.is_err())
    }

    #[test]
    fn test_try_from_hex() {
        let id_a = IdCan2A::try_from_hex("00F").unwrap();
        let id_b = IdCan2A::try_from_hex("FFF");

        assert_eq!(0b000_0000_1111, id_a.0 .0);
        assert!(id_b.is_err())
    }

    #[test]
    fn test_into_bits() {
        let id_a = IdCan2A::from_bits(15);

        assert_eq!(15, id_a.into_bits())
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_into_hex() {
        let id_a = IdCan2A::from_bits(15);

        assert_eq!("00F", id_a.into_hex())
    }
}
