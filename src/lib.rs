//! A rust crate for encoding or decoding various Controller Area Network (CAN) data types, including the SAE J1939 standard.
//!
//! ## `no_std`
//! By default, this crate supports `no_std`, meaning it can be used in resource-constrained environments like embedded systems and `IoT` devices, allowing for minimal memory usage without compromising functionality.
//!
//! ## Examples
//!
//! ```rust,no_run
//!     use can_types::{IdExtended, CommunicationMode, GroupExtension};
//!
//!     fn main() -> Result<(), anyhow::Error> {
//!
//!         // Hex to J1939 Identifier
//!         let id_a = IdExtended::from_hex("0CF00400")?;
//!
//!         assert_eq!(0b00001100111100000000010000000000, id_a.to_bits());
//!         assert_eq!(3, id_a.priority_bits());
//!         assert_eq!(false, id_a.reserved_bits());
//!         assert_eq!(false, id_a.data_page_bits());
//!         assert_eq!(240, id_a.pdu_format_bits());   
//!         
//!         // Decode J1939 PGN
//!         let id_b = IdExtended::from_hex("18FEF200")?;
//!         let id_c = IdExtended::from_hex("0C00290B")?;
//!         
//!         assert_eq!(CommunicationMode::Broadcast, id_b.communication_mode());
//!         assert_eq!(CommunicationMode::P2P, id_c.communication_mode());
//!         assert_eq!(GroupExtension::Some(242), id_b.group_extension());
//!         assert_eq!(GroupExtension::None, id_c.group_extension());
//!
//!         Ok(())
//!     }
//! ```

#![no_std]

pub mod data;
pub mod identifier;
pub mod name;
pub mod pgn;

pub use crate::data::*;
pub use crate::identifier::*;
pub use crate::name::*;
pub use crate::pgn::*;
