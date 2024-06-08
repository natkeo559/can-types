use bitfield_struct::bitfield;

/// Trait representing the kind of CAN identifier.
pub trait IdKind {}

impl IdKind for Standard {}
impl IdKind for Extended {}

/// Bitfield representation of a standard 11-bit CAN identifier.
#[bitfield(u16, order = Msb)]
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
#[bitfield(u32, order = Msb)]
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
pub struct Id<T: IdKind> {
    bitfield: T,
}

/// Represents a Controller Area Network (CAN) extended 29-bit identifier.
pub type IdExtended = Id<Extended>;
/// Represents a Controller Area Network (CAN) standard 11-bit identifier.
pub type IdStandard = Id<Standard>;

impl IdExtended {
    /// Returns the integer representation of the Identifier bitfield
    #[must_use]
    pub fn to_bits(&self) -> u32 {
        self.bitfield.0
    }

    /// Creates a new 29-bit identifier from a hexadecimal string representation of the identifier bits.
    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let mut buf = [0; 4];
        base16ct::upper::decode(hex_str, &mut buf).map_err(anyhow::Error::msg)?;
        let bits = u32::from_be_bytes(buf);
        if bits > 0x1FFF_FFFF {
            Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0x00000000..0x1FFFFFFF - got {:#08X}",
                bits
            ))
        } else {
            Ok(Self {
                bitfield: Extended::from_bits(bits),
            })
        }
    }

    /// Creates a new hexidecimal string representing the 29-bit identifier.
    /// # Errors
    /// - If insufficient destination buffer length (function assumes 8 bytes)
    /// - If resulting hex bytes are not UTF-8
    pub fn to_hex<'a>(&self) -> Result<&'a str, anyhow::Error> {
        static mut BUFFER: [u8; 8] = [0; 8];
        let hex_bytes =
            base16ct::upper::encode(&self.to_bits().to_be_bytes(), unsafe { &mut BUFFER })
                .map_err(anyhow::Error::msg)?;
        let hex_str = core::str::from_utf8(hex_bytes).map_err(anyhow::Error::msg)?;
        Ok(hex_str)
    }

    /// Creates a new 29-bit identifier from the given identifier bits.
    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_bits(bits: u32) -> Result<Self, anyhow::Error> {
        if bits > 0x1FFF_FFFF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0..536870911 - got {}",
                bits
            ));
        }
        let bitfield = Extended::from_bits(bits);

        Ok(Self { bitfield })
    }

    /// Creates an error identifier.
    #[must_use]
    pub fn error() -> Self {
        let bitfield = Extended::from_bits(0xFFFF_FFFF);
        Self { bitfield }
    }

    /// Returns the raw parts of the 29-bit identifier.
    #[must_use]
    pub const fn into_raw_parts(&self) -> (u8, bool, bool, u8, u8, u8) {
        let p = self.bitfield.priority_bits();
        let r = self.bitfield.reserved_bits();
        let dp = self.bitfield.data_page_bits();
        let pf = self.bitfield.pdu_format_bits();
        let ps = self.bitfield.pdu_specific_bits();
        let sa = self.bitfield.source_address_bits();

        (p, r, dp, pf, ps, sa)
    }

    /// Creates a new 29-bit identifier from the given raw parts.
    /// # Errors
    /// - If requested priority is out of the valid range for message priorities.
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

        Ok(Self { bitfield })
    }

    /// Retrieves the priority bits of the 29-bit identifier.
    #[must_use]
    pub const fn priority_bits(&self) -> u8 {
        self.bitfield.priority_bits()
    }

    /// Retrieves the reserved bit of the 29-bit identifier.
    #[must_use]
    pub const fn reserved_bits(&self) -> bool {
        self.bitfield.reserved_bits()
    }

    /// Retrieves the data page bit of the 29-bit identifier.
    #[must_use]
    pub const fn data_page_bits(&self) -> bool {
        self.bitfield.data_page_bits()
    }

    /// Retrieves the PDU format bits of the 29-bit identifier.
    #[must_use]
    pub const fn pdu_format_bits(&self) -> u8 {
        self.bitfield.pdu_format_bits()
    }

    /// Retrieves the PDU specific bits of the 29-bit identifier.
    #[must_use]
    pub const fn pdu_specific_bits(&self) -> u8 {
        self.bitfield.pdu_specific_bits()
    }

    /// Retrieves the source address bits of the 29-bit identifier.
    #[must_use]
    pub const fn source_address_bits(&self) -> u8 {
        self.bitfield.source_address_bits()
    }
}

impl IdStandard {
    /// Returns the integer representation of the Identifier bitfield
    #[must_use]
    pub fn to_bits(&self) -> u16 {
        self.bitfield.0
    }

    /// Creates a new 11-bit identifier from a hexadecimal string representation of the identifier bits.
    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let bits = u16::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x7FF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0x000..0x7FF - got {:#03X}",
                bits
            ));
        }
        let bitfield = Standard::from_bits(bits);

        Ok(Self { bitfield })
    }

    /// Creates a new hexidecimal string representing the 29-bit identifier.
    /// # Errors
    /// - If insufficient destination buffer length (function assumes 8 bytes)
    /// - If resulting hex bytes are not UTF-8
    pub fn to_hex<'a>(&self) -> Result<&'a str, anyhow::Error> {
        static mut BUFFER: [u8; 4] = [0; 4];
        let hex_bytes =
            base16ct::upper::encode(&self.to_bits().to_be_bytes(), unsafe { &mut BUFFER })
                .map_err(anyhow::Error::msg)?;
        let hex_str = core::str::from_utf8(hex_bytes).map_err(anyhow::Error::msg)?;
        Ok(&hex_str[1..])
    }

    /// Creates a new 11-bit identifier from the given identifier bits.
    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_bits(bits: u16) -> Result<Self, anyhow::Error> {
        if bits > 0x7FF {
            return Err(anyhow::anyhow!(
                "Identifier bits out of range! Valid range is 0..2047 - got {}",
                bits
            ));
        }
        let bitfield = Standard::from_bits(bits);

        Ok(Self { bitfield })
    }

    /// Creates an error identifier.
    #[must_use]
    pub fn error() -> Self {
        let bitfield = Standard::from_bits(0xFFF);
        Self { bitfield }
    }

    /// Retrieves the raw parts of the 11-bit identifier.
    #[must_use]
    pub const fn into_raw_parts(&self) -> (u8, bool, bool, u8) {
        let p = self.bitfield.priority_bits();
        let r = self.bitfield.reserved_bits();
        let dp = self.bitfield.data_page_bits();
        let pf = self.bitfield.pdu_format_bits();

        (p, r, dp, pf)
    }

    /// Creates a new 11-bit identifier from the given raw parts.
    /// # Errors
    /// - If requested priority is out of the valid range for message priorities.
    /// - If requested pdu format is out of the valid range for pdu formats.
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

        Ok(Self { bitfield })
    }

    /// Retrieves the priority bits of the 11-bit identifier.
    #[must_use]
    pub const fn priority_bits(&self) -> u8 {
        self.bitfield.priority_bits()
    }

    /// Retrieves the reserved bit of the 11-bit identifier.
    #[must_use]
    pub const fn reserved_bits(&self) -> bool {
        self.bitfield.reserved_bits()
    }

    /// Retrieves the data page bit of the 11-bit identifier.
    #[must_use]
    pub const fn data_page_bits(&self) -> bool {
        self.bitfield.data_page_bits()
    }

    /// Retrieves the PDU format bits of the 11-bit identifier.
    #[must_use]
    pub const fn pdu_format_bits(&self) -> u8 {
        self.bitfield.pdu_format_bits()
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
    fn test_extended_id() -> Result<(), anyhow::Error> {
        let id_a = Id::<Extended>::from_bits(0)?;

        assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.to_bits());
        Ok(())
    }

    #[test]
    fn test_standard_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "00F";

        let id_a = IdStandard::from_hex(hex_str)?;

        assert_eq!(0b00000_000_0_0_001111, id_a.to_bits());
        assert_eq!(0, id_a.priority_bits());
        assert_eq!(false, id_a.reserved_bits());
        assert_eq!(false, id_a.data_page_bits());
        assert_eq!(15, id_a.pdu_format_bits());

        Ok(())
    }

    #[test]
    fn test_standard_to_hex() -> Result<(), anyhow::Error> {
        let id_dec = 15;
        let id_hex = "00F";
        let id_a = IdStandard::from_bits(id_dec)?;

        assert_eq!(id_hex, id_a.to_hex()?);

        Ok(())
    }

    #[test]
    fn test_extended_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "0CF00400";

        let id_a = IdExtended::from_hex(hex_str)?;

        assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id_a.to_bits());
        assert_eq!(3, id_a.priority_bits());
        assert_eq!(false, id_a.reserved_bits());
        assert_eq!(false, id_a.data_page_bits());
        assert_eq!(240, id_a.pdu_format_bits());

        Ok(())
    }

    #[test]
    fn test_standard_from_bits() -> Result<(), anyhow::Error> {
        let bits = 15;

        let id_a = IdStandard::from_bits(bits)?;

        assert_eq!(0b00000_000_0_0_001111, id_a.to_bits());
        assert_eq!(0, id_a.priority_bits());
        assert_eq!(false, id_a.reserved_bits());
        assert_eq!(false, id_a.data_page_bits());
        assert_eq!(15, id_a.pdu_format_bits());

        Ok(())
    }

    #[test]
    fn test_extended_from_bits() -> Result<(), anyhow::Error> {
        let bits = 217056256;

        let id_a = IdExtended::from_bits(bits)?;

        assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id_a.to_bits());

        assert_eq!(3, id_a.priority_bits());
        assert_eq!(false, id_a.reserved_bits());
        assert_eq!(false, id_a.data_page_bits());
        assert_eq!(240, id_a.pdu_format_bits());

        Ok(())
    }

    #[test]
    pub fn test_extended_to_hex() -> Result<(), anyhow::Error> {
        let id_dec = 217056256;
        let id_hex = "0CF00400";
        let id_a = IdExtended::from_bits(id_dec)?;

        assert_eq!(id_hex, id_a.to_hex()?);

        Ok(())
    }
}
