use anyhow::anyhow;
use bitfield_struct::bitfield;

pub trait IdKind {}

impl IdKind for Standard {}
impl IdKind for Extended {}

pub type IdExtended = Id<Extended>;
pub type IdStandard = Id<Standard>;

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

pub struct Id<T: IdKind> {
    bitfield: T,
}

impl Id<Extended> {
    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let bits = u32::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x1FFF_FFFF {
            return Err(anyhow!(
                "Identifier bits out of range! Valid range is 0x000..0x1FFFFFFF"
            ));
        }
        let bitfield = Extended::from_bits(bits);

        Ok(Self { bitfield })
    }

    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_bits(bits: u32) -> Result<Self, anyhow::Error> {
        if bits > 0x1FFF_FFFF {
            return Err(anyhow!(
                "Identifier bits out of range! Valid range is 0x000..0x1FFFFFFF"
            ));
        }
        let bitfield = Extended::from_bits(bits);

        Ok(Self { bitfield })
    }

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

    #[must_use]
    pub const fn priority_bits(&self) -> u8 {
        self.bitfield.priority_bits()
    }

    #[must_use]
    pub const fn reserved_bits(&self) -> bool {
        self.bitfield.reserved_bits()
    }

    #[must_use]
    pub const fn data_page_bits(&self) -> bool {
        self.bitfield.data_page_bits()
    }

    #[must_use]
    pub const fn pdu_format_bits(&self) -> u8 {
        self.bitfield.pdu_format_bits()
    }

    #[must_use]
    pub const fn pdu_specific_bits(&self) -> u8 {
        self.bitfield.pdu_specific_bits()
    }

    #[must_use]
    pub const fn source_address_bits(&self) -> u8 {
        self.bitfield.source_address_bits()
    }
}

impl Id<Standard> {
    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let bits = u16::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x7FF {
            return Err(anyhow!(
                "Identifier bits out of range! Valid range is 0x000..0x7FF"
            ));
        }
        let bitfield = Standard::from_bits(bits);

        Ok(Self { bitfield })
    }

    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_bits(bits: u16) -> Result<Self, anyhow::Error> {
        if bits > 0x7FF {
            return Err(anyhow!(
                "Identifier bits out of range! Valid range is 0x000..0x7FF"
            ));
        }
        let bitfield = Standard::from_bits(bits);

        Ok(Self { bitfield })
    }

    #[must_use]
    pub const fn into_raw_parts(&self) -> (u8, bool, bool, u8) {
        let p = self.bitfield.priority_bits();
        let r = self.bitfield.reserved_bits();
        let dp = self.bitfield.data_page_bits();
        let pf = self.bitfield.pdu_format_bits();

        (p, r, dp, pf)
    }

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

    #[must_use]
    pub const fn priority_bits(&self) -> u8 {
        self.bitfield.priority_bits()
    }

    #[must_use]
    pub const fn reserved_bits(&self) -> bool {
        self.bitfield.reserved_bits()
    }

    #[must_use]
    pub const fn data_page_bits(&self) -> bool {
        self.bitfield.data_page_bits()
    }

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

        assert_eq!(0b00000_000_0_0_000000, id_a.0);
        assert_eq!(0b00000_011_1_1_011111, id_b.0);
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

        assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.0);
        assert_eq!(0b000_011_1_1_00001001_00000000_11111111, id_b.0);
    }

    #[test]
    fn test_extended_id() -> Result<(), anyhow::Error> {
        let id_a = Id::<Extended>::from_bits(0)?;

        assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.bitfield.0);
        Ok(())
    }

    #[test]
    fn test_standard_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "000F";

        let id_a = IdStandard::from_hex(hex_str)?;

        assert_eq!(0b00000_000_0_0_001111, id_a.bitfield.0);
        assert_eq!(0, id_a.priority_bits());
        assert_eq!(false, id_a.reserved_bits());
        assert_eq!(false, id_a.data_page_bits());
        assert_eq!(15, id_a.pdu_format_bits());

        Ok(())
    }

    #[test]
    fn test_extended_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "0CF00400";

        let id_a = IdExtended::from_hex(hex_str)?;

        assert_eq!(0b00001100111100000000010000000000, id_a.bitfield.0);
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

        assert_eq!(0b00000_000_0_0_001111, id_a.bitfield.0);
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

        assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id_a.bitfield.0);

        assert_eq!(3, id_a.priority_bits());
        assert_eq!(false, id_a.reserved_bits());
        assert_eq!(false, id_a.data_page_bits());
        assert_eq!(240, id_a.pdu_format_bits());

        Ok(())
    }
}
