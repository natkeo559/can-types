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
    reserved: u8,
    #[bits(1)]
    data_page: u8,
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
    reserved: u8,
    #[bits(1)]
    data_page: u8,
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
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let dec = u32::from_str_radix(hex_str, 16)?;
        let bitfield = Extended::from_bits(dec);

        Ok(Self { bitfield })
    }

    pub fn as_hex(&self) -> () {}

    pub fn from_bits(bits: u32) -> Result<Self, anyhow::Error> {
        let bitfield = Extended::from_bits(bits);

        Ok(Self { bitfield })
    }

    pub fn into_raw_parts(&self) -> (u8, u8, u8, u8, u8, u8) {
        let p = self.bitfield.priority();
        let r = self.bitfield.reserved();
        let dp = self.bitfield.data_page();
        let pf = self.bitfield.pdu_format();
        let ps = self.bitfield.pdu_specific();
        let sa = self.bitfield.source_address();

        (p, r, dp, pf, ps, sa)
    }

    pub fn from_raw_parts(
        priority: u8,
        reserved: u8,
        data_page: u8,
        pdu_format: u8,
        pdu_specific: u8,
        source_addr: u8,
    ) -> Result<Self, anyhow::Error> {
        if priority > 7 {
            return Err(anyhow::anyhow!(
                "Invalid priority! The priority value must be between 0 and 7 inclusive - got {}.",
                priority
            ));
        }

        if reserved > 1 {
            return Err(anyhow::anyhow!(
                "Invalid reserved bit! The reserved bit must be 0 or 1 - got {}.",
                reserved
            ));
        }

        if data_page > 1 {
            return Err(anyhow::anyhow!(
                "Invalid data page bit! The data page bit must be 0 or 1 - got {}.",
                data_page
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

    pub fn priority(&self) -> u8 {
        self.bitfield.priority()
    }

    pub fn reserved(&self) -> u8 {
        self.bitfield.reserved()
    }

    pub fn data_page(&self) -> u8 {
        self.bitfield.data_page()
    }

    pub fn pdu_format(&self) -> u8 {
        self.bitfield.pdu_format()
    }

    pub fn pdu_specific(&self) -> u8 {
        self.bitfield.pdu_specific()
    }

    pub fn source_address(&self) -> u8 {
        self.bitfield.source_address()
    }

}

impl Id<Standard> {
    pub fn from_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let dec = u16::from_str_radix(hex_str, 16)?;
        let bitfield = Standard::from_bits(dec);

        Ok(Self { bitfield })
    }

    pub fn as_hex(&self) -> () {}

    pub fn from_bits(bits: u16) -> Result<Self, anyhow::Error> {
        let bitfield = Standard::from_bits(bits);

        Ok(Self { bitfield })
    }

    pub fn into_raw_parts(&self) -> (u8, u8, u8, u8) {
        let p = self.bitfield.priority();
        let r = self.bitfield.reserved();
        let dp = self.bitfield.data_page();
        let pf = self.bitfield.pdu_format();

        (p, r, dp, pf)
    }

    pub fn from_raw_parts(
        priority: u8,
        reserved: u8,
        data_page: u8,
        pdu_format: u8
    ) -> Result<Self, anyhow::Error> {
        if priority > 7 {
            return Err(anyhow::anyhow!(
                "Invalid priority! The priority value must be between 0 and 7 inclusive - got {}.",
                priority
            ));
        }

        if reserved > 1 {
            return Err(anyhow::anyhow!(
                "Invalid reserved bit! The reserved bit must be 0 or 1 - got {}.",
                reserved
            ));
        }

        if data_page > 1 {
            return Err(anyhow::anyhow!(
                "Invalid data page bit! The data page bit must be 0 or 1 - got {}.",
                data_page
            ));
        }

        if pdu_format > 63 {
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

    pub fn priority(&self) -> u8 {
        self.bitfield.priority()
    }

    pub fn reserved(&self) -> u8 {
        self.bitfield.reserved()
    }

    pub fn data_page(&self) -> u8 {
        self.bitfield.data_page()
    }

    pub fn pdu_format(&self) -> u8 {
        self.bitfield.pdu_format()
    }
}

#[cfg(test)]
mod id_tests {
    use super::*;

    #[test]
    fn test_new_id_bits() {
        let id_std_a = Standard::new()
            .with_priority(0)
            .with_reserved(0)
            .with_data_page(0)
            .with_pdu_format(0);

        let id_std_b = Standard::new()
            .with_priority(3)
            .with_reserved(1)
            .with_data_page(1)
            .with_pdu_format(31);

        let id_ext_a = Extended::new()
            .with_priority(0)
            .with_reserved(0)
            .with_data_page(0)
            .with_pdu_format(0)
            .with_pdu_specific(0)
            .with_source_address(0);

        let id_ext_b = Extended::new()
            .with_priority(3)
            .with_reserved(1)
            .with_data_page(1)
            .with_pdu_format(9)
            .with_pdu_specific(0)
            .with_source_address(255);

        assert_eq!(0b00000_000_0_0_000000, id_std_a.0);
        assert_eq!(0b00000_011_1_1_011111, id_std_b.0);
        assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_ext_a.0);
        assert_eq!(0b000_011_1_1_00001001_00000000_11111111, id_ext_b.0);
    }

    #[test]
    fn test_extended_id() -> Result<(), anyhow::Error> {
        let id_a = Id::<Extended>::from_bits(0)?;

        assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.bitfield.0);
        Ok(())
    }

    #[test]
    fn test_extended_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "0CF00400";

        let id_ext_a = Id::<Extended>::from_hex(hex_str)?;

        assert_eq!(0b00001100111100000000010000000000, id_ext_a.bitfield.0);
        assert_eq!(3, id_ext_a.priority());
        assert_eq!(0, id_ext_a.reserved());
        assert_eq!(0, id_ext_a.data_page());
        assert_eq!(240, id_ext_a.pdu_format());

        Ok(())
    }

    #[test]
    fn test_standard_from_hex() -> Result<(), anyhow::Error> {
        let hex_str = "000F";

        let id_ext_a = Id::<Standard>::from_hex(hex_str)?;

        assert_eq!(0b00000_000_0_0_001111, id_ext_a.bitfield.0);
        assert_eq!(0, id_ext_a.priority());
        assert_eq!(0, id_ext_a.reserved());
        assert_eq!(0, id_ext_a.data_page());
        assert_eq!(15, id_ext_a.pdu_format());

        Ok(())
    }
}
