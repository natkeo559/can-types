if_alloc! {
    use crate::prelude::String;
}

use bitfield_struct::bitfield;

use crate::conversion::Conversion;

/// Bitfield representing an 8-byte data field
/// messages.
#[bitfield(u64, order = Msb)]
#[derive(PartialEq, Eq)]
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

impl Conversion<u64> for Data {
    type Error = anyhow::Error;

    /// Creates a new 64-bit integer from the `Data` bitfield.
    /// # Errors
    /// - Never (conversion is trivial)
    fn try_into_bits(self) -> Result<u64, Self::Error> {
        Ok(self.into_bits())
    }

    /// Creates a new base-16 (hex) `String` from the `Data` bitfield.
    /// # Errors
    /// - If invalid encoding of provided Base16 string
    /// - If insufficient output buffer length
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn try_into_hex(self) -> Result<String, Self::Error> {
        let mut buffer: [u8; 8] = [b'0'; 8];
        let hex_bytes: &[u8] =
            base16ct::upper::encode(&self.into_bits().to_be_bytes(), &mut buffer)
                .map_err(anyhow::Error::msg)?;
        String::from_utf8(hex_bytes.to_vec()).map_err(anyhow::Error::msg)
    }

    /// Creates a new `Data` bitfield from a 64-bit integer.
    /// # Errors
    /// - Never (conversion is trivial)
    fn try_from_bits(bits: u64) -> Result<Self, Self::Error> {
        Ok(Self(bits))
    }

    /// Creates a new `Data` bitfield from a base-16 (hex) string slice.
    /// # Errors
    /// - If invalid encoding of provided Base16 string
    /// - If insufficient output buffer length
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let mut buffer: [u8; 8] = [b'0'; 8];
        base16ct::upper::decode(hex_str, &mut buffer).map_err(anyhow::Error::msg)?;
        let bits: u64 = u64::from_be_bytes(buffer);

        Ok(Self(bits))
    }

    /// Creates a new 64-bit integer from the `Data` bitfield.
    fn into_bits(self) -> u64 {
        self.into_bits()
    }

    /// Creates a new base-16 (hex) `String` from the `Data` bitfield.
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        let mut buffer: [u8; 8] = [b'0'; 8];
        let hex_bytes: &[u8] =
            base16ct::upper::encode(&self.into_bits().to_be_bytes(), &mut buffer)
                .unwrap_or_default();
        String::from_utf8(hex_bytes.to_vec()).unwrap_or_default()
    }

    /// Creates a new `Data` bitfield from a 64-bit integer.
    fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Creates a new `Data` bitfield from a base-16 (hex) string slice.
    fn from_hex(hex_str: &str) -> Self {
        let mut buffer: [u8; 8] = [b'0'; 8];
        base16ct::upper::decode(hex_str, &mut buffer).unwrap_or_default();
        let bits: u64 = u64::from_be_bytes(buffer);

        Self(bits)
    }
}

impl Data {
    /// Retrieve byte 0.
    #[must_use]
    pub const fn byte_0(self) -> u8 {
        self.byte_0_bits()
    }

    /// Retrieve byte 1.
    #[must_use]
    pub const fn byte_1(self) -> u8 {
        self.byte_1_bits()
    }

    /// Retrieve byte 2.
    #[must_use]
    pub const fn byte_2(self) -> u8 {
        self.byte_2_bits()
    }

    /// Retrieve byte 3.
    #[must_use]
    pub const fn byte_3(self) -> u8 {
        self.byte_3_bits()
    }

    /// Retrieve byte 4.
    #[must_use]
    pub const fn byte_4(self) -> u8 {
        self.byte_4_bits()
    }

    /// Retrieve byte 5.
    #[must_use]
    pub const fn byte_5(self) -> u8 {
        self.byte_5_bits()
    }

    /// Retrieve byte 6.
    #[must_use]
    pub const fn byte_6(self) -> u8 {
        self.byte_6_bits()
    }

    /// Retrieve byte 7.
    #[must_use]
    pub const fn byte_7(self) -> u8 {
        self.byte_7_bits()
    }

    /// Return the 64-bit `Data` bitfield as little-endian bytes.
    #[must_use]
    pub const fn to_le_bytes(&self) -> [u8; 8] {
        self.into_bits().to_le_bytes()
    }

    /// Return the 64-bit `Data` bitfield as big-endian bytes.
    #[must_use]
    pub const fn to_be_bytes(&self) -> [u8; 8] {
        self.into_bits().to_be_bytes()
    }

    /// Return the 64-bit `Data` bitfield as native-endian bytes.
    #[must_use]
    pub const fn to_ne_bytes(&self) -> [u8; 8] {
        self.into_bits().to_ne_bytes()
    }

    /// Convert the `Data` bitfield to little-endian byte format.
    #[must_use]
    pub const fn to_le(&self) -> Self {
        Self(self.into_bits().to_le())
    }

    /// Convert the `Data` bitfield to big-endian byte format.
    #[must_use]
    pub const fn to_be(&self) -> Self {
        Self(self.into_bits().to_be())
    }
}

#[cfg(test)]
mod data_tests {
    use super::*;

    #[test]
    fn test_data_field() -> Result<(), anyhow::Error> {
        let data_a = Data::from_hex("FFFF82DF1AFFFFFF");
        let be_bytes_a: [u8; 8] = [0xFF, 0xFF, 0x82, 0xDF, 0x1A, 0xFF, 0xFF, 0xFF];
        let le_bytes_a: [u8; 8] = [0xFF, 0xFF, 0xFF, 0x1A, 0xDF, 0x82, 0xFF, 0xFF];

        assert_eq!(be_bytes_a, data_a.to_be_bytes());
        assert_eq!(le_bytes_a, data_a.to_le_bytes());

        assert_eq!(18446606493475143679, data_a.into_bits());

        assert_eq!(Data(18446743089616977919), data_a.to_be());
        assert_eq!(Data(18446606493475143679), data_a.to_le());

        Ok(())
    }
}
