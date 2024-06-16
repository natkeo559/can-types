use crate::prelude::{Conversion, Data, Extended, Id, IdExtended, IdKind};

pub struct Message<I: IdKind> {
    id: Id<I>,
    data: Data,
}

impl Message<Extended> {
    #[must_use]
    pub fn from_parts(id: IdExtended, data: Data) -> Self {
        Self { id, data }
    }

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

    #[must_use]
    pub fn from_bits(hex_id: u32, hex_data: u64) -> Self {
        let id: Id<Extended> = IdExtended::from_bits(hex_id);
        let data: Data = Data::from_bits(hex_data);

        Self { id, data }
    }

    #[must_use]
    pub fn from_hex(hex_id: &str, hex_data: &str) -> Self {
        let id: Id<Extended> = IdExtended::from_hex(hex_id);
        let data: Data = Data::from_hex(hex_data);

        Self { id, data }
    }

    #[must_use]
    pub fn id(&self) -> IdExtended {
        self.id
    }

    #[must_use]
    pub fn data(&self) -> Data {
        self.data
    }
}
