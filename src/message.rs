// Copyright (c) 2024 Nathan H. Keough
//
// This work is dual-licensed under MIT OR Apache 2.0 (or any later version).
// You may choose between one of them if you use this work.
//
// For further detail, please refer to the individual licenses located at the root of this crate.

//! Defines a CAN message (Identifier with associated PDU).

use crate::{
    conversion::Conversion,
    identifier::{Id, IsProtocol},
    payload::{Data, IsDataUnit, Name, Pdu},
    protocol::j1939::identifier::J1939,
};

/// Represents a CAN message with its associated identifier ([`Id`]) and protocol data unit ([`Pdu`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message<P: IsProtocol, U: IsDataUnit> {
    id: Id<P>,
    pdu: Pdu<U>,
}

impl Message<J1939, Data> {
    /// Constructs a new Message from its parts: a 29-bit J1939 identifier and pdu containing 64 bits of generic data.
    ///
    /// # Arguments
    /// - `id`: An [`Id<J1939>`] representing the 29-bit identifier of the message.
    /// - `pdu`: A [`Pdu<Data>`] containing the payload or content of the message.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the provided parts.
    #[inline]
    #[must_use]
    pub fn from_parts(id: Id<J1939>, pdu: Pdu<Data>) -> Self {
        Self { id, pdu }
    }

    /// Destructures the [`Message`] into its parts: a 29-bit J1939 identifier and pdu containing 64 bits of generic data.
    ///
    /// # Returns
    /// A tuple containing:
    /// - An [`Id<J1939>`] representing the 29-bit identifier.
    /// - A [`Pdu<Data>`] containing the payload or content of the message.
    #[inline]
    #[must_use]
    pub fn into_parts(self) -> (Id<J1939>, Pdu<Data>) {
        (self.id, self.pdu)
    }

    /// Constructs a new [`Message`] from raw bit representations of its components.
    /// # Errors
    /// - If failed to construct the identifier field from bits
    /// - If failed to construct the pdu field from bits
    #[inline]
    pub fn try_from_bits(hex_id: u32, hex_pdu: u64) -> Result<Self, anyhow::Error> {
        let id = Id::<J1939>::from_bits(hex_id);
        let pdu = Pdu::<Data>::try_from_bits(hex_pdu)?;

        Ok(Self { id, pdu })
    }

    /// Constructs a new [`Message`] from hexadecimal string representations of its components.
    /// # Errors
    /// - If failed to construct the identifier field from hex
    /// - If failed to construct the pdu field from hex
    #[inline]
    pub fn try_from_hex(hex_id: &str, hex_pdu: &str) -> Result<Self, anyhow::Error> {
        let id = Id::<J1939>::try_from_hex(hex_id)?;
        let pdu = Pdu::<Data>::try_from_hex(hex_pdu)?;

        Ok(Self { id, pdu })
    }

    /// Constructs a new [`Message`] from raw bit representations of its components.
    ///
    /// # Arguments
    /// - `hex_id`: A `u32` representing the hexadecimal encoded 29-bit J1939 identifier.
    /// - `hex_pdu`: A `u64` representing the hexadecimal encoded pdu.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the decoded components.
    #[inline]
    #[must_use]
    pub fn from_bits(hex_id: u32, hex_pdu: u64) -> Self {
        let id = Id::<J1939>::from_bits(hex_id);
        let pdu = Pdu::<Data>::from_bits(hex_pdu);

        Self { id, pdu }
    }

    /// Constructs a new [`Message`] from hexadecimal string representations of its components.
    ///
    /// # Arguments
    /// - `hex_id`: A `&str` representing the hexadecimal encoded 29-bit J1939 identifier.
    /// - `hex_pdu`: A `&str` representing the hexadecimal encoded pdu.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the decoded components.
    #[inline]
    #[must_use]
    pub fn from_hex(hex_id: &str, hex_pdu: &str) -> Self {
        let id = Id::<J1939>::from_hex(hex_id);
        let pdu = Pdu::<Data>::from_hex(hex_pdu);

        Self { id, pdu }
    }

    /// Retrieves the 29-bit J1939 identifier from the message.
    ///
    /// # Returns
    /// The [`Id<J1939>`] bitfield associated with the message.
    #[inline]
    #[must_use]
    pub fn id(&self) -> Id<J1939> {
        self.id
    }

    /// Retrieves the pdu from the message.
    ///
    /// # Returns
    /// The [`Pdu<Data>`] bitfield associated with the message.
    #[inline]
    #[must_use]
    pub fn pdu(&self) -> Pdu<Data> {
        self.pdu
    }
}

impl Message<J1939, Name> {
    /// Constructs a new [`Message`] from its parts: an identifier and pdu.
    ///
    /// # Arguments
    /// - `id`: An [`Id<J1939>`] representing the 29-bit identifier of the message.
    /// - `pdu`: A [`Pdu<Data>`] containing the payload or content of the message.
    ///
    /// # Returns
    /// A new [`Message`] instance initialized with the provided parts.
    #[inline]
    #[must_use]
    pub fn from_parts(id: Id<J1939>, pdu: Pdu<Name>) -> Self {
        Self { id, pdu }
    }

    /// Destructures the [`Message`] into its parts: an identifier and pdu.
    ///
    /// # Returns
    /// A tuple containing:
    /// - An [`Id<J1939>`] representing the 29-bit identifier.
    /// - A [`Pdu<Data>`] containing the payload or content of the message.
    #[inline]
    #[must_use]
    pub fn into_parts(self) -> (Id<J1939>, Pdu<Name>) {
        (self.id, self.pdu)
    }

    /// Constructs a new [`Message`] from raw bit representations of its components.
    /// # Errors
    /// - If failed to construct the identifier field from bits
    /// - If failed to construct the pdu field from bits
    #[inline]
    pub fn try_from_bits(hex_id: u32, hex_pdu: u64) -> Result<Self, anyhow::Error> {
        let id = Id::<J1939>::try_from_bits(hex_id)?;
        let pdu = Pdu::<Name>::try_from_bits(hex_pdu)?;

        Ok(Self { id, pdu })
    }

    /// Constructs a new [`Message`] from hexadecimal string representations of its components.
    /// # Errors
    /// - If failed to construct the identifier field from hex
    /// - If failed to construct the pdu field from hex
    #[inline]
    pub fn try_from_hex(hex_id: &str, hex_pdu: &str) -> Result<Self, anyhow::Error> {
        let id = Id::<J1939>::try_from_hex(hex_id)?;
        let pdu = Pdu::<Name>::try_from_hex(hex_pdu)?;

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
    #[inline]
    #[must_use]
    pub fn from_bits(hex_id: u32, hex_pdu: u64) -> Self {
        let id = Id::<J1939>::from_bits(hex_id);
        let pdu = Pdu::<Name>::from_bits(hex_pdu);

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
    #[inline]
    #[must_use]
    pub fn from_hex(hex_id: &str, hex_pdu: &str) -> Self {
        let id = Id::<J1939>::from_hex(hex_id);
        let pdu = Pdu::<Name>::from_hex(hex_pdu);

        Self { id, pdu }
    }

    /// Retrieves the 29-bit identifier from the message.
    ///
    /// # Returns
    /// The [`Id<J1939>`] bitfield associated with the message.
    #[inline]
    #[must_use]
    pub fn id(&self) -> Id<J1939> {
        self.id
    }

    /// Retrieves the pdu from the message.
    ///
    /// # Returns
    /// The [`Pdu<Data>`] bitfield associated with the message.
    #[inline]
    #[must_use]
    pub fn pdu(&self) -> Pdu<Name> {
        self.pdu
    }
}

impl From<Message<J1939, Data>> for Message<J1939, Name> {
    fn from(value: Message<J1939, Data>) -> Self {
        Self {
            id: value.id(),
            pdu: value.pdu().into(),
        }
    }
}

impl From<Message<J1939, Name>> for Message<J1939, Data> {
    fn from(value: Message<J1939, Name>) -> Self {
        Self {
            id: value.id(),
            pdu: value.pdu().into(),
        }
    }
}
