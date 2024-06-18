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

use crate::prelude::{Conversion, Data, Extended, Id, IdExtended, IdKind, Pdu, PduData, PduKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message<I: IdKind, P: PduKind> {
    id: Id<I>,
    data: Pdu<P>,
}

impl Message<Extended, Data> {
    /// Constructs a new `Message` from its parts: an extended identifier and data.
    ///
    /// # Arguments
    /// - `id`: An `IdExtended` representing the extended identifier of the message.
    /// - `data`: A `Data` containing the payload or content of the message.
    ///
    /// # Returns
    /// A new `Message` instance initialized with the provided parts.
    #[must_use]
    pub fn from_parts(id: IdExtended, data: PduData) -> Self {
        Self { id, data }
    }

    /// Destructures the `Message` into its parts: an extended identifier and data.
    ///
    /// # Returns
    /// A tuple containing:
    /// - An `Id<Extended>` representing the extended identifier.
    /// - A `Data` containing the payload or content of the message.
    #[must_use]
    pub fn into_parts(self) -> (Id<Extended>, PduData) {
        (self.id, self.data)
    }

    /// # Errors
    /// - If failed to construct the identifier field from bits
    /// - If failed to construct the data field from bits
    pub fn try_from_bits(hex_id: u32, hex_data: u64) -> Result<Self, anyhow::Error> {
        let id: Id<Extended> = IdExtended::try_from_bits(hex_id)?;
        let data: Pdu<Data> = PduData::try_from_bits(hex_data)?;

        Ok(Self { id, data })
    }

    /// # Errors
    /// - If failed to construct the identifier field from hex
    /// - If failed to construct the data field from hex
    pub fn try_from_hex(hex_id: &str, hex_data: &str) -> Result<Self, anyhow::Error> {
        let id: Id<Extended> = IdExtended::try_from_hex(hex_id)?;
        let data: Pdu<Data> = PduData::try_from_hex(hex_data)?;

        Ok(Self { id, data })
    }

    /// Constructs a new `Message` from raw bit representations of its components.
    ///
    /// # Arguments
    /// - `hex_id`: A `u32` representing the hexadecimal encoded extended identifier.
    /// - `hex_data`: A `u64` representing the hexadecimal encoded data payload.
    ///
    /// # Returns
    /// A new `Message` instance initialized with the decoded components.
    #[must_use]
    pub fn from_bits(hex_id: u32, hex_data: u64) -> Self {
        let id: Id<Extended> = IdExtended::from_bits(hex_id);
        let data: Pdu<Data> = PduData::from_bits(hex_data);

        Self { id, data }
    }

    /// Constructs a new `Message` from hexadecimal string representations of its components.
    ///
    /// # Arguments
    /// - `hex_id`: A `&str` representing the hexadecimal encoded extended identifier.
    /// - `hex_data`: A `&str` representing the hexadecimal encoded data payload.
    ///
    /// # Returns
    /// A new `Message` instance initialized with the decoded components.
    #[must_use]
    pub fn from_hex(hex_id: &str, hex_data: &str) -> Self {
        let id: Id<Extended> = IdExtended::from_hex(hex_id);
        let data: Pdu<Data> = PduData::from_hex(hex_data);

        Self { id, data }
    }

    /// Retrieves the extended identifier from the message.
    ///
    /// # Returns
    /// The `IdExtended` bitfield associated with the message.
    #[must_use]
    pub fn id(&self) -> IdExtended {
        self.id
    }

    /// Retrieves the data payload from the message.
    ///
    /// # Returns
    /// The `Data` bitfield associated with the message.
    #[must_use]
    pub fn data(&self) -> PduData {
        self.data
    }
}
