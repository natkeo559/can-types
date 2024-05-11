use bitfield_struct::bitfield;

use crate::{Extended, Id};

pub enum PduAssignment {
    SAE(u32),
    MANUFACTURER(u32),
}

pub enum PduFormat {
    PDU1(u8),
    PDU2(u8),
}

pub enum CommunicationMode {
    P2P,
    Broadcast,
}

pub enum GroupExtension {
    None,
    Some(u8),
}
pub enum DestinationAddress {
    None,
    Some(u8),
}

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

impl Id<Extended> {
    #[must_use]
    pub const fn pgn_bits(&self) -> u32 {
        let pgn_bitfield = PgnBits::new()
            .with_reserved_bits(self.reserved_bits())
            .with_data_page_bits(self.data_page_bits())
            .with_pdu_format_bits(self.pdu_format_bits())
            .with_pdu_specific_bits(self.pdu_specific_bits());

        pgn_bitfield.0
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
            (true, a) => PduFormat::PDU1(a),
            (false, b) => PduFormat::PDU2(b),
        }
    }

    #[must_use]
    pub const fn group_extension(&self) -> GroupExtension {
        match self.pdu_format() {
            PduFormat::PDU1(_) => GroupExtension::None,
            PduFormat::PDU2(_padding_bits) => GroupExtension::Some(self.pdu_specific_bits()),
        }
    }

    #[must_use]
    pub const fn destination_address(&self) -> DestinationAddress {
        match self.pdu_format() {
            PduFormat::PDU1(_) => DestinationAddress::None,
            PduFormat::PDU2(_) => DestinationAddress::Some(self.pdu_specific_bits()),
        }
    }

    #[must_use]
    pub const fn communication_mode(&self) -> CommunicationMode {
        match self.pdu_format() {
            PduFormat::PDU1(_) => CommunicationMode::P2P,
            PduFormat::PDU2(_) => CommunicationMode::Broadcast,
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
}
