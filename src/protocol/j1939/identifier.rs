if_alloc! {
    use crate::alloc::{string::String, fmt::format};
}

use bitfield_struct::bitfield;

use crate::{
    conversion::Conversion,
    prelude::{Id, IsProtocol},
};

use super::address::SourceAddr;

/// Bitfield representation of a 29-bit J1939 CAN identifier.
///
/// ### Repr: `u32`
///
/// | Field                  | Size (bits) |
/// |------------------------|-------------|
/// | Padding bits (private) | 3           |
/// | Priority bits          | 3           |
/// | Reserved bits          | 1           |
/// | Data page bits         | 1           |
/// | PDU format bits        | 8           |
/// | PDU specific bits      | 8           |
/// | Source address bits    | 8           |
#[bitfield(u32, order = Msb)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct J1939 {
    #[bits(3)]
    _padding_bits: u8,
    #[bits(3)]
    priority_bits: u8,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(1)]
    data_page_bits: bool,
    #[bits(8)]
    pdu_format_bits: u8,
    #[bits(8)]
    pdu_specific_bits: u8,
    #[bits(8)]
    source_address_bits: u8,
}

impl IsProtocol for J1939 {}

impl Conversion<u32> for Id<J1939> {
    type Error = anyhow::Error;

    /// Creates a new 29-bit J1939 identifier from a 32-bit integer.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, J1939, Conversion};
    /// let id_a = Id::<J1939>::from_bits(0);
    /// let id_b = Id::<J1939>::from_bits(4294967295);
    ///
    /// assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.into_bits());
    /// assert_eq!(0b111_111_1_1_11111111_11111111_11111111, id_b.into_bits());
    /// ```
    fn from_bits(bits: u32) -> Self {
        let bitfield = J1939(bits);

        Self(bitfield)
    }

    /// Creates a new 29-bit J1939 identifier from a base-16 (hex) string slice.
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, J1939, Conversion};
    /// let hex_str = "0CF00400";
    ///
    /// let id_a = Id::<J1939>::from_hex(hex_str);
    ///
    /// assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id_a.into_bits());
    /// assert_eq!(217056256, id_a.into_bits());
    /// ```
    fn from_hex(hex_str: &str) -> Self {
        let bits = u32::from_str_radix(hex_str, 16).unwrap_or_default();
        let bitfield = J1939(bits);

        Self(bitfield)
    }

    /// Creates a new 29-bit J1939 identifier from a 32-bit integer.
    ///
    /// # Errors
    /// - If value out of range for valid 29-bit identifiers
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, J1939, Conversion};
    /// let id_a = Id::<J1939>::try_from_bits(0);
    /// let id_b = Id::<J1939>::try_from_bits(4294967295);
    ///
    /// assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.unwrap().into_bits());
    /// assert!(id_b.is_err());
    /// ```
    fn try_from_bits(bits: u32) -> Result<Self, Self::Error> {
        if bits > 0x1FFF_FFFF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0..536870911 - got {}",
                bits
            ));
        }
        let bitfield = J1939(bits);

        Ok(Self(bitfield))
    }

    /// Creates a new 29-bit J1939 identifier from a base-16 (hex) string slice.
    ///
    /// # Errors
    /// - If failed to parse input hexadecimal string slice.
    /// - If value out of range for valid 29-bit identifiers
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, J1939, Conversion};
    /// let id_a = Id::<J1939>::try_from_hex("00FF00FF").unwrap();
    /// let id_b = Id::<J1939>::try_from_hex("20000000");
    ///
    /// assert_eq!(0b000_0_0_11111111_00000000_11111111, id_a.into_bits());
    /// assert!(id_b.is_err())
    /// ```
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let bits = u32::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x1FFF_FFFF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0x00000000..0x1FFFFFFF - got {:#08X}",
                bits
            ));
        }
        let bitfield = J1939(bits);

        Ok(Self(bitfield))
    }

    /// Creates a new 32-bit integer from the 29-bit J1939 identifier.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, J1939, Conversion};
    /// let id_a = Id::<J1939>::from_bits(0);
    ///
    /// assert_eq!(0, id_a.into_bits());
    /// ```
    fn into_bits(self) -> u32 {
        self.0.into_bits()
    }

    /// Creates a new base-16 (hex) `String` from the 29-bit J1939 identifier.
    ///
    /// # Requires
    /// - `alloc`
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, J1939, Conversion};
    /// let id_a = Id::<J1939>::from_bits(15);
    ///
    /// assert_eq!("0000000F", id_a.into_hex());
    /// ```
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:08X}", self.0.into_bits()))
    }
}

impl Id<J1939> {
    /// Decomposes the 29-bit J1939 identifier into its raw parts.
    ///
    /// Returns a tuple containing the priority, reserved flag, data page flag,
    /// PDU format, PDU specific, and source address bits.
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, J1939, Conversion};
    /// let id_a = Id::<J1939>::from_hex("00FF00FF");
    ///
    /// let (p, r, dp, pf, ps, sa) = id_a.into_raw_parts();
    /// ```
    #[must_use]
    pub const fn into_raw_parts(self) -> (u8, bool, bool, u8, u8, u8) {
        let p = self.0.priority_bits();
        let r = self.0.reserved_bits();
        let dp = self.0.data_page_bits();
        let pf = self.0.pdu_format_bits();
        let ps = self.0.pdu_specific_bits();
        let sa = self.0.source_address_bits();

        (p, r, dp, pf, ps, sa)
    }

    /// Constructs a 29-bit J1939 identifier from its raw parts.
    ///
    /// # Arguments
    /// - `priority`: `u8`.
    /// - `reserved`: `bool`.
    /// - `data_page`: `bool`.
    /// - `pdu_format`: `u8`.
    /// - `pdu_specific`: `u8`.
    /// - `source_addr`: `u8`.
    ///
    /// # Errors
    /// - If priority value is invalid
    ///
    /// # Examples
    /// ```rust
    /// # use can_types::prelude::{Id, J1939, Conversion};
    /// let expected_id = Id::<J1939>::from_hex("00FF00FF");
    ///
    /// let id_a = Id::<J1939>::from_raw_parts(0x0, false, false, 0xFF, 0x00, 0xFF);
    ///
    /// assert_eq!(expected_id, id_a.unwrap());
    /// ```
    pub fn from_raw_parts(
        priority: u8,
        reserved: bool,
        data_page: bool,
        pdu_format: u8,
        pdu_specific: u8,
        source_addr: u8,
    ) -> Result<Self, anyhow::Error> {
        if priority > 0x7 {
            return Err(anyhow::anyhow!(
                "Invalid priority! The priority value must be between 0 and 7 inclusive - got {}.",
                priority
            ));
        }

        let bitfield = J1939::new()
            .with_priority_bits(priority)
            .with_reserved_bits(reserved)
            .with_data_page_bits(data_page)
            .with_pdu_format_bits(pdu_format)
            .with_pdu_specific_bits(pdu_specific)
            .with_source_address_bits(source_addr);

        Ok(Self(bitfield))
    }

    /// Returns the priority bits indicating the priority level.
    ///
    /// 0 = highest priority
    #[must_use]
    pub const fn priority(&self) -> u8 {
        self.0.priority_bits()
    }

    /// Returns the reserved flag - 0 or 1
    #[must_use]
    pub const fn reserved(&self) -> bool {
        self.0.reserved_bits()
    }

    /// Returns the data page flag - 0 or 1
    #[must_use]
    pub const fn data_page(&self) -> bool {
        self.0.data_page_bits()
    }

    /// Returns the PDU format bits specifying the Protocol Data Unit format.
    #[must_use]
    pub const fn pdu_format(&self) -> u8 {
        self.0.pdu_format_bits()
    }

    /// Returns the PDU specific bits providing additional details about the PDU.
    #[must_use]
    pub const fn pdu_specific(&self) -> u8 {
        self.0.pdu_specific_bits()
    }

    /// Returns the source address bits identifying the source of the data.
    #[must_use]
    pub fn source_address(&self) -> SourceAddr {
        SourceAddr::Some(self.0.source_address_bits())
    }
}

#[cfg(test)]
mod j1939_tests {
    use crate::prelude::{Conversion, Id};

    use super::J1939;

    #[test]
    fn test_from_bits() {
        let id_a = Id::<J1939>::from_bits(16711935);

        assert_eq!(0b000_0_0_11111111_00000000_11111111, id_a.0 .0)
    }

    #[test]
    fn test_from_hex() {
        let id_a = Id::<J1939>::from_hex("00FF00FF");

        assert_eq!(0b000_0_0_11111111_00000000_11111111, id_a.0 .0)
    }

    #[test]
    fn test_try_from_bits() {
        let id_a = Id::<J1939>::try_from_bits(16711935).unwrap();
        let id_b = Id::<J1939>::try_from_bits(536870912);

        assert_eq!(0b000_0_0_11111111_00000000_11111111, id_a.0 .0);
        assert!(id_b.is_err())
    }

    #[test]
    fn test_try_from_hex() {
        let id_a = Id::<J1939>::try_from_hex("00FF00FF").unwrap();
        let id_b = Id::<J1939>::try_from_hex("20000000");

        assert_eq!(0b000_0_0_11111111_00000000_11111111, id_a.0 .0);
        assert!(id_b.is_err())
    }

    #[test]
    fn test_into_bits() {
        let id_a = Id::<J1939>::from_bits(16711935);

        assert_eq!(16711935, id_a.into_bits())
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_into_hex() {
        let id_a = Id::<J1939>::from_bits(16711935);

        assert_eq!("00FF00FF", id_a.into_hex())
    }
}
