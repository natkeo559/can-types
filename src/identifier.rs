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

if_alloc! {
    use crate::alloc::{string::String, fmt::format};
}

use bitfield_struct::bitfield;

use crate::conversion::Conversion;

pub trait IdKind {}

impl IdKind for Standard {}
impl IdKind for Extended {}

/// Bitfield representation of a standard 11-bit CAN identifier.
///
/// ### Repr: `u16`
///
/// | Field                  | Size (bits) |
/// |------------------------|-------------|
/// | Padding bits (private) | 5           |
/// | Priority bits          | 3           |
/// | Reserved bits          | 1           |
/// | Data page bits         | 1           |
/// | PDU format bits        | 6           |
#[bitfield(u16, order = Msb)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Standard {
    #[bits(5)]
    _padding_bits: u8,
    #[bits(3)]
    priority_bits: u8,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(1)]
    data_page_bits: bool,
    #[bits(6)]
    pdu_format_bits: u8,
}

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
pub struct Extended {
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

/// Represents a Controller Area Network (CAN) identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id<T: IdKind>(T);

/// Represents a Controller Area Network (CAN) extended 29-bit identifier.
pub type IdExtended = Id<Extended>;

/// Represents a Controller Area Network (CAN) standard 11-bit identifier.
pub type IdStandard = Id<Standard>;

impl Conversion<u32> for IdExtended {
    type Error = anyhow::Error;

    /// Creates a new [`Extended`] bitfield from a 32-bit integer.
    fn from_bits(bits: u32) -> Self {
        let bitfield = Extended(bits);

        Self(bitfield)
    }

    /// Creates a new [`Extended`] bitfield from a base-16 (hex) string slice.
    fn from_hex(hex_str: &str) -> Self {
        let bits = u32::from_str_radix(hex_str, 16).unwrap_or_default();
        let bitfield = Extended(bits);

        Self(bitfield)
    }

    /// Creates a new [`Extended`] bitfield from a 32-bit integer.
    /// # Errors
    /// - If value out of range for valid 29-bit identifiers
    fn try_from_bits(bits: u32) -> Result<Self, Self::Error> {
        if bits > 0x1FFF_FFFF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0..536870911 - got {}",
                bits
            ));
        }
        let bitfield = Extended(bits);

        Ok(Self(bitfield))
    }

    /// Creates a new [`Extended`] bitfield from a base-16 (hex) string slice.
    /// # Errors
    /// - If failed to parse input hexadecimal string slice.
    /// - If value out of range for valid 29-bit identifiers
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let bits = u32::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x1FFF_FFFF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0x00000000..0x1FFFFFFF - got {:#08X}",
                bits
            ));
        }
        let bitfield = Extended(bits);

        Ok(Self(bitfield))
    }

    /// Creates a new 32-bit integer from the [`Extended`] bitfield.
    fn into_bits(self) -> u32 {
        self.0.into_bits()
    }

    /// Creates a new base-16 (hex) `String` from the [`Extended`] bitfield.
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:08X}", self.0.into_bits()))
    }
}

impl Conversion<u16> for IdStandard {
    type Error = anyhow::Error;

    /// Creates a new [`Standard`] bitfield from a 16-bit integer.
    fn from_bits(bits: u16) -> Self {
        let bitfield = Standard(bits);

        Self(bitfield)
    }

    /// Creates a new [`Standard`] bitfield from a base-16 (hex) string slice.
    fn from_hex(hex_str: &str) -> Self {
        let bits = u16::from_str_radix(hex_str, 16).unwrap_or_default();
        let bitfield = Standard(bits);

        Self(bitfield)
    }

    /// Creates a new [`Standard`] bitfield from a 16-bit integer.
    /// # Errors
    /// - If value out of range for valid 11-bit identifiers
    fn try_from_bits(bits: u16) -> Result<Self, Self::Error> {
        if bits > 0x7FF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0..2047 - got {}",
                bits
            ));
        }
        let bitfield = Standard(bits);

        Ok(Self(bitfield))
    }

    /// Creates a new [`Standard`] bitfield from a base-16 (hex) string slice.
    /// # Errors
    /// - If failed to parse input hexadecimal string slice.
    /// - If value out of range for valid 11-bit identifiers
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let bits = u16::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x7FF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0x000..0x7FF - got {:#03X}",
                bits
            ));
        }
        let bitfield = Standard(bits);

        Ok(Self(bitfield))
    }

    /// Creates a new 16-bit integer from the [`Standard`] bitfield.
    fn into_bits(self) -> u16 {
        self.0.into_bits()
    }

    /// Creates a new base-16 (hex) `String` from the [`Standard`] bitfield.
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:03X}", self.0.into_bits()))
    }
}

impl IdExtended {
    /// Decomposes the [`IdExtended`] into its raw parts.
    ///
    /// Returns a tuple containing the priority, reserved flag, data page flag,
    /// PDU format, PDU specific, and source address bits.
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

    /// Constructs an [`IdExtended`] from its raw parts.
    ///
    /// # Arguments
    /// - `priority`: Priority bits as `u8`.
    /// - `reserved`: Reserved flag as `bool`.
    /// - `data_page`: Data page flag as `bool`.
    /// - `pdu_format`: PDU format bits as `u8`.
    /// - `pdu_specific`: PDU specific bits as `u8`.
    /// - `source_addr`: Source address bits as `u8`.
    ///
    /// # Errors
    /// - If priority value is invalid
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

        let bitfield = Extended::new()
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
    pub const fn source_address(&self) -> u8 {
        self.0.source_address_bits()
    }
}

impl IdStandard {
    /// Decomposes the [`IdStandard`] into its raw parts.
    ///
    /// Returns a tuple containing the priority, reserved flag, data page flag,
    /// and PDU format bits.
    #[must_use]
    pub const fn into_raw_parts(self) -> (u8, bool, bool, u8) {
        let p = self.0.priority_bits();
        let r = self.0.reserved_bits();
        let dp = self.0.data_page_bits();
        let pf = self.0.pdu_format_bits();

        (p, r, dp, pf)
    }

    /// Constructs an [`IdStandard`] from its raw parts.
    ///
    /// # Arguments
    /// - `priority`: Priority bits as `u8`.
    /// - `reserved`: Reserved flag as `bool`.
    /// - `data_page`: Data page flag as `bool`.
    /// - `pdu_format`: PDU format bits as `u8`.
    ///
    /// # Errors
    /// - If priority value is invalid
    /// - If PDU format is invalid
    pub fn from_raw_parts(
        priority: u8,
        reserved: bool,
        data_page: bool,
        pdu_format: u8,
    ) -> Result<Self, anyhow::Error> {
        if priority > 0x7 {
            return Err(anyhow::anyhow!(
                "Invalid priority! The priority value must be between 0 and 7 inclusive - got {}.",
                priority
            ));
        }

        if pdu_format > 0x3F {
            return Err(anyhow::anyhow!(
                "Invalid pdu format! The pdu format must be between 0 and 63 inclusive - got {}.",
                data_page
            ));
        }

        let bitfield = Standard::new()
            .with_priority_bits(priority)
            .with_reserved_bits(reserved)
            .with_data_page_bits(data_page)
            .with_pdu_format_bits(pdu_format);

        Ok(Self(bitfield))
    }

    /// Returns the priority bits indicating the priority level.
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
}

#[cfg(test)]
mod id_tests {
    use super::*;

    #[test]
    fn test_new_standard_id_bitfield() {
        let id_a = Standard::new()
            .with_priority_bits(0)
            .with_reserved_bits(false)
            .with_data_page_bits(false)
            .with_pdu_format_bits(0);

        let id_b = Standard::new()
            .with_priority_bits(3)
            .with_reserved_bits(true)
            .with_data_page_bits(true)
            .with_pdu_format_bits(31);

        assert_eq!(0b00000_000_0_0_000000, id_a.into_bits());
        assert_eq!(0b00000_011_1_1_011111, id_b.into_bits());
    }

    #[test]
    fn test_new_extended_id_bitfield() {
        let id_a = Extended::new()
            .with_priority_bits(0)
            .with_reserved_bits(false)
            .with_data_page_bits(false)
            .with_pdu_format_bits(0)
            .with_pdu_specific_bits(0)
            .with_source_address_bits(0);

        let id_b = Extended::new()
            .with_priority_bits(3)
            .with_reserved_bits(true)
            .with_data_page_bits(true)
            .with_pdu_format_bits(9)
            .with_pdu_specific_bits(0)
            .with_source_address_bits(255);

        assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.into_bits());
        assert_eq!(0b000_011_1_1_00001001_00000000_11111111, id_b.into_bits());
    }

    #[test]
    fn test_extended_try_from_bits() -> Result<(), anyhow::Error> {
        let id_a = Id::<Extended>::try_from_bits(0)?;
        let id_b = Id::<Extended>::try_from_bits(4294967295);

        assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.into_bits());
        assert!(id_b.is_err());

        Ok(())
    }

    #[test]
    fn test_extended_from_bits() -> Result<(), anyhow::Error> {
        let id_a = Id::<Extended>::from_bits(0);
        let id_b = Id::<Extended>::from_bits(4294967295);

        assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.into_bits());
        assert_eq!(0b111_111_1_1_11111111_11111111_11111111, id_b.into_bits());

        Ok(())
    }

    #[test]
    fn test_standard_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "00F";

        let id_a = IdStandard::try_from_hex(hex_str)?;

        assert_eq!(0b00000_000_0_0_001111, id_a.into_bits());
        assert_eq!(0, id_a.priority());
        assert_eq!(false, id_a.reserved());
        assert_eq!(false, id_a.data_page());
        assert_eq!(15, id_a.pdu_format());

        Ok(())
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_standard_to_hex() -> Result<(), anyhow::Error> {
        let id_dec = 15;
        let id_hex = "00F";
        let id_a = IdStandard::try_from_bits(id_dec)?;

        assert_eq!(id_hex, id_a.into_hex());

        Ok(())
    }

    #[test]
    fn test_extended_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "0CF00400";

        let id_a = IdExtended::try_from_hex(hex_str)?;

        assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id_a.into_bits());
        assert_eq!(3, id_a.priority());
        assert_eq!(false, id_a.reserved());
        assert_eq!(false, id_a.data_page());
        assert_eq!(240, id_a.pdu_format());

        Ok(())
    }

    #[test]
    fn test_standard_from_bits() -> Result<(), anyhow::Error> {
        let bits = 15;

        let id_a = IdStandard::try_from_bits(bits)?;

        assert_eq!(0b00000_000_0_0_001111, id_a.into_bits());
        assert_eq!(0, id_a.priority());
        assert_eq!(false, id_a.reserved());
        assert_eq!(false, id_a.data_page());
        assert_eq!(15, id_a.pdu_format());

        Ok(())
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_extended_to_hex() -> Result<(), anyhow::Error> {
        let id_dec = 217056256;
        let id_hex = "0CF00400";
        let id_a = IdExtended::try_from_bits(id_dec)?;

        assert_eq!(id_hex, id_a.into_hex());

        Ok(())
    }

    #[test]
    fn test_id2() -> Result<(), anyhow::Error> {
        Ok(())
    }
}
