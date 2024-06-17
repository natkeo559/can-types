//! A rust crate for encoding or decoding various Controller Area Network (CAN) data types, including the SAE J1939 standard.
//!
//! # `no_std`
//! This crate supports `no_std`, meaning it can be used in resource-constrained environments like embedded systems and `IoT` devices, allowing for minimal memory usage without compromising functionality.
//!
//! To enable `no_std`, use the `--no-default-features` flag:
//! ```shell
//! $ cargo add can_types --no-default-features
//! ```
//!
//! #### `no_std` caveats:
//! - No ability to generate `String` or string slice hexadecimal representations of the data structures provided in this crate.
//!
//! See [Conversion](crate::prelude::Conversion) for more details.
//!
//! # Examples
//! ## Hex to J1939 Identifier
//! ```rust
//!     # use can_types::prelude::*;
//!     # fn main() -> Result<(), anyhow::Error> {
//!         let id_a = IdExtended::try_from_hex("0CF00400")?;
//!
//!         # assert_eq!(0b00001100111100000000010000000000, id_a.try_into_bits()?);
//!         assert_eq!(3, id_a.priority());
//!         assert_eq!(0, id_a.source_address());
//!         # Ok(())
//!     # }
//! ```
//!
//! ## Decode J1939 PGN
//! ```rust
//!     # use can_types::prelude::*;
//!     # fn main() -> Result<(), anyhow::Error> {
//!         let id_a = IdExtended::try_from_hex("18FEF200")?;
//!
//!         assert_eq!(CommunicationMode::Broadcast, id_a.pgn().communication_mode());
//!         assert_eq!(GroupExtension::Some(242), id_a.pgn().group_extension());
//!  
//!         let id_b = IdExtended::try_from_hex("0C00290B")?;
//!         
//!         // SA 11 = Brake
//!         assert_eq!(11, id_b.source_address());        
//!
//!         assert_eq!(PduFormat::Pdu1(0), id_b.pgn().pdu_format());
//!         assert_eq!(CommunicationMode::P2P, id_b.pgn().communication_mode());
//!         assert_eq!(GroupExtension::None, id_b.pgn().group_extension());
//!
//!         // DA 41 = Retarder, Exhaust, Engine
//!         assert_eq!(DestinationAddress::Some(41), id_b.pgn().destination_address());
//!         # Ok(())
//!     # }
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
