use bitfield_struct::bitfield;

/// Represents a Name in the SAE J1939 protocol.
///
/// The Name structure is used in the SAE J1939 protocol to represent the identity of a device or
/// component within a vehicle's network.
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

impl Name {
    /// Indicates whether or not the ECU/CA can negotiate an address (true = yes; false = no).
    #[must_use]
    pub fn arbitrary_address(&self) -> bool {
        self.arbitrary_address_bits()
    }

    /// These codes are associated with particular industries such as on-highway equipment,
    /// agricultural equipment, and more.
    #[must_use]
    pub fn industry_group(&self) -> u8 {
        self.industry_group_bits()
    }

    /// Assigns a number to each instance on the Vehicle System (in case you connect several
    /// networks – e.g. connecting cars on a train).
    #[must_use]
    pub fn vehicle_system_instance(&self) -> u8 {
        self.vehicle_system_instance_bits()
    }

    /// Vehicle systems are associated with the Industry Group and they can be, for instance,
    /// “tractor” in the “Common” industry or “trailer” in the “On-Highway” industry group.
    #[must_use]
    pub fn vehicle_system(&self) -> u8 {
        self.vehicle_system_bits()
    }

    /// Always zero(false).
    #[must_use]
    pub fn reserved(&self) -> bool {
        self.reserved_bits()
    }

    /// This code, in a range between 128 and 255, is assigned according to the Industry Group. A
    /// value between 0 and 127 is not associated with any other parameter.
    #[must_use]
    pub fn function(&self) -> u8 {
        self.function_bits()
    }

    /// Returns the function instance.
    #[must_use]
    pub fn function_instance(&self) -> u8 {
        self.function_instance_bits()
    }

    /// A J1939 network may accommodate several ECUs of the same kind (i.e. same functionality).
    /// The Instance code separates them.
    #[must_use]
    pub fn ecu_instance(&self) -> u8 {
        self.ecu_instance_bits()
    }

    /// The 11-Bit Manufacturer Code is assigned by the SAE.
    #[must_use]
    pub fn manufacturer_code(&self) -> u16 {
        self.manufacturer_code_bits()
    }

    /// This field is assigned by the manufacturer, similar to a serial number, i.e. the code must
    /// be uniquely assigned to the unit.
    #[must_use]
    pub fn identity_number(&self) -> u32 {
        self.identity_number_bits()
    }
}

#[cfg(test)]
mod test_name {
    use crate::Name;

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
