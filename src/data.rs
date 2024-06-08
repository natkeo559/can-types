#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Data(u64);

impl Data {
    /// Creates a new data field from a hexadecimal string representation of the data bytes.
    /// # Errors
    /// - If the hex string fails to parse
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let bits = u64::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;

        Ok(Self(bits))
    }

    /// Creates a new data field from the given data field bits.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Return the data bits as a u64 integer
    #[must_use]
    pub const fn to_bits(&self) -> u64 {
        self.0
    }

    /// Converts the value to little-endian bytes.
    ///
    /// # Returns
    /// A byte array representing the value in little-endian byte order.
    #[must_use]
    pub const fn to_le_bytes(&self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    /// Converts the value to big-endian bytes.
    ///
    /// # Returns
    /// A byte array representing the value in big-endian byte order.
    #[must_use]
    pub const fn to_be_bytes(&self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    /// Converts the value to native-endian bytes.
    ///
    /// # Returns
    /// A byte array representing the value in native-endian byte order.
    #[must_use]
    pub const fn to_ne_bytes(&self) -> [u8; 8] {
        self.0.to_ne_bytes()
    }

    /// Converts the value to little-endian byte order.
    ///
    /// # Returns
    /// A new value in little-endian byte order.
    #[must_use]
    pub const fn to_le(&self) -> Self {
        Self(self.0.to_le())
    }

    /// Converts the value to big-endian byte order.
    ///
    /// # Returns
    /// A new value in big-endian byte order.
    #[must_use]
    pub const fn to_be(&self) -> Self {
        Self(self.0.to_be())
    }
}

#[cfg(test)]
mod data_tests {
    use super::*;

    #[test]
    fn test_data_field() -> Result<(), anyhow::Error> {
        let data_a = Data::from_hex("FFFF82DF1AFFFFFF")?;
        let be_bytes_a = [0xFF, 0xFF, 0x82, 0xDF, 0x1A, 0xFF, 0xFF, 0xFF];
        let le_bytes_a = [0xFF, 0xFF, 0xFF, 0x1A, 0xDF, 0x82, 0xFF, 0xFF];

        assert_eq!(be_bytes_a, data_a.to_be_bytes());
        assert_eq!(le_bytes_a, data_a.to_le_bytes());

        assert_eq!(18446606493475143679, data_a.to_bits());

        assert_eq!(Data(18446743089616977919), data_a.to_be());
        assert_eq!(Data(18446606493475143679), data_a.to_le());

        Ok(())
    }
}
