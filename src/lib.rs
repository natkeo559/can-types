// Copyright (c) 2024 Nathan H. Keough
//
// This work is dual-licensed under MIT OR Apache 2.0 (or any later version).
// You may choose between one of them if you use this work.
//
// For further detail, please refer to the individual licenses located at the root of this crate.

//! A rust crate for encoding or decoding various Controller Area Network (CAN) data types, including the SAE J1939 standard.
//!
//! ## Supported CAN Protocols
//! This crate provides decoding capabilities for Controller Area Network (CAN) protocols utilizing both 11-bit and 29-bit identifiers.
//! - **CAN 2.0 (Classical CAN)**: Commonly used in automotive applications and industrial automation.
//! - **SAE J1939**: Commonly used in heavy-duty trucks and buses.
//!
//! ## `no_std` Support
//! This crate supports `no_std`, meaning it can be used in resource-constrained environments like embedded systems and `IoT` devices, allowing for minimal memory usage without compromising functionality.
//!
//! To enable `no_std`, use the `--no-default-features` flag:
//! ```shell
//! $ cargo add can_types --no-default-features
//! ```
//!
//! ### `no_std` limitations:
//! - This mode does not support generating [`String`](alloc::string::String) or string slice hexadecimal representations of the provided data structures.
//!
//! # Examples
//! ## Hex to J1939 Identifier
//! ```rust
//! # use can_types::prelude::*;
//! # fn main() -> Result<(), anyhow::Error> {
//! let id_a = IdJ1939::try_from_hex("0CF00400")?;
//!
//! assert_eq!(3, id_a.priority());
//! assert_eq!(SourceAddr::Some(0), id_a.source_address());
//! assert_eq!(Some(Addr::PrimaryEngineController), id_a.source_address().lookup());
//! # Ok(())
//! # }
//! ```
//!
//! ## Decode J1939 PGN
//! ```rust
//! # use can_types::prelude::*;
//! # fn main() -> Result<(), anyhow::Error> {
//! let id_a = IdJ1939::try_from_hex("18FEF200")?;
//!
//! assert_eq!(CommunicationMode::Broadcast, id_a.pgn().communication_mode());
//! assert_eq!(GroupExtension::Some(242), id_a.pgn().group_extension());
//!  
//! let id_b = IdJ1939::try_from_hex("0C00290B")?;
//!         
//! // SA 11 = Brakes
//! assert_eq!(SourceAddr::Some(11), id_b.source_address());
//! assert_eq!(Some(Addr::Brakes), id_b.source_address().lookup());
//!
//! assert_eq!(PduFormat::Pdu1(0), id_b.pgn().pdu_format());
//! assert_eq!(CommunicationMode::P2P, id_b.pgn().communication_mode());
//! assert_eq!(GroupExtension::None, id_b.pgn().group_extension());
//!
//! // DA 41 = Retarder, Exhaust, Engine
//! assert_eq!(DestinationAddr::Some(41), id_b.pgn().destination_address());
//! assert_eq!(Some(Addr::RetarderExhaustEngine1), id_b.pgn().destination_address().lookup());   
//! # Ok(())
//! # }
//! ```

#![no_std]

macro_rules! if_alloc {
    ($($i:item)*) => ($(
        #[cfg(feature = "alloc")] $i
    )*)
}

if_alloc! {
    extern crate alloc;
}

pub mod conversion;
pub mod identifier;
pub mod message;
pub mod payload;
pub mod protocol;

#[doc(hidden)]
pub mod prelude {
    use super::{conversion, identifier, message, payload, protocol};

    pub use conversion::Conversion;
    pub use identifier::{Id, IsProtocol};
    pub use message::Message;
    pub use payload::{Data, IsDataUnit, Name, Pdu};
    pub use protocol::{
        can2_a::identifier::{Can2A, IdCan2A},
        can2_b::identifier::{Can2B, IdCan2B},
        j1939::{
            address::{Addr, DestinationAddr, SourceAddr},
            identifier::{IdJ1939, J1939},
            pgn::{CommunicationMode, GroupExtension, PduAssignment, PduFormat, Pgn},
        },
    };
}
