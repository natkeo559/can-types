use anyhow::anyhow;
use bitfield_struct::bitfield;

pub trait IdKind {}

impl IdKind for Standard {}
impl IdKind for Extended {}

#[bitfield(u16, order = Msb)]
struct Standard {
    #[bits(5)]
    _padding: u8,
    #[bits(3)]
    priority: u8,
    #[bits(1)]
    reserved: bool,
    #[bits(1)]
    data_page: bool,
    #[bits(6)]
    pdu_format: u8,
}

#[bitfield(u32, order = Msb)]
struct Extended {
    #[bits(3)]
    _padding: u8,
    #[bits(3)]
    priority: u8,
    #[bits(1)]
    reserved: bool,
    #[bits(1)]
    data_page: bool,
    #[bits(8)]
    pdu_format: u8,
    #[bits(8)]
    pdu_specific: u8,
    #[bits(8)]
    source_address: u8,
}

pub struct Id<T: IdKind> {
    bitfield: T,
}

impl Id<Extended> {
    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let bits = u32::from_str_radix(hex_str, 16)?;
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
        let p = self.bitfield.priority();
        let r = self.bitfield.reserved();
        let dp = self.bitfield.data_page();
        let pf = self.bitfield.pdu_format();
        let ps = self.bitfield.pdu_specific();
        let sa = self.bitfield.source_address();

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
            .with_priority(priority)
            .with_reserved(reserved)
            .with_data_page(data_page)
            .with_pdu_format(pdu_format)
            .with_pdu_specific(pdu_specific)
            .with_source_address(source_addr);

        Ok(Self { bitfield })
    }

    #[must_use]
    pub const fn priority(&self) -> u8 {
        self.bitfield.priority()
    }

    #[must_use]
    pub const fn reserved(&self) -> bool {
        self.bitfield.reserved()
    }

    #[must_use]
    pub const fn data_page(&self) -> bool {
        self.bitfield.data_page()
    }

    #[must_use]
    pub const fn pdu_format(&self) -> u8 {
        self.bitfield.pdu_format()
    }

    #[must_use]
    pub const fn pdu_specific(&self) -> u8 {
        self.bitfield.pdu_specific()
    }

    #[must_use]
    pub const fn source_address(&self) -> u8 {
        self.bitfield.source_address()
    }
}

impl Id<Standard> {
    /// # Errors
    /// - If requested identifier is out of the valid range for identifiers.
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let bits = u16::from_str_radix(hex_str, 16)?;
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
        let p = self.bitfield.priority();
        let r = self.bitfield.reserved();
        let dp = self.bitfield.data_page();
        let pf = self.bitfield.pdu_format();

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
            .with_priority(priority)
            .with_reserved(reserved)
            .with_data_page(data_page)
            .with_pdu_format(pdu_format);

        Ok(Self { bitfield })
    }

    #[must_use]
    pub const fn priority(&self) -> u8 {
        self.bitfield.priority()
    }

    #[must_use]
    pub const fn reserved(&self) -> bool {
        self.bitfield.reserved()
    }

    #[must_use]
    pub const fn data_page(&self) -> bool {
        self.bitfield.data_page()
    }

    #[must_use]
    pub const fn pdu_format(&self) -> u8 {
        self.bitfield.pdu_format()
    }
}

#[cfg(test)]
mod id_tests {
    use super::*;

    #[test]
    fn test_new_standard_id_bitfield() {
        let id_a = Standard::new()
            .with_priority(0)
            .with_reserved(false)
            .with_data_page(false)
            .with_pdu_format(0);

        let id_b = Standard::new()
            .with_priority(3)
            .with_reserved(true)
            .with_data_page(true)
            .with_pdu_format(31);

        assert_eq!(0b00000_000_0_0_000000, id_a.0);
        assert_eq!(0b00000_011_1_1_011111, id_b.0);
    }

    #[test]
    fn test_new_extended_id_bitfield() {
        let id_a = Extended::new()
            .with_priority(0)
            .with_reserved(false)
            .with_data_page(false)
            .with_pdu_format(0)
            .with_pdu_specific(0)
            .with_source_address(0);

        let id_b = Extended::new()
            .with_priority(3)
            .with_reserved(true)
            .with_data_page(true)
            .with_pdu_format(9)
            .with_pdu_specific(0)
            .with_source_address(255);

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

        let id_a: Id<Standard> = Id::<Standard>::from_hex(hex_str)?;

        assert_eq!(0b00000_000_0_0_001111, id_a.bitfield.0);
        assert_eq!(0, id_a.priority());
        assert_eq!(false, id_a.reserved());
        assert_eq!(false, id_a.data_page());
        assert_eq!(15, id_a.pdu_format());

        Ok(())
    }

    #[test]
    fn test_extended_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "0CF00400";

        let id_a = Id::<Extended>::from_hex(hex_str)?;

        assert_eq!(0b00001100111100000000010000000000, id_a.bitfield.0);
        assert_eq!(3, id_a.priority());
        assert_eq!(false, id_a.reserved());
        assert_eq!(false, id_a.data_page());
        assert_eq!(240, id_a.pdu_format());

        Ok(())
    }

    #[test]
    fn test_standard_from_bits() -> Result<(), anyhow::Error> {
        let bits = 15;

        let id_a: Id<Standard> = Id::<Standard>::from_bits(bits)?;

        assert_eq!(0b00000_000_0_0_001111, id_a.bitfield.0);
        assert_eq!(0, id_a.priority());
        assert_eq!(false, id_a.reserved());
        assert_eq!(false, id_a.data_page());
        assert_eq!(15, id_a.pdu_format());

        Ok(())
    }

    #[test]
    fn test_extended_from_bits() -> Result<(), anyhow::Error> {
        let bits = 217056256;

        let id_a = Id::<Extended>::from_bits(bits)?;

        assert_eq!(0b00001100111100000000010000000000, id_a.bitfield.0);
        assert_eq!(3, id_a.priority());
        assert_eq!(false, id_a.reserved());
        assert_eq!(false, id_a.data_page());
        assert_eq!(240, id_a.pdu_format());

        Ok(())
    }
}
