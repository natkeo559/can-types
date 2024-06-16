//! A rust crate for encoding or decoding various Controller Area Network (CAN) data types, including the SAE J1939 standard.
//!
//! ## `no_std`
//! By default, this crate supports `no_std`, meaning it can be used in resource-constrained environments like embedded systems and `IoT` devices, allowing for minimal memory usage without compromising functionality.
//!
//! ## Examples
//!
//! ```rust,no_run
//!     use can_types::prelude::*;
//!
//!     fn main() -> Result<(), anyhow::Error> {
//!
//!         // Hex to J1939 Identifier
//!         let id_a = IdExtended::try_from_hex("0CF00400")?;
//!
//!         assert_eq!(0b00001100111100000000010000000000, id_a.try_into_bits()?);
//!         assert_eq!(3, id_a.priority());
//!         assert_eq!(false, id_a.reserved());
//!         assert_eq!(false, id_a.data_page());
//!         assert_eq!(240, id_a.pdu_format());   
//!         
//!         // Decode J1939 PGN
//!         let id_b = IdExtended::try_from_hex("18FEF200")?;
//!         let id_c = IdExtended::try_from_hex("0C00290B")?;
//!         
//!         assert_eq!(CommunicationMode::Broadcast, id_b.pgn().communication_mode());
//!         assert_eq!(CommunicationMode::P2P, id_c.pgn().communication_mode());
//!         assert_eq!(GroupExtension::Some(242), id_b.pgn().group_extension());
//!         assert_eq!(GroupExtension::None, id_c.pgn().group_extension());
//!
//!         Ok(())
//!     }
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
pub mod data;
pub mod identifier;
pub mod message;
pub mod name;
pub mod pgn;

pub mod prelude {
    pub use crate::conversion::*;
    pub use crate::data::*;
    pub use crate::identifier::*;
    pub use crate::message::*;
    pub use crate::name::*;
    pub use crate::pgn::*;
    if_alloc! {
        pub use alloc::string::String;
    }
}
