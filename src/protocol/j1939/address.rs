// Copyright (c) 2024 Nathan H. Keough
//
// This work is dual-licensed under MIT OR Apache 2.0 (or any later version).
// You may choose between one of them if you use this work.
//
// For further detail, please refer to the individual licenses located at the root of this crate.

//! # Vehicle Control Units/Modules, Source Adresses, and Destination Addresses

if_alloc! {
    use crate::alloc::fmt::{Display, Formatter, Result};
}

/// The [`Addr`] enum represents various types of electronic control units (ECUs) and modules
/// commonly found in automotive and heavy-duty vehicle networks. Each variant corresponds to a
/// specific ECU or system component that communicates over the network.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Addr {
    PrimaryEngineController,
    SecondaryEngineController,
    PrimaryTransmissionController,
    TransmissionShiftSelector,
    Brakes,
    Retarder,
    CruiseControl,
    FuelSystem,
    SteeringController,
    InstrumentCluster,
    ClimateControl1,
    Compass,
    BodyController,
    OffVehicleGateway,
    DidVid,
    RetarderExhaustEngine1,
    HeadwayController,
    Suspension,
    CabController,
    TirePressureController,
    LightingControlModule,
    ClimateControl2,
    ExhaustEmissionController,
    AuxiliaryHeater,
    ChassisController,
    CommunicationsUnit,
    Radio,
    SafetyRestraintSystem,
    AftertreatmentControlModule,
    MultiPurposeCamera,
    SwitchExpansionModule,
    AuxillaryGaugeSwitchPack,
    Iteris,
    QualcommPeopleNetTranslatorBox,
    StandAloneRealTimeClock,
    CenterPanel1,
    CenterPanel2,
    CenterPanel3,
    CenterPanel4,
    CenterPanel5,
    WabcoOnGuardRadar,
    SecondaryInstrumentCluster,
    OffboardDiagnostics,
    Trailer3Bridge,
    Trailer2Bridge,
    Trailer1Bridge,
    SafetyDirectProcessor,
    ForwardRoadImageProcessor,
    LeftRearDoorPod,
    RightRearDoorPod,
    DoorController1,
    DoorController2,
    Tachograph,
    HybridSystem,
    AuxiliaryPowerUnit,
    ServiceTool,
    SourceAddressRequest0,
    SourceAddressRequest1,
    Unknown(u8),
}

impl From<u8> for Addr {
    fn from(value: u8) -> Self {
        match value {
            0 => Addr::PrimaryEngineController,
            1 => Addr::SecondaryEngineController,
            3 => Addr::PrimaryTransmissionController,
            5 => Addr::TransmissionShiftSelector,
            11 => Addr::Brakes,
            15 => Addr::Retarder,
            17 => Addr::CruiseControl,
            18 => Addr::FuelSystem,
            19 => Addr::SteeringController,
            23 => Addr::InstrumentCluster,
            25 => Addr::ClimateControl1,
            28 => Addr::Compass,
            33 => Addr::BodyController,
            37 => Addr::OffVehicleGateway,
            40 => Addr::DidVid,
            41 => Addr::RetarderExhaustEngine1,
            42 => Addr::HeadwayController,
            47 => Addr::Suspension,
            49 => Addr::CabController,
            51 => Addr::TirePressureController,
            55 => Addr::LightingControlModule,
            58 => Addr::ClimateControl2,
            61 => Addr::ExhaustEmissionController,
            69 => Addr::AuxiliaryHeater,
            71 => Addr::ChassisController,
            74 => Addr::CommunicationsUnit,
            76 => Addr::Radio,
            83 => Addr::SafetyRestraintSystem,
            85 => Addr::AftertreatmentControlModule,
            127 => Addr::MultiPurposeCamera,
            128 => Addr::SwitchExpansionModule,
            132 => Addr::AuxillaryGaugeSwitchPack,
            139 => Addr::Iteris,
            142 => Addr::QualcommPeopleNetTranslatorBox,
            150 => Addr::StandAloneRealTimeClock,
            151 => Addr::CenterPanel1,
            152 => Addr::CenterPanel2,
            153 => Addr::CenterPanel3,
            154 => Addr::CenterPanel4,
            155 => Addr::CenterPanel5,
            160 => Addr::WabcoOnGuardRadar,
            167 => Addr::SecondaryInstrumentCluster,
            172 => Addr::OffboardDiagnostics,
            184 => Addr::Trailer3Bridge,
            192 => Addr::Trailer2Bridge,
            200 => Addr::Trailer1Bridge,
            209 => Addr::SafetyDirectProcessor,
            232 => Addr::ForwardRoadImageProcessor,
            233 => Addr::LeftRearDoorPod,
            234 => Addr::RightRearDoorPod,
            236 => Addr::DoorController1,
            237 => Addr::DoorController2,
            238 => Addr::Tachograph,
            239 => Addr::HybridSystem,
            247 => Addr::AuxiliaryPowerUnit,
            249 => Addr::ServiceTool,
            254 => Addr::SourceAddressRequest0,
            255 => Addr::SourceAddressRequest1,
            a => Addr::Unknown(a),
        }
    }
}

impl From<Addr> for u8 {
    fn from(value: Addr) -> Self {
        match value {
            Addr::PrimaryEngineController => 0,
            Addr::SecondaryEngineController => 1,
            Addr::PrimaryTransmissionController => 3,
            Addr::TransmissionShiftSelector => 5,
            Addr::Brakes => 11,
            Addr::Retarder => 15,
            Addr::CruiseControl => 17,
            Addr::FuelSystem => 18,
            Addr::SteeringController => 19,
            Addr::InstrumentCluster => 23,
            Addr::ClimateControl1 => 25,
            Addr::Compass => 28,
            Addr::BodyController => 33,
            Addr::OffVehicleGateway => 37,
            Addr::DidVid => 40,
            Addr::RetarderExhaustEngine1 => 41,
            Addr::HeadwayController => 42,
            Addr::Suspension => 47,
            Addr::CabController => 49,
            Addr::TirePressureController => 51,
            Addr::LightingControlModule => 55,
            Addr::ClimateControl2 => 58,
            Addr::ExhaustEmissionController => 61,
            Addr::AuxiliaryHeater => 69,
            Addr::ChassisController => 71,
            Addr::CommunicationsUnit => 74,
            Addr::Radio => 76,
            Addr::SafetyRestraintSystem => 83,
            Addr::AftertreatmentControlModule => 85,
            Addr::MultiPurposeCamera => 127,
            Addr::SwitchExpansionModule => 128,
            Addr::AuxillaryGaugeSwitchPack => 132,
            Addr::Iteris => 139,
            Addr::QualcommPeopleNetTranslatorBox => 142,
            Addr::StandAloneRealTimeClock => 150,
            Addr::CenterPanel1 => 151,
            Addr::CenterPanel2 => 152,
            Addr::CenterPanel3 => 153,
            Addr::CenterPanel4 => 154,
            Addr::CenterPanel5 => 155,
            Addr::WabcoOnGuardRadar => 160,
            Addr::SecondaryInstrumentCluster => 167,
            Addr::OffboardDiagnostics => 172,
            Addr::Trailer3Bridge => 184,
            Addr::Trailer2Bridge => 192,
            Addr::Trailer1Bridge => 200,
            Addr::SafetyDirectProcessor => 209,
            Addr::ForwardRoadImageProcessor => 232,
            Addr::LeftRearDoorPod => 233,
            Addr::RightRearDoorPod => 234,
            Addr::DoorController1 => 236,
            Addr::DoorController2 => 237,
            Addr::Tachograph => 238,
            Addr::HybridSystem => 239,
            Addr::AuxiliaryPowerUnit => 247,
            Addr::ServiceTool => 249,
            Addr::SourceAddressRequest0 => 254,
            Addr::SourceAddressRequest1 => 255,
            Addr::Unknown(a) => a,
        }
    }
}

#[cfg(feature = "alloc")]
impl Display for Addr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Addr::PrimaryEngineController => write!(f, "Primary Engine Controller | (CPC, ECM)"),
            Addr::SecondaryEngineController => write!(f, "Secondary Engine Controller | (MCM, ECM #2)"),
            Addr::PrimaryTransmissionController => write!(f, "Primary Transmission Controller | (TCM)"),
            Addr::TransmissionShiftSelector => write!(f, "Transmission Shift Selector | (TSS)"),
            Addr::Brakes => write!(f, "Brakes | System Controller (ABS)"),
            Addr::Retarder => write!(f, "Retarder"),
            Addr::CruiseControl => write!(f, "Cruise Control | (IPM, PCC)"),
            Addr::FuelSystem => write!(f, "Fuel System | Controller (CNG)"),
            Addr::SteeringController => write!(f, "Steering Controller | (SAS)"),
            Addr::InstrumentCluster => write!(f, "Instrument Guage Cluster (EGC) | (ICU, RX)"),
            Addr::ClimateControl1 => write!(f, "Climate Control #1 | (FCU)"),
            Addr::Compass => write!(f, "Compass"),
            Addr::BodyController => write!(f, "Body Controller | (SSAM, SAM-CAB, BHM)"),
            Addr::OffVehicleGateway => write!(f, "Off-Vehicle Gateway | (CGW)"),
            Addr::DidVid => write!(f, "Vehicle Information Display | Driver Information Display"),
            Addr::RetarderExhaustEngine1 => write!(f, "Retarder, Exhaust, Engine #1"),
            Addr::HeadwayController => write!(f, "Headway Controller | (RDF) | (OnGuard)"),
            Addr::Suspension => write!(f, "Suspension | System Controller (ECAS)"),
            Addr::CabController => write!(f, "Cab Controller | Primary (MSF, SHM, ECC)"),
            Addr::TirePressureController => write!(f, "Tire Pressure Controller | (TPMS)"),
            Addr::LightingControlModule => write!(f, "Lighting Control Module | (LCM)"),
            Addr::ClimateControl2 => write!(f, "Climate Control #2 | Rear HVAC | (ParkSmart)"),
            Addr::ExhaustEmissionController => write!(f, "Exhaust Emission Controller | (ACM) | (DCU)"),
            Addr::AuxiliaryHeater => write!(f, "Auxiliary Heater | (ACU)"),
            Addr::ChassisController => write!(f, "Chassis Controller | (CHM, SAM-Chassis)"),
            Addr::CommunicationsUnit => write!(f, "Communications Unit | Cellular (CTP, VT)"),
            Addr::Radio => write!(f, "Radio"),
            Addr::SafetyRestraintSystem => write!(f, "Safety Restraint System | Air Bag | (SRS)"),
            Addr::AftertreatmentControlModule => write!(f, "Aftertreatment Control Module | (ACM)"),
            Addr::MultiPurposeCamera => write!(f, "Multi-Purpose Camera | (MPC)"),
            Addr::SwitchExpansionModule => write!(f, "Switch Expansion Module | (SEM #1)"),
            Addr::AuxillaryGaugeSwitchPack => write!(f, "Auxiliary Gauge Switch Pack | (AGSP3)"),
            Addr::Iteris => write!(f, "Iteris"),
            Addr::QualcommPeopleNetTranslatorBox => write!(f, "Qualcomm - PeopleNet Translator Box"),
            Addr::StandAloneRealTimeClock => write!(f, "Stand-Alone Real Time Clock | (SART)"),
            Addr::CenterPanel1 => write!(f, "Center Panel MUX Switch Pack #1"),
            Addr::CenterPanel2 => write!(f, "Center Panel MUX Switch Pack #2"),
            Addr::CenterPanel3 => write!(f, "Center Panel MUX Switch Pack #3"),
            Addr::CenterPanel4 => write!(f, "Center Panel MUX Switch Pack #4"),
            Addr::CenterPanel5 => write!(f, "Center Panel MUX Switch Pack #5"),
            Addr::WabcoOnGuardRadar => write!(f, "Wabco OnGuard Radar | OnGuard Display | Collison Mitigation System"),
            Addr::SecondaryInstrumentCluster => write!(f, "Secondary Instrument Cluster | (SIC)"),
            Addr::OffboardDiagnostics => write!(f, "Offboard Diagnostics"),
            Addr::Trailer3Bridge => write!(f, "Trailer #3 Bridge"),
            Addr::Trailer2Bridge => write!(f, "Trailer #2 Bridge"),
            Addr::Trailer1Bridge => write!(f, "Trailer #1 Bridge"),
            Addr::SafetyDirectProcessor => write!(f, "Bendix Camera | Safety Direct Processor (SDP) Module"),
            Addr::ForwardRoadImageProcessor => write!(f, "Forward Road Image Processor | PAM Module | Lane Departure Warning (LDW) Module | (VRDU)"),
            Addr::LeftRearDoorPod => write!(f, "Left Rear Door Pod"),
            Addr::RightRearDoorPod => write!(f, "Right Rear Door Pod"),
            Addr::DoorController1 => write!(f, "Door Controller #1"),
            Addr::DoorController2 => write!(f, "Door Controller #2"),
            Addr::Tachograph => write!(f, "Tachograph | (TCO)"),
            Addr::HybridSystem => write!(f, "Hybrid System"),
            Addr::AuxiliaryPowerUnit => write!(f, "Auxiliary Power Unit | (APU)"),
            Addr::ServiceTool => write!(f, "Service Tool"),
            Addr::SourceAddressRequest0 => write!(f, "Source Address Request 0"),
            Addr::SourceAddressRequest1 => write!(f, "Source Address Request 1"),
            Addr::Unknown(num) => write!(f, "Unknown({num})"),
        }
    }
}

/// Represents the source address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceAddr {
    /// No source address.
    None,
    /// Source address with a specific value.
    Some(u8),
}

/// Represents the destination address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationAddr {
    /// No destination address.
    None,
    /// Destination address with a specific value.
    Some(u8),
}

impl SourceAddr {
    /// Lookup and translate the [`SourceAddr`] object.
    ///
    /// # Returns
    /// - `Some(Addr)`: If generic J1939 address is known.
    /// - `None`: If the pdu specific bits do not contain a destination address.
    #[must_use]
    pub fn lookup(self) -> Option<Addr> {
        match self {
            SourceAddr::Some(value) => Some(value.into()),
            SourceAddr::None => None,
        }
    }
}

impl DestinationAddr {
    /// Lookup and translate the [`DestinationAddr`] object.
    ///
    /// # Returns
    /// - `Some(Addr)`: If generic J1939 address is known.
    /// - `None`: If the pdu specific bits do not contain a destination address.
    #[must_use]
    pub fn lookup(self) -> Option<Addr> {
        match self {
            DestinationAddr::Some(value) => Some(value.into()),
            DestinationAddr::None => None,
        }
    }
}

#[cfg(test)]
mod sa_tests {
    if_alloc! {
        use alloc::format;
    }

    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_translate() {
        let sa_translated: Addr = 41.into();
        assert_eq!("Retarder, Exhaust, Engine #1", format!("{sa_translated}"));

        let sa_value: u8 = Addr::RetarderExhaustEngine1.into();
        assert_eq!(41, sa_value)
    }
}
