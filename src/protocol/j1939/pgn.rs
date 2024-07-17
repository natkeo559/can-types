// Copyright (C) 2024  Nathan H. Keough
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # J1939 Parameter Group Number (PGN)
//! 
//! **Description:**
//! The Parameter Group Number (PGN) is a key component in the J1939 protocol that identifies a specific data group
//! within the CAN bus network. Each PGN corresponds to a particular type of message or data set, facilitating structured
//! communication between electronic control units (ECUs) in a vehicle or machine.
//! 
//! - **Function:** PGNs categorize and standardize the types of messages transmitted over the J1939 network, allowing
//!   different devices to understand and process the data correctly.
//! - **Format:** PGNs are 18-bit identifiers, which are part of the 29-bit extended frame format. The structure
//!   of a PGN includes fields such as the priority, data page, and parameter group number itself.
//! - **Usage:** Each PGN represents a different parameter group, such as engine parameters, vehicle diagnostics,
//!   or environmental data. For example, a specific PGN might be used to report engine temperature, while another
//!   could be used for transmission status.
//! 
//! **Examples of PGNs:**
//! - *PGN 61444:* Provides information on the engine temperature.
//! - *PGN 65265:* Transmits data related to the vehicle's diagnostic information.
//! 
//! **Source Documents:**
//! - *SAE J1939-21*
//! - *SAE J1939-71*

if_alloc! {
    use crate::alloc::{string::String, fmt::format};
}

use bitfield_struct::bitfield;

use crate::{conversion::Conversion, identifier::Id, protocol::j1939::identifier::J1939};

use super::address::DestinationAddr;

/// Represents the assignment type of a Protocol Data Unit (PDU).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduAssignment {
    /// Society of Automotive Engineers (SAE) assigned PDU.  
    /// Contains the PDU value.
    Sae(u32),
    /// Manufacturer/proprietary assigned PDU.  
    /// Contains the PDU value.
    Manufacturer(u32),
    /// Unknown or unrecognized PDU assignment.
    /// Contains the PDU value.
    Unknown(u32),
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

/// Bitfield representation of 18-bit Parameter Group Number (PGN).
///
/// ### Repr: `u32`
///
/// | Field        | Size (bits) |
/// |--------------|-------------|
/// | Padding      | 14          |
/// | Reserved     | 1           |
/// | Data Page    | 1           |
/// | PDU Format   | 8           |
/// | PDU Specific | 8           |
#[bitfield(u32, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct Pgn {
    #[bits(14)]
    __: u16,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(1)]
    data_page_bits: bool,
    #[bits(8)]
    pdu_format_bits: u8,
    #[bits(8)]
    pdu_specific_bits: u8,
}

impl Conversion<u32> for Pgn {
    type Error = anyhow::Error;

    /// Creates a new [`Pgn`] bitfield from a 32-bit integer.
    #[inline]
    fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Creates a new [`Pgn`] bitfield from a base-16 (hex) string slice.
    #[inline]
    fn from_hex(hex_str: &str) -> Self {
        let bits = u32::from_str_radix(hex_str, 16).unwrap_or_default();

        Self(bits)
    }

    /// Creates a new [`Pgn`] bitfield from a 32-bit integer.
    /// # Errors
    /// - Never (conversion is trivial)
    #[inline]
    fn try_from_bits(bits: u32) -> Result<Self, Self::Error> {
        if bits > 0x3FFFF {
            return Err(anyhow::anyhow!(
                "PGN bits out of range! Valid range is 0x0000..0xFFFF - got {bits:#04X}"
            ));
        }
        Ok(Self(bits))
    }

    /// Creates a new [`Pgn`] bitfield from a base-16 (hex) string slice.
    /// # Errors
    /// - If failed to parse input hexadecimal string slice.
    /// - If value out of range for valid 18-bit PGNs.
    #[inline]
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let bits = u32::from_str_radix(hex_str, 16).map_err(anyhow::Error::msg)?;
        if bits > 0x3FFFF {
            return Err(anyhow::anyhow!(
                "PGN bits out of range! Valid range is 0x0000..0xFFFF - got {bits:#04X}"
            ));
        }
        Ok(Self(bits))
    }

    /// Creates a new 32-bit integer from the [`Pgn`] bitfield.
    #[inline]
    fn into_bits(self) -> u32 {
        self.0
    }

    /// Creates a new base-16 (hex) `String` from the [`Pgn`] bitfield.
    /// # Requires
    /// - `alloc`
    #[inline]
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        format(format_args!("{:05X}", self.into_bits()))
    }
}

impl Pgn {
    /// Returns the PDU format based on the parsed bits.
    ///
    /// # Returns
    /// - `PduFormat::Pdu1(bits)` if the PDU format value is less than 240.
    /// - `PduFormat::Pdu2(bits)` otherwise.
    #[inline]
    #[must_use]
    pub const fn pdu_format(&self) -> PduFormat {
        match (self.pdu_format_bits() < 240, self.pdu_format_bits()) {
            (true, a) => PduFormat::Pdu1(a),
            (false, b) => PduFormat::Pdu2(b),
        }
    }

    /// Returns the group extension based on the parsed bits.
    ///
    /// # Returns
    /// - `GroupExtension::None` if the PDU format is `Pdu1`.
    /// - `GroupExtension::Some(bits)` if the PDU format is `Pdu2`.
    #[inline]
    #[must_use]
    pub const fn group_extension(&self) -> GroupExtension {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => GroupExtension::None,
            PduFormat::Pdu2(_) => GroupExtension::Some(self.pdu_specific_bits()),
        }
    }

    /// Returns the destination address based on the parsed PDU format.
    ///
    /// # Returns
    /// - `DestinationAddress::Some(bits)` if the PDU format is `Pdu1`.
    /// - `DestinationAddress::None` if the PDU format is `Pdu2`.
    #[inline]
    #[must_use]
    pub const fn destination_address(&self) -> DestinationAddr {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => DestinationAddr::Some(self.pdu_specific_bits()),
            PduFormat::Pdu2(_) => DestinationAddr::None,
        }
    }

    /// Returns the communication mode based on the parsed PDU format.
    ///
    /// # Returns
    /// - `CommunicationMode::P2P` if the PDU format is `Pdu1`.
    /// - `CommunicationMode::Broadcast` if the PDU format is `Pdu2`.
    #[inline]
    #[must_use]
    pub const fn communication_mode(&self) -> CommunicationMode {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => CommunicationMode::P2P,
            PduFormat::Pdu2(_) => CommunicationMode::Broadcast,
        }
    }

    /// Checks if the communication mode is point-to-point (P2P).
    ///
    /// # Returns
    /// - `true` if the communication mode is `P2P`.
    /// - `false` if the communication mode is `Broadcast`.
    #[inline]
    #[must_use]
    pub const fn is_p2p(&self) -> bool {
        match self.communication_mode() {
            CommunicationMode::P2P => true,
            CommunicationMode::Broadcast => false,
        }
    }

    /// Checks if the communication mode is broadcast.
    ///
    /// # Returns
    /// - `true` if the communication mode is `Broadcast`.
    /// - `false` if the communication mode is `P2P`.
    #[inline]
    #[must_use]
    pub const fn is_broadcast(&self) -> bool {
        match self.communication_mode() {
            CommunicationMode::P2P => false,
            CommunicationMode::Broadcast => true,
        }
    }

    /// Determines the PDU assignment based on the parsed bits.
    ///
    /// # Returns
    /// - `PduAssignment::Sae(bits)` for known SAE-defined PDU assignments.
    /// - `PduAssignment::Manufacturer(bits)` for manufacturer-defined PDU assignments.
    /// - `PduAssignment::Unknown(bits)` for unrecognized PDU assignments.
    #[must_use]
    pub fn pdu_assignment(&self) -> PduAssignment {
        match self.into_bits() {
            0x0000_0000..=0x0000_EE00
            | 0x0000_F000..=0x0000_FEFF
            | 0x0001_0000..=0x0001_EE00
            | 0x0001_F000..=0x0001_FEFF => PduAssignment::Sae(self.into_bits()),

            0x0000_EF00 | 0x0000_FF00..=0x0000_FFFF | 0x0001_EF00 | 0x0001_FF00..=0x0001_FFFF => {
                PduAssignment::Manufacturer(self.into_bits())
            }
            p => PduAssignment::Unknown(p),
        }
    }
}

impl Id<J1939> {
    /// Computes the PGN bitfield value based on the 29-bit identifier fields.
    ///
    /// # Returns
    /// The combined PGN bitfield value.
    #[inline]
    #[must_use]
    pub const fn pgn_bits(&self) -> u32 {
        let pgn_bitfield = Pgn::new()
            .with_reserved_bits(self.reserved())
            .with_data_page_bits(self.data_page())
            .with_pdu_format_bits(self.pdu_format())
            .with_pdu_specific_bits(self.pdu_specific());

        pgn_bitfield.0
    }

    /// Constructs and returns a [`Pgn`] struct based on the 29-bit identifier fields.
    ///
    /// # Returns
    /// A [`Pgn`] bitfield initialized with the 29-bit identifier fields.
    #[inline]
    #[must_use]
    pub const fn pgn(&self) -> Pgn {
        Pgn::new()
            .with_reserved_bits(self.reserved())
            .with_data_page_bits(self.data_page())
            .with_pdu_format_bits(self.pdu_format())
            .with_pdu_specific_bits(self.pdu_specific())
    }
}

#[cfg(test)]
mod pgn_tests {
    // use crate::{
    //     conversion::Conversion,
    //     identifier::IdExtended,
    //     pgn::{CommunicationMode, DestinationAddress, GroupExtension, PduAssignment, PduFormat},
    // };

    use super::*;
    use crate::protocol::j1939::address::Addr;

    #[test]
    fn test_pdu_assignment() -> Result<(), anyhow::Error> {
        let id_a = Id::<J1939>::try_from_hex("18FEF200")?;
        let id_b = Id::<J1939>::try_from_hex("1CFE9201")?;
        let id_c = Id::<J1939>::try_from_hex("10FF2121")?;
        let id_d = Id::<J1939>::try_from_hex("0C00290B")?;

        let assignment_a = id_a.pgn().pdu_assignment();
        let assignment_b = id_b.pgn().pdu_assignment();
        let assignment_c = id_c.pgn().pdu_assignment();
        let assignment_d = id_d.pgn().pdu_assignment();

        assert_eq!(PduAssignment::Sae(65266), assignment_a);
        assert_eq!(PduAssignment::Sae(65170), assignment_b);
        assert_eq!(PduAssignment::Manufacturer(65313), assignment_c);
        assert_eq!(PduAssignment::Sae(41), assignment_d);

        Ok(())
    }

    #[test]
    fn test_communication_mode() -> Result<(), anyhow::Error> {
        let id_a = Id::<J1939>::try_from_hex("18FEF200")?;
        let id_b = Id::<J1939>::try_from_hex("1CFE9201")?;
        let id_c = Id::<J1939>::try_from_hex("10FF2121")?;
        let id_d = Id::<J1939>::try_from_hex("0C00290B")?;

        let comms_mode_a = id_a.pgn().communication_mode();
        let comms_mode_b = id_b.pgn().communication_mode();
        let comms_mode_c = id_c.pgn().communication_mode();
        let comms_mode_d = id_d.pgn().communication_mode();

        assert_eq!(CommunicationMode::Broadcast, comms_mode_a);
        assert_eq!(CommunicationMode::Broadcast, comms_mode_b);
        assert_eq!(CommunicationMode::Broadcast, comms_mode_c);
        assert_eq!(CommunicationMode::P2P, comms_mode_d);

        Ok(())
    }

    #[test]
    fn test_destination_address() -> Result<(), anyhow::Error> {
        let id_a = Id::<J1939>::try_from_hex("18FEF200")?;
        let id_b = Id::<J1939>::try_from_hex("1CFE9201")?;
        let id_c = Id::<J1939>::try_from_hex("10FF2121")?;
        let id_d = Id::<J1939>::try_from_hex("0C00290B")?;

        let dest_addr_a = id_a.pgn().destination_address();
        let dest_addr_b = id_b.pgn().destination_address();
        let dest_addr_c = id_c.pgn().destination_address();
        let dest_addr_d = id_d.pgn().destination_address();

        assert_eq!(DestinationAddr::None, dest_addr_a);
        assert_eq!(DestinationAddr::None, dest_addr_b);
        assert_eq!(DestinationAddr::None, dest_addr_c);
        assert_eq!(DestinationAddr::Some(41), dest_addr_d);
        assert_eq!(Some(Addr::RetarderExhaustEngine1), dest_addr_d.lookup());

        Ok(())
    }

    #[test]
    fn test_group_extension() -> Result<(), anyhow::Error> {
        let id_a = Id::<J1939>::try_from_hex("18FEF200")?;
        let id_b = Id::<J1939>::try_from_hex("1CFE9201")?;
        let id_c = Id::<J1939>::try_from_hex("10FF2121")?;
        let id_d = Id::<J1939>::try_from_hex("0C00290B")?;

        let group_ext_a = id_a.pgn().group_extension();
        let group_ext_b = id_b.pgn().group_extension();
        let group_ext_c = id_c.pgn().group_extension();
        let group_ext_d = id_d.pgn().group_extension();

        assert_eq!(GroupExtension::Some(242), group_ext_a);
        assert_eq!(GroupExtension::Some(146), group_ext_b);
        assert_eq!(GroupExtension::Some(33), group_ext_c);
        assert_eq!(GroupExtension::None, group_ext_d);

        Ok(())
    }

    #[test]
    fn test_pdu_format() -> Result<(), anyhow::Error> {
        let id_a = Id::<J1939>::try_from_hex("18FEF200")?;
        let id_b = Id::<J1939>::try_from_hex("1CFE9201")?;
        let id_c = Id::<J1939>::try_from_hex("10FF2121")?;
        let id_d = Id::<J1939>::try_from_hex("0C00290B")?;

        let pdu_format_a = id_a.pgn().pdu_format();
        let pdu_format_b = id_b.pgn().pdu_format();
        let pdu_format_c = id_c.pgn().pdu_format();
        let pdu_format_d = id_d.pgn().pdu_format();

        assert_eq!(PduFormat::Pdu2(254), pdu_format_a);
        assert_eq!(PduFormat::Pdu2(254), pdu_format_b);
        assert_eq!(PduFormat::Pdu2(255), pdu_format_c);
        assert_eq!(PduFormat::Pdu1(0), pdu_format_d);

        Ok(())
    }

    #[test]
    fn test_pgn_bits() -> Result<(), anyhow::Error> {
        let id_a = Id::<J1939>::try_from_hex("18FEF200")?;
        let id_b = Id::<J1939>::try_from_hex("1CFE9201")?;
        let id_c = Id::<J1939>::try_from_hex("10FF2121")?;
        let id_d = Id::<J1939>::try_from_hex("0C00290B")?;

        let pgn_bits_a = id_a.pgn();
        let pgn_bits_b = id_b.pgn();
        let pgn_bits_c = id_c.pgn();
        let pgn_bits_d = id_d.pgn();

        assert_eq!(65266, pgn_bits_a.into_bits());
        assert_eq!(65170, pgn_bits_b.into_bits());
        assert_eq!(65313, pgn_bits_c.into_bits());
        assert_eq!(41, pgn_bits_d.into_bits());

        Ok(())
    }
}
