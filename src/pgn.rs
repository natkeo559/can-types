use anyhow::anyhow;
use bitfield_struct::bitfield;

use crate::IdExtended;

#[derive(Debug, PartialEq, Eq)]
pub enum PduAssignment {
    Sae(u32),
    Manufacturer(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PduFormat {
    Pdu1(u8),
    Pdu2(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommunicationMode {
    P2P,
    Broadcast,
}

#[derive(Debug, PartialEq, Eq)]

pub enum GroupExtension {
    None,
    Some(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DestinationAddress {
    None,
    Some(u8),
}

/// Represents the bit layout of a Parameter Group Number (PGN) within a Controller Area Network (CAN) message or a J1939 message.
///
/// This struct provides a structured representation of the bits composing a PGN, including reserved bits, data page bits,
/// PDU format bits, and PDU specific bits.
/// ```
#[bitfield(u32, order = Msb)]
pub struct PgnBits {
    #[bits(14)]
    _padding_bits: u16,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(1)]
    data_page_bits: bool,
    #[bits(8)]
    pdu_format_bits: u8,
    #[bits(8)]
    pdu_specific_bits: u8,
}

impl IdExtended {
    #[must_use]
    pub const fn pgn_bits(&self) -> u32 {
        let pgn_bitfield = PgnBits::new()
            .with_reserved_bits(self.reserved_bits())
            .with_data_page_bits(self.data_page_bits())
            .with_pdu_format_bits(self.pdu_format_bits())
            .with_pdu_specific_bits(self.pdu_specific_bits());

        pgn_bitfield.into_bits()
    }

    #[must_use]
    pub const fn pgn(&self) -> PgnBits {
        PgnBits::new()
            .with_reserved_bits(self.reserved_bits())
            .with_data_page_bits(self.data_page_bits())
            .with_pdu_format_bits(self.pdu_format_bits())
            .with_pdu_specific_bits(self.pdu_specific_bits())
    }

    #[must_use]
    pub const fn pdu_format(&self) -> PduFormat {
        match (self.pdu_format_bits() < 240, self.pdu_format_bits()) {
            (true, a) => PduFormat::Pdu1(a),
            (false, b) => PduFormat::Pdu2(b),
        }
    }

    #[must_use]
    pub const fn group_extension(&self) -> GroupExtension {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => GroupExtension::None,
            PduFormat::Pdu2(_) => GroupExtension::Some(self.pdu_specific_bits()),
        }
    }

    #[must_use]
    pub const fn destination_address(&self) -> DestinationAddress {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => DestinationAddress::Some(self.pdu_specific_bits()),
            PduFormat::Pdu2(_) => DestinationAddress::None,
        }
    }

    #[must_use]
    pub const fn communication_mode(&self) -> CommunicationMode {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => CommunicationMode::P2P,
            PduFormat::Pdu2(_) => CommunicationMode::Broadcast,
        }
    }

    #[must_use]
    pub const fn is_p2p(&self) -> bool {
        match self.communication_mode() {
            CommunicationMode::P2P => true,
            CommunicationMode::Broadcast => false,
        }
    }

    #[must_use]
    pub const fn is_broadcast(&self) -> bool {
        match self.communication_mode() {
            CommunicationMode::P2P => false,
            CommunicationMode::Broadcast => true,
        }
    }

    /// Get the PDU assignment (either SAE or Manufacturer).
    /// Returns the assignment with the `u32` pgn value.
    /// # Note
    /// There are gaps between pgn bit ranges which aren't assignable.
    /// # Errors
    /// - If PGN is not withing a known valid range.
    pub fn pdu_assignment(&self) -> Result<PduAssignment, anyhow::Error> {
        match self.pgn_bits() {
            0x0000_0000..=0x0000_EE00
            | 0x0000_F000..=0x0000_FEFF
            | 0x0001_0000..=0x0001_EE00
            | 0x0001_F000..=0x0001_FEFF => Ok(PduAssignment::Sae(self.pgn_bits())),

            0x0000_EF00 | 0x0000_FF00..=0x0000_FFFF | 0x0001_EF00 | 0x0001_FF00..=0x0001_FFFF => {
                Ok(PduAssignment::Manufacturer(self.pgn_bits()))
            }
            _ => Err(anyhow!("PGN not within a known valid range!")),
        }
    }
}

#[cfg(test)]
mod pgn_tests {
    use crate::{
        CommunicationMode, DestinationAddress, GroupExtension, IdExtended, PduAssignment, PduFormat,
    };
    extern crate std;
    #[test]
    fn test_pdu_assignment() -> Result<(), anyhow::Error> {
        let id_a = IdExtended::from_hex("18FEF200")?;
        let id_b = IdExtended::from_hex("1CFE9201")?;
        let id_c = IdExtended::from_hex("10FF2121")?;
        let id_d = IdExtended::from_hex("0C00290B")?;

        let assignment_a = id_a.pdu_assignment()?;
        let assignment_b = id_b.pdu_assignment()?;
        let assignment_c = id_c.pdu_assignment()?;
        let assignment_d = id_d.pdu_assignment()?;

        assert_eq!(PduAssignment::Sae(65266), assignment_a);
        assert_eq!(PduAssignment::Sae(65170), assignment_b);
        assert_eq!(PduAssignment::Manufacturer(65313), assignment_c);
        assert_eq!(PduAssignment::Sae(41), assignment_d);

        Ok(())
    }

    #[test]
    fn test_communication_mode() -> Result<(), anyhow::Error> {
        let id_a = IdExtended::from_hex("18FEF200")?;
        let id_b = IdExtended::from_hex("1CFE9201")?;
        let id_c = IdExtended::from_hex("10FF2121")?;
        let id_d = IdExtended::from_hex("0C00290B")?;

        let comms_mode_a = id_a.communication_mode();
        let comms_mode_b = id_b.communication_mode();
        let comms_mode_c = id_c.communication_mode();
        let comms_mode_d = id_d.communication_mode();

        assert_eq!(CommunicationMode::Broadcast, comms_mode_a);
        assert_eq!(CommunicationMode::Broadcast, comms_mode_b);
        assert_eq!(CommunicationMode::Broadcast, comms_mode_c);
        assert_eq!(CommunicationMode::P2P, comms_mode_d);

        Ok(())
    }

    #[test]
    fn test_destination_address() -> Result<(), anyhow::Error> {
        let id_a = IdExtended::from_hex("18FEF200")?;
        let id_b = IdExtended::from_hex("1CFE9201")?;
        let id_c = IdExtended::from_hex("10FF2121")?;
        let id_d = IdExtended::from_hex("0C00290B")?;

        let dest_addr_a = id_a.destination_address();
        let dest_addr_b = id_b.destination_address();
        let dest_addr_c = id_c.destination_address();
        let dest_addr_d = id_d.destination_address();

        assert_eq!(DestinationAddress::None, dest_addr_a);
        assert_eq!(DestinationAddress::None, dest_addr_b);
        assert_eq!(DestinationAddress::None, dest_addr_c);
        assert_eq!(DestinationAddress::Some(41), dest_addr_d);

        Ok(())
    }

    #[test]
    fn test_group_extension() -> Result<(), anyhow::Error> {
        let id_a = IdExtended::from_hex("18FEF200")?;
        let id_b = IdExtended::from_hex("1CFE9201")?;
        let id_c = IdExtended::from_hex("10FF2121")?;
        let id_d = IdExtended::from_hex("0C00290B")?;

        let group_ext_a = id_a.group_extension();
        let group_ext_b = id_b.group_extension();
        let group_ext_c = id_c.group_extension();
        let group_ext_d = id_d.group_extension();

        assert_eq!(GroupExtension::Some(242), group_ext_a);
        assert_eq!(GroupExtension::Some(146), group_ext_b);
        assert_eq!(GroupExtension::Some(33), group_ext_c);
        assert_eq!(GroupExtension::None, group_ext_d);

        Ok(())
    }

    #[test]
    fn test_pdu_format() -> Result<(), anyhow::Error> {
        let id_a = IdExtended::from_hex("18FEF200")?;
        let id_b = IdExtended::from_hex("1CFE9201")?;
        let id_c = IdExtended::from_hex("10FF2121")?;
        let id_d = IdExtended::from_hex("0C00290B")?;

        let pdu_format_a = id_a.pdu_format();
        let pdu_format_b = id_b.pdu_format();
        let pdu_format_c = id_c.pdu_format();
        let pdu_format_d = id_d.pdu_format();

        assert_eq!(PduFormat::Pdu2(254), pdu_format_a);
        assert_eq!(PduFormat::Pdu2(254), pdu_format_b);
        assert_eq!(PduFormat::Pdu2(255), pdu_format_c);
        assert_eq!(PduFormat::Pdu1(0), pdu_format_d);

        Ok(())
    }

    #[test]
    fn test_pgn_bits() -> Result<(), anyhow::Error> {
        let id_a = IdExtended::from_hex("18FEF200")?;
        let id_b = IdExtended::from_hex("1CFE9201")?;
        let id_c = IdExtended::from_hex("10FF2121")?;
        let id_d = IdExtended::from_hex("0C00290B")?;

        let pgn_bits_a = id_a.pgn_bits();
        let pgn_bits_b = id_b.pgn_bits();
        let pgn_bits_c = id_c.pgn_bits();
        let pgn_bits_d = id_d.pgn_bits();

        assert_eq!(65266, pgn_bits_a);
        assert_eq!(65170, pgn_bits_b);
        assert_eq!(65313, pgn_bits_c);
        assert_eq!(41, pgn_bits_d);

        Ok(())
    }
}
