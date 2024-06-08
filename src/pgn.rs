use bitfield_struct::bitfield;

use crate::IdExtended;

/// Represents the assignment type of a Protocol Data Unit (PDU).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduAssignment {
    /// Society of Automotive Engineers (SAE) assigned PDU.  
    /// Contains the PDU value.
    Sae(u32),
    /// Manufacturer/proprietary assigned PDU.  
    /// Contains the PDU value.
    Manufacturer(u32),
}

/// Represents the format of a Protocol Data Unit (PDU).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduFormat {
    /// PDU format 1.  
    /// Contains PDU format value.
    Pdu1(u8),
    /// PDU format 2.  
    /// Contains PDU format value.
    Pdu2(u8),
}

/// Represents the communication mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommunicationMode {
    /// Point-to-Point communication mode.  
    /// This PDU communication variant may contain a destination address.
    P2P,
    /// Broadcast communication mode.  
    Broadcast,
}

/// Represents the group extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupExtension {
    /// No group extension.
    None,
    /// Group extension with a specific value.
    Some(u8),
}

/// Represents the destination address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationAddress {
    /// No destination address.
    None,
    /// Destination address with a specific value.
    Some(u8),
}

/// Represents the bit layout of a Parameter Group Number (PGN) within a Controller Area Network
/// (CAN) message or a J1939 message.
///
/// This struct provides a structured representation of the bits composing a PGN, including
/// reserved bits, data page bits,
/// PDU format bits, and PDU specific bits.
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
    /// Returns the PGN bits representation as a u32.
    #[must_use]
    pub const fn pgn_bits(&self) -> u32 {
        let pgn_bitfield = PgnBits::new()
            .with_reserved_bits(self.reserved_bits())
            .with_data_page_bits(self.data_page_bits())
            .with_pdu_format_bits(self.pdu_format_bits())
            .with_pdu_specific_bits(self.pdu_specific_bits());

        pgn_bitfield.into_bits()
    }

    /// Returns the PGN bits representation as a bitfield struct.
    #[must_use]
    pub const fn pgn(&self) -> PgnBits {
        PgnBits::new()
            .with_reserved_bits(self.reserved_bits())
            .with_data_page_bits(self.data_page_bits())
            .with_pdu_format_bits(self.pdu_format_bits())
            .with_pdu_specific_bits(self.pdu_specific_bits())
    }

    /// Returns the PDU format.
    #[must_use]
    pub const fn pdu_format(&self) -> PduFormat {
        match (self.pdu_format_bits() < 240, self.pdu_format_bits()) {
            (true, a) => PduFormat::Pdu1(a),
            (false, b) => PduFormat::Pdu2(b),
        }
    }

    /// Returns the group extension.
    #[must_use]
    pub const fn group_extension(&self) -> GroupExtension {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => GroupExtension::None,
            PduFormat::Pdu2(_) => GroupExtension::Some(self.pdu_specific_bits()),
        }
    }

    /// Returns the destination address.
    #[must_use]
    pub const fn destination_address(&self) -> DestinationAddress {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => DestinationAddress::Some(self.pdu_specific_bits()),
            PduFormat::Pdu2(_) => DestinationAddress::None,
        }
    }

    /// Returns the communication mode.
    #[must_use]
    pub const fn communication_mode(&self) -> CommunicationMode {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => CommunicationMode::P2P,
            PduFormat::Pdu2(_) => CommunicationMode::Broadcast,
        }
    }

    /// Checks if the communication mode is Point-to-Point (P2P).
    #[must_use]
    pub const fn is_p2p(&self) -> bool {
        match self.communication_mode() {
            CommunicationMode::P2P => true,
            CommunicationMode::Broadcast => false,
        }
    }

    /// Checks if the communication mode is Broadcast.
    #[must_use]
    pub const fn is_broadcast(&self) -> bool {
        match self.communication_mode() {
            CommunicationMode::P2P => false,
            CommunicationMode::Broadcast => true,
        }
    }

    /// Get the PDU assignment (SAE or Manufacturer).
    /// Returns the assignment with the `u32` PGN value.
    /// # Note
    /// There are gaps between PGN bit ranges which aren't assignable.
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
            _ => Err(anyhow::anyhow!("PGN not within a known valid range!")),
        }
    }
}

#[cfg(test)]
mod pgn_tests {
    use crate::{
        CommunicationMode, DestinationAddress, GroupExtension, IdExtended, PduAssignment, PduFormat,
    };

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
