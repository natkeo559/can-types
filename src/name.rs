if_alloc! {
    use crate::prelude::String;
}
use bitfield_struct::bitfield;

use crate::conversion::Conversion;

/// Represents a Name in the SAE J1939 protocol.
///
/// The Name structure is used in the SAE J1939 protocol to represent the identity of a device or
/// component within a vehicle's network.
///
/// ### Repr: `u64`
/// | Field                             | Size (bits) |
/// |-----------------------------------|-------------|
/// | Arbitrary address bits            | 1           |
/// | Industry group bits               | 3           |
/// | Vehicle system instance bits      | 4           |
/// | Vehicle system bits               | 7           |
/// | Reserved bits                     | 1           |
/// | Function bits                     | 8           |
/// | Function instance bits            | 5           |
/// | ECU instance bits                 | 3           |
/// | Manufacturer code bits            | 11          |
/// | Identity number bits              | 21          |
#[bitfield(u64, order = Msb)]
pub struct Name {
    #[bits(1)]
    arbitrary_address_bits: bool,
    #[bits(3)]
    industry_group_bits: u8,
    #[bits(4)]
    vehicle_system_instance_bits: u8,
    #[bits(7)]
    vehicle_system_bits: u8,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(8)]
    function_bits: u8,
    #[bits(5)]
    function_instance_bits: u8,
    #[bits(3)]
    ecu_instance_bits: u8,
    #[bits(11)]
    manufacturer_code_bits: u16,
    #[bits(21)]
    identity_number_bits: u32,
}

impl Conversion<u64> for Name {
    type Error = anyhow::Error;

    /// Creates a new 64-bit integer from the `Name` bitfield.
    /// # Errors
    /// - Never (conversion is trivial)
    fn try_into_bits(self) -> Result<u64, Self::Error> {
        Ok(self.into_bits())
    }

    /// Creates a new base-16 (hex) `String` from the `Name` bitfield.
    /// # Errors
    /// - If invalid encoding of provided Base16 string
    /// - If insufficient output buffer length
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn try_into_hex(self) -> Result<String, Self::Error> {
        let mut buffer: [u8; 8] = [b'0'; 8];
        let hex_bytes: &[u8] =
            base16ct::upper::encode(&self.into_bits().to_be_bytes(), &mut buffer)
                .map_err(anyhow::Error::msg)?;
        String::from_utf8(hex_bytes.to_vec()).map_err(anyhow::Error::msg)
    }

    /// Creates a new `Name` bitfield from a 64-bit integer.
    /// # Errors
    /// - Never (conversion is trivial)
    fn try_from_bits(bits: u64) -> Result<Self, Self::Error> {
        Ok(Self(bits))
    }

    /// Creates a new `Name` bitfield from a base-16 (hex) string slice.
    /// # Errors
    /// - If invalid encoding of provided Base16 string
    /// - If insufficient output buffer length
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let mut buffer: [u8; 8] = [b'0'; 8];
        base16ct::upper::decode(hex_str, &mut buffer).map_err(anyhow::Error::msg)?;
        let bits: u64 = u64::from_be_bytes(buffer);

        Ok(Self(bits))
    }

    /// Creates a new 64-bit integer from the `Name` bitfield.
    fn into_bits(self) -> u64 {
        self.into_bits()
    }

    /// Creates a new base-16 (hex) `String` from the `Name` bitfield.
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String {
        let mut buffer: [u8; 8] = [b'0'; 8];
        let hex_bytes: &[u8] =
            base16ct::upper::encode(&self.into_bits().to_be_bytes(), &mut buffer)
                .unwrap_or_default();
        String::from_utf8(hex_bytes.to_vec()).unwrap_or_default()
    }

    /// Creates a new `Name` bitfield from a 64-bit integer.
    fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Creates a new `Name` bitfield from a base-16 (hex) string slice.
    fn from_hex(hex_str: &str) -> Self {
        let mut buffer: [u8; 8] = [b'0'; 8];
        base16ct::upper::decode(hex_str, &mut buffer).unwrap_or_default();
        let bits: u64 = u64::from_be_bytes(buffer);

        Self(bits)
    }
}

impl Name {
    /// Indicates whether or not the ECU/CA can negotiate an address (true = yes; false = no).
    #[must_use]
    pub const fn arbitrary_address(&self) -> bool {
        self.arbitrary_address_bits()
    }

    /// These codes are associated with particular industries such as on-highway equipment,
    /// agricultural equipment, and more.
    #[must_use]
    pub const fn industry_group(&self) -> u8 {
        self.industry_group_bits()
    }

    /// Assigns a number to each instance on the Vehicle System (in case you connect several
    /// networks – e.g. connecting cars on a train).
    #[must_use]
    pub const fn vehicle_system_instance(&self) -> u8 {
        self.vehicle_system_instance_bits()
    }

    /// Vehicle systems are associated with the Industry Group and they can be, for instance,
    /// “tractor” in the “Common” industry or “trailer” in the “On-Highway” industry group.
    #[must_use]
    pub const fn vehicle_system(&self) -> u8 {
        self.vehicle_system_bits()
    }

    /// Always zero(false).
    #[must_use]
    pub const fn reserved(&self) -> bool {
        self.reserved_bits()
    }

    /// This code, in a range between 128 and 255, is assigned according to the Industry Group. A
    /// value between 0 and 127 is not associated with any other parameter.
    #[must_use]
    pub const fn function(&self) -> u8 {
        self.function_bits()
    }

    /// Returns the function instance.
    #[must_use]
    pub const fn function_instance(&self) -> u8 {
        self.function_instance_bits()
    }

    /// A J1939 network may accommodate several ECUs of the same kind (i.e. same functionality).
    /// The Instance code separates them.
    #[must_use]
    pub const fn ecu_instance(&self) -> u8 {
        self.ecu_instance_bits()
    }

    /// The 11-Bit Manufacturer Code is assigned by the SAE.
    #[must_use]
    pub const fn manufacturer_code(&self) -> u16 {
        self.manufacturer_code_bits()
    }

    /// This field is assigned by the manufacturer, similar to a serial number, i.e. the code must
    /// be uniquely assigned to the unit.
    #[must_use]
    pub const fn identity_number(&self) -> u32 {
        self.identity_number_bits()
    }
}

#[cfg(test)]
mod test_name {
    use crate::name::Name;

    #[test]
    fn test_name_bitfield() {
        let name_a = Name::new()
            .with_arbitrary_address_bits(true)
            .with_industry_group_bits(0)
            .with_vehicle_system_instance_bits(0x5)
            .with_vehicle_system_bits(0x6)
            .with_reserved_bits(false)
            .with_function_bits(0x5)
            .with_function_instance_bits(0x2)
            .with_ecu_instance_bits(0x1)
            .with_manufacturer_code_bits(0x122)
            .with_identity_number_bits(0xB0309);

        let bytes_a: [u8; 8] = [0x09, 0x03, 0x4B, 0x24, 0x11, 0x05, 0x0C, 0x85];
        let name_a_bytes = name_a.into_bits().to_le_bytes();

        assert_eq!(bytes_a, name_a_bytes);
    }
}
