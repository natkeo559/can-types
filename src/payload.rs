// MIT License
//
// Copyright (c) 2024 Nathan H. Keough
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Defines various PDU contents, including generic `Data` or the `Name` fields.

if_alloc! {
    use crate::alloc::{string::String, fmt::format};
}

use bitfield_struct::bitfield;

use crate::conversion::Conversion;

/// Marks a type, associating it with a protocol data unit (PDU)
pub trait IsDataUnit {}

/// Bitfield representing an 8-byte data field.
///
/// ### Repr `u64`
///
/// | Field  | Size (bits) |
/// |--------|-------------|
/// | byte 0 | 8           |
/// | byte 1 | 8           |
/// | byte 2 | 8           |
/// | byte 3 | 8           |
/// | byte 4 | 8           |
/// | byte 5 | 8           |
/// | byte 6 | 8           |
/// | byte 7 | 8           |
#[bitfield(u64, order = Msb, conversion = false)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Data {
    #[bits(8)]
    byte_0_bits: u8,
    #[bits(8)]
    byte_1_bits: u8,
    #[bits(8)]
    byte_2_bits: u8,
    #[bits(8)]
    byte_3_bits: u8,
    #[bits(8)]
    byte_4_bits: u8,
    #[bits(8)]
    byte_5_bits: u8,
    #[bits(8)]
    byte_6_bits: u8,
    #[bits(8)]
    byte_7_bits: u8,
}

/// Represents a `Name` in the SAE J1939 protocol.
///
/// The `Name` structure is used in the SAE J1939 protocol to represent the identity of a device or
/// component within a vehicle's network.
///
/// ### Repr: `u64`
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Arbitrary Address       | 1           |
/// | Industry Group          | 3           |
/// | Vehicle System Instance | 4           |
/// | Vehicle system          | 7           |
/// | Reserved                | 1           |
/// | Function                | 8           |
/// | Function Instance       | 5           |
/// | ECU Instance            | 3           |
/// | Manufacturer Code       | 11          |
/// | Identity Number         | 21          |
#[bitfield(u64, order = Msb, conversion = false)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Name {
    #[bits(1)]
    arbitrary_address_bits: bool,
    #[bits(3)]
    industry_group_bits: u8,
    #[bits(4)]
    vehicle_system_instance_bits: u8,
    #[bits(7)]
    vehicle_system_bits: u8,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(8)]
    function_bits: u8,
    #[bits(5)]
    function_instance_bits: u8,
    #[bits(3)]
    ecu_instance_bits: u8,
    #[bits(11)]
    manufacturer_code_bits: u16,
    #[bits(21)]
    identity_number_bits: u32,
}

impl IsDataUnit for Data {}
impl IsDataUnit for Name {}

/// Represents a Protocol Data Unit (PDU) in the context of Controller Area Network (CAN).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pdu<U: IsDataUnit>(pub(crate) U);

impl Conversion<u64> for Pdu<Data> {
    type Error = anyhow::Error;

    /// Creates a new [`Data`] bitfield from a 64-bit integer.
    fn from_bits(bits: u64) -> Self {
        Self(Data(bits))
    }

    /// Creates a new [`Data`] bitfield from a base-16 (hex) string slice.
    fn from_hex(hex_str: &str) -> Self {
        let bits = u64::from_str_radix(hex_str, 16).unwrap_or_default();

        Self(Data(bits))
    }

    /// Creates a new [`Data`] bitfield from a 64-bit integer.
    /// # Errors
    /// - Never (conversion is trivial)
    fn try_from_bits(bits: u64) -> Result<Self, Self::Error> {
        Ok(Self(Data(bits)))
    }

    /// Creates a new [`Data`] bitfield from a base-16 (hex) string slice.
    /// # Errors
    /// - If failed to parse input hexadecimal string slice.
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let bits = u64::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;

        Ok(Self(Data(bits)))
    }

    /// Creates a new 64-bit integer from the [`Data`] bitfield.
    fn into_bits(self) -> u64 {
        self.0 .0
    }

    /// Creates a new base-16 (hex) [`String`] from the [`Data`] bitfield.
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:016X}", self.0 .0))
    }
}

impl Pdu<Data> {
    /// Retrieve byte 0.
    #[must_use]
    pub const fn byte_0(self) -> u8 {
        self.0.byte_0_bits()
    }

    /// Retrieve byte 1.
    #[must_use]
    pub const fn byte_1(self) -> u8 {
        self.0.byte_1_bits()
    }

    /// Retrieve byte 2.
    #[must_use]
    pub const fn byte_2(self) -> u8 {
        self.0.byte_2_bits()
    }

    /// Retrieve byte 3.
    #[must_use]
    pub const fn byte_3(self) -> u8 {
        self.0.byte_3_bits()
    }

    /// Retrieve byte 4.
    #[must_use]
    pub const fn byte_4(self) -> u8 {
        self.0.byte_4_bits()
    }

    /// Retrieve byte 5.
    #[must_use]
    pub const fn byte_5(self) -> u8 {
        self.0.byte_5_bits()
    }

    /// Retrieve byte 6.
    #[must_use]
    pub const fn byte_6(self) -> u8 {
        self.0.byte_6_bits()
    }

    /// Retrieve byte 7.
    #[must_use]
    pub const fn byte_7(self) -> u8 {
        self.0.byte_7_bits()
    }

    /// Return the 64-bit [`Data`] bitfield as little-endian bytes.
    #[must_use]
    pub const fn to_le_bytes(&self) -> [u8; 8] {
        self.0 .0.to_le_bytes()
    }

    /// Return the 64-bit [`Data`] bitfield as big-endian bytes.
    #[must_use]
    pub const fn to_be_bytes(&self) -> [u8; 8] {
        self.0 .0.to_be_bytes()
    }

    /// Return the 64-bit [`Data`] bitfield as native-endian bytes.
    #[must_use]
    pub const fn to_ne_bytes(&self) -> [u8; 8] {
        self.0 .0.to_ne_bytes()
    }

    /// Convert the [`Data`] bitfield to little-endian byte format.
    #[must_use]
    pub const fn to_le(&self) -> Self {
        Self(Data(self.0 .0.to_le()))
    }

    /// Convert the [`Data`] bitfield to big-endian byte format.
    #[must_use]
    pub const fn to_be(&self) -> Self {
        Self(Data(self.0 .0.to_be()))
    }
}

impl Conversion<u64> for Pdu<Name> {
    type Error = anyhow::Error;

    /// Creates a new [`Name`] bitfield from a 64-bit integer.
    fn from_bits(bits: u64) -> Self {
        Self(Name(bits))
    }

    /// Creates a new [`Name`] bitfield from a base-16 (hex) string slice.
    fn from_hex(hex_str: &str) -> Self {
        let bits = u64::from_str_radix(hex_str, 16).unwrap_or_default();

        Self(Name(bits))
    }

    /// Creates a new [`Name`] bitfield from a 64-bit integer.
    /// # Errors
    /// - Never (conversion is trivial)
    fn try_from_bits(bits: u64) -> Result<Self, Self::Error> {
        Ok(Self(Name(bits)))
    }

    /// Creates a new [`Name`] bitfield from a base-16 (hex) string slice.
    /// # Errors
    /// - If invalid encoding of provided Base16 string
    /// - If insufficient output buffer length
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let bits = u64::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;

        Ok(Self(Name(bits)))
    }

    /// Creates a new 64-bit integer from the [`Name`] bitfield.
    fn into_bits(self) -> u64 {
        self.0 .0
    }

    /// Creates a new base-16 (hex) [`String`] from the [`Name`] bitfield.
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:016X}", self.0 .0))
    }
}

impl Pdu<Name> {
    /// Indicates whether or not the ECU/CA can negotiate an address (true = yes; false = no).
    #[must_use]
    pub const fn arbitrary_address(&self) -> bool {
        self.0.arbitrary_address_bits()
    }

    /// These codes are associated with particular industries such as on-highway equipment,
    /// agricultural equipment, and more.
    #[must_use]
    pub const fn industry_group(&self) -> u8 {
        self.0.industry_group_bits()
    }

    /// Assigns a number to each instance on the Vehicle System (in case you connect several
    /// networks – e.g. connecting cars on a train).
    #[must_use]
    pub const fn vehicle_system_instance(&self) -> u8 {
        self.0.vehicle_system_instance_bits()
    }

    /// Vehicle systems are associated with the Industry Group and they can be, for instance,
    /// “tractor” in the “Common” industry or “trailer” in the “On-Highway” industry group.
    #[must_use]
    pub const fn vehicle_system(&self) -> u8 {
        self.0.vehicle_system_bits()
    }

    /// Always zero(false).
    #[must_use]
    pub const fn reserved(&self) -> bool {
        self.0.reserved_bits()
    }

    /// This code, in a range between 128 and 255, is assigned according to the Industry Group. A
    /// value between 0 and 127 is not associated with any other parameter.
    #[must_use]
    pub const fn function(&self) -> u8 {
        self.0.function_bits()
    }

    /// Returns the function instance.
    #[must_use]
    pub const fn function_instance(&self) -> u8 {
        self.0.function_instance_bits()
    }

    /// A J1939 network may accommodate several ECUs of the same kind (i.e. same functionality).
    /// The Instance code separates them.
    #[must_use]
    pub const fn ecu_instance(&self) -> u8 {
        self.0.ecu_instance_bits()
    }

    /// The 11-Bit Manufacturer Code is assigned by the SAE.
    #[must_use]
    pub const fn manufacturer_code(&self) -> u16 {
        self.0.manufacturer_code_bits()
    }

    /// This field is assigned by the manufacturer, similar to a serial number, i.e. the code must
    /// be uniquely assigned to the unit.
    #[must_use]
    pub const fn identity_number(&self) -> u32 {
        self.0.identity_number_bits()
    }
}

#[cfg(test)]
mod data_tests {
    use super::*;

    #[test]
    fn test_data_bitfield() -> Result<(), anyhow::Error> {
        let data_a = Pdu::<Data>::from_hex("FFFF82DF1AFFFFFF");
        let be_bytes_a: [u8; 8] = [0xFF, 0xFF, 0x82, 0xDF, 0x1A, 0xFF, 0xFF, 0xFF];
        let le_bytes_a: [u8; 8] = [0xFF, 0xFF, 0xFF, 0x1A, 0xDF, 0x82, 0xFF, 0xFF];

        assert_eq!(be_bytes_a, data_a.to_be_bytes());
        assert_eq!(le_bytes_a, data_a.to_le_bytes());

        assert_eq!(18446606493475143679, data_a.into_bits());

        assert_eq!(Pdu::<Data>(Data(18446743089616977919)), data_a.to_be());
        assert_eq!(Pdu::<Data>(Data(18446606493475143679)), data_a.to_le());

        Ok(())
    }

    #[test]
    fn test_name_bitfield() {
        let name_a = Name::new()
            .with_arbitrary_address_bits(true)
            .with_industry_group_bits(0)
            .with_vehicle_system_instance_bits(0x5)
            .with_vehicle_system_bits(0x6)
            .with_reserved_bits(false)
            .with_function_bits(0x5)
            .with_function_instance_bits(0x2)
            .with_ecu_instance_bits(0x1)
            .with_manufacturer_code_bits(0x122)
            .with_identity_number_bits(0xB0309);

        let pdu_name = Pdu::<Name>(name_a);

        let bytes_a: [u8; 8] = [0x09, 0x03, 0x4B, 0x24, 0x11, 0x05, 0x0C, 0x85];
        let name_a_bytes = pdu_name.into_bits().to_le_bytes();

        assert_eq!(bytes_a, name_a_bytes);
    }
}
