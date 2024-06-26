if_alloc! {
    use crate::alloc::{string::String, fmt::format};
}

use bitfield_struct::bitfield;

use crate::{
    conversion::Conversion,
    prelude::{Id, IsProtocol},
};

/// Bitfield representation of an extended 29-bit CAN identifier.
///
/// ### Repr: `u32`
///
/// | Field                  | Size (bits) |
/// |------------------------|-------------|
/// | Padding bits (private) | 3           |
/// | Identifier bits        | 29          |
#[bitfield(u32, order = Msb)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Can2B {
    #[bits(3)]
    _padding_bits: u8,
    #[bits(29)]
    id_bits: u32,
}

impl IsProtocol for Can2B {}

impl Conversion<u32> for Id<Can2B> {
    type Error = anyhow::Error;

    /// Creates a new 29-bit extended identifier from a 16-bit integer.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, Can2B, Conversion};
    /// let id_a = Id::<Can2B>::from_bits(16711935);
    ///
    /// assert_eq!(0b00000_11111111_00000000_11111111, id_a.into_bits());
    /// ```
    fn from_bits(bits: u32) -> Self {
        let id_bitfield = Can2B(bits);

        Self(id_bitfield)
    }

    /// Creates a new 29-bit extended identifier from a base-16 (hex) string slice.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, Can2B, Conversion};
    /// let id_a = Id::<Can2B>::from_hex("00FF00FF");
    ///
    /// assert_eq!(0b00000_11111111_00000000_11111111, id_a.into_bits());
    /// ```
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
    /// # use can_types::prelude::{Id, Can2B, Conversion};
    /// let id_a = Id::<Can2B>::try_from_bits(16711935).unwrap();
    /// let id_b = Id::<Can2B>::try_from_bits(536870912);
    ///
    /// assert_eq!(0b00000_11111111_00000000_11111111, id_a.into_bits());
    /// assert!(id_b.is_err());
    /// ```
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
    /// # use can_types::prelude::{Id, Can2B, Conversion};
    /// let id_a = Id::<Can2B>::try_from_hex("00FF00FF").unwrap();
    /// let id_b = Id::<Can2B>::try_from_hex("20000000");
    ///
    /// assert_eq!(0b00000_11111111_00000000_11111111, id_a.into_bits());
    /// assert!(id_b.is_err());
    /// ```
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
    /// # use can_types::prelude::{Id, Can2B, Conversion};
    /// let id_a = Id::<Can2B>::from_bits(16711935);
    ///
    /// assert_eq!(16711935, id_a.into_bits());
    /// ```
    fn into_bits(self) -> u32 {
        self.0.into_bits()
    }

    /// Creates a new base-16 (hex) `String` from the 29-bit extended identifier.
    ///
    /// # Requires
    /// - `alloc`
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, Can2B, Conversion};
    /// let id_a = Id::<Can2B>::from_bits(16711935);
    ///
    /// assert_eq!("00FF00FF", id_a.into_hex());
    /// ```
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:08X}", self.0.into_bits()))
    }
}

#[cfg(test)]
mod can2b_tests {
    use crate::prelude::{Conversion, Id};

    use super::Can2B;

    #[test]
    fn test_from_bits() {
        let id_a = Id::<Can2B>::from_bits(16711935);

        assert_eq!(0b00000_11111111_00000000_11111111, id_a.0 .0)
    }

    #[test]
    fn test_from_hex() {
        let id_a = Id::<Can2B>::from_hex("00FF00FF");

        assert_eq!(0b00000_11111111_00000000_11111111, id_a.0 .0)
    }

    #[test]
    fn test_try_from_bits() {
        let id_a = Id::<Can2B>::try_from_bits(16711935).unwrap();
        let id_b = Id::<Can2B>::try_from_bits(536870912);

        assert_eq!(0b00000_11111111_00000000_11111111, id_a.0 .0);
        assert!(id_b.is_err())
    }

    #[test]
    fn test_try_from_hex() {
        let id_a = Id::<Can2B>::try_from_hex("00FF00FF").unwrap();
        let id_b = Id::<Can2B>::try_from_hex("20000000");

        assert_eq!(0b00000_11111111_00000000_11111111, id_a.0 .0);
        assert!(id_b.is_err())
    }

    #[test]
    fn test_into_bits() {
        let id_a = Id::<Can2B>::from_bits(16711935);

        assert_eq!(16711935, id_a.into_bits())
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_into_hex() {
        let id_a = Id::<Can2B>::from_bits(16711935);

        assert_eq!("00FF00FF", id_a.into_hex())
    }
}
