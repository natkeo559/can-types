use crate::prelude::{Conversion, Data, Extended, Id, IdExtended, IdKind};

pub struct Message<I: IdKind> {
    id: Id<I>,
    data: Data,
}

impl Message<Extended> {
    /// Constructs a new `Message` from its parts: an extended identifier and data.
    ///
    /// # Arguments
    /// - `id`: An `IdExtended` representing the extended identifier of the message.
    /// - `data`: A `Data` containing the payload or content of the message.
    ///
    /// # Returns
    /// A new `Message` instance initialized with the provided parts.
    #[must_use]
    pub fn from_parts(id: IdExtended, data: Data) -> Self {
        Self { id, data }
    }

    /// Destructures the `Message` into its parts: an extended identifier and data.
    ///
    /// # Returns
    /// A tuple containing:
    /// - An `Id<Extended>` representing the extended identifier.
    /// - A `Data` containing the payload or content of the message.
    #[must_use]
    pub fn into_parts(self) -> (Id<Extended>, Data) {
        (self.id, self.data)
    }

    /// # Errors
    /// - If failed to construct the identifier field from bits
    /// - If failed to construct the data field from bits
    pub fn try_from_bits(hex_id: u32, hex_data: u64) -> Result<Self, anyhow::Error> {
        let id: Id<Extended> = IdExtended::try_from_bits(hex_id)?;
        let data: Data = Data::try_from_bits(hex_data)?;

        Ok(Self { id, data })
    }

    /// # Errors
    /// - If failed to construct the identifier field from hex
    /// - If failed to construct the data field from hex
    pub fn try_from_hex(hex_id: &str, hex_data: &str) -> Result<Self, anyhow::Error> {
        let id: Id<Extended> = IdExtended::try_from_hex(hex_id)?;
        let data: Data = Data::try_from_hex(hex_data)?;

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
        let data: Data = Data::from_bits(hex_data);

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
        let data: Data = Data::from_hex(hex_data);

        Self { id, data }
    }

    /// Retrieves the extended identifier from the message.
    ///
    /// # Returns
    /// The `IdExtended` associated with the message.
    #[must_use]
    pub fn id(&self) -> IdExtended {
        self.id
    }

    /// Retrieves the data payload from the message.
    ///
    /// # Returns
    /// The `Data` payload associated with the message.
    #[must_use]
    pub fn data(&self) -> Data {
        self.data
    }
}
