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

use crate::prelude::{
    Conversion, Data, Extended, Id, IdExtended, IdKind, Name, Pdu, PduData, PduKind, PduName,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message<I: IdKind, P: PduKind> {
    id: Id<I>,
    pdu: Pdu<P>,
}

impl Message<Extended, Data> {
    /// Constructs a new [`Message`] from its parts: an identifier and pdu.
    ///
    /// # Arguments
    /// - `id`: An [`IdExtended`] representing the 29-bit identifier of the message.
    /// - `pdu`: A [`PduData`] containing the payload or content of the message.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the provided parts.
    #[must_use]
    pub fn from_parts(id: IdExtended, pdu: PduData) -> Self {
        Self { id, pdu }
    }

    /// Destructures the [`Message`] into its parts: an identifier and pdu.
    ///
    /// # Returns
    /// A tuple containing:
    /// - An [`IdExtended`] representing the 29-bit identifier.
    /// - A [`PduData`] containing the payload or content of the message.
    #[must_use]
    pub fn into_parts(self) -> (Id<Extended>, PduData) {
        (self.id, self.pdu)
    }

    /// Constructs a new [`Message`] from raw bit representations of its components.
    /// # Errors
    /// - If failed to construct the identifier field from bits
    /// - If failed to construct the pdu field from bits
    pub fn try_from_bits(hex_id: u32, hex_pdu: u64) -> Result<Self, anyhow::Error> {
        let id = IdExtended::try_from_bits(hex_id)?;
        let pdu = PduData::try_from_bits(hex_pdu)?;

        Ok(Self { id, pdu })
    }

    /// Constructs a new [`Message`] from hexadecimal string representations of its components.
    /// # Errors
    /// - If failed to construct the identifier field from hex
    /// - If failed to construct the pdu field from hex
    pub fn try_from_hex(hex_id: &str, hex_pdu: &str) -> Result<Self, anyhow::Error> {
        let id = IdExtended::try_from_hex(hex_id)?;
        let pdu = PduData::try_from_hex(hex_pdu)?;

        Ok(Self { id, pdu })
    }

    /// Constructs a new [`Message`] from raw bit representations of its components.
    ///
    /// # Arguments
    /// - `hex_id`: A `u32` representing the hexadecimal encoded 29-bit identifier.
    /// - `hex_pdu`: A `u64` representing the hexadecimal encoded pdu.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the decoded components.
    #[must_use]
    pub fn from_bits(hex_id: u32, hex_pdu: u64) -> Self {
        let id = IdExtended::from_bits(hex_id);
        let pdu = PduData::from_bits(hex_pdu);

        Self { id, pdu }
    }

    /// Constructs a new [`Message`] from hexadecimal string representations of its components.
    ///
    /// # Arguments
    /// - `hex_id`: A `&str` representing the hexadecimal encoded 29-bit identifier.
    /// - `hex_pdu`: A `&str` representing the hexadecimal encoded pdu.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the decoded components.
    #[must_use]
    pub fn from_hex(hex_id: &str, hex_pdu: &str) -> Self {
        let id = IdExtended::from_hex(hex_id);
        let pdu = PduData::from_hex(hex_pdu);

        Self { id, pdu }
    }

    /// Retrieves the 29-bit identifier from the message.
    ///
    /// # Returns
    /// The [`IdExtended`] bitfield associated with the message.
    #[must_use]
    pub fn id(&self) -> IdExtended {
        self.id
    }

    /// Retrieves the pdu from the message.
    ///
    /// # Returns
    /// The [`PduName`] bitfield associated with the message.
    #[must_use]
    pub fn pdu(&self) -> PduData {
        self.pdu
    }
}

impl Message<Extended, Name> {
    /// Constructs a new [`Message`] from its parts: an identifier and pdu.
    ///
    /// # Arguments
    /// - `id`: An [`IdExtended`] representing the 29-bit identifier of the message.
    /// - `pdu`: A [`PduName`] containing the payload or content of the message.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the provided parts.
    #[must_use]
    pub fn from_parts(id: IdExtended, pdu: PduName) -> Self {
        Self { id, pdu }
    }

    /// Destructures the [`Message`] into its parts: an identifier and pdu.
    ///
    /// # Returns
    /// A tuple containing:
    /// - An [`IdExtended`] representing the 29-bit identifier.
    /// - A [`PduName`] containing the payload or content of the message.
    #[must_use]
    pub fn into_parts(self) -> (Id<Extended>, PduName) {
        (self.id, self.pdu)
    }

    /// Constructs a new [`Message`] from raw bit representations of its components.
    /// # Errors
    /// - If failed to construct the identifier field from bits
    /// - If failed to construct the pdu field from bits
    pub fn try_from_bits(hex_id: u32, hex_pdu: u64) -> Result<Self, anyhow::Error> {
        let id = IdExtended::try_from_bits(hex_id)?;
        let pdu = PduName::try_from_bits(hex_pdu)?;

        Ok(Self { id, pdu })
    }

    /// Constructs a new [`Message`] from hexadecimal string representations of its components.
    /// # Errors
    /// - If failed to construct the identifier field from hex
    /// - If failed to construct the pdu field from hex
    pub fn try_from_hex(hex_id: &str, hex_pdu: &str) -> Result<Self, anyhow::Error> {
        let id = IdExtended::try_from_hex(hex_id)?;
        let pdu = PduName::try_from_hex(hex_pdu)?;

        Ok(Self { id, pdu })
    }

    /// Constructs a new [`Message`] from raw bit representations of its components.
    ///
    /// # Arguments
    /// - `hex_id`: A `u32` representing the hexadecimal encoded 29-bit identifier.
    /// - `hex_pdu`: A `u64` representing the hexadecimal encoded pdu.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the decoded components.
    #[must_use]
    pub fn from_bits(hex_id: u32, hex_pdu: u64) -> Self {
        let id = IdExtended::from_bits(hex_id);
        let pdu = PduName::from_bits(hex_pdu);

        Self { id, pdu }
    }

    /// Constructs a new [`Message`] from hexadecimal string representations of its components.
    ///
    /// # Arguments
    /// - `hex_id`: A `&str` representing the hexadecimal encoded 29-bit identifier.
    /// - `hex_pdu`: A `&str` representing the hexadecimal encoded pdu.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the decoded components.
    #[must_use]
    pub fn from_hex(hex_id: &str, hex_pdu: &str) -> Self {
        let id = IdExtended::from_hex(hex_id);
        let pdu = PduName::from_hex(hex_pdu);

        Self { id, pdu }
    }

    /// Retrieves the 29-bit identifier from the message.
    ///
    /// # Returns
    /// The [`IdExtended`] bitfield associated with the message.
    #[must_use]
    pub fn id(&self) -> IdExtended {
        self.id
    }

    /// Retrieves the pdu from the message.
    ///
    /// # Returns
    /// The [`PduName`] bitfield associated with the message.
    #[must_use]
    pub fn pdu(&self) -> PduName {
        self.pdu
    }
}

impl From<Message<Extended, Data>> for Message<Extended, Name> {
    fn from(value: Message<Extended, Data>) -> Self {
        Self {
            id: value.id(),
            pdu: value.pdu().into(),
        }
    }
}

impl From<Message<Extended, Name>> for Message<Extended, Data> {
    fn from(value: Message<Extended, Name>) -> Self {
        Self {
            id: value.id(),
            pdu: value.pdu().into(),
        }
    }
}
