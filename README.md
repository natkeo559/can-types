A rust crate for encoding or decoding various Controller Area Network (CAN) data types, including the SAE J1939 standard.
## Supported CAN Protocols
This crate provides decoding capabilities for Controller Area Network (CAN) protocols utilizing both 11-bit and 29-bit identifiers.
- **CAN 2.0 (Classical CAN)**: Commonly used in automotive applications and industrial automation.
- **SAE J1939**: Commonly used in heavy-duty trucks and buses.
## `no_std` Support
This crate supports `no_std`, meaning it can be used in resource-constrained environments like embedded systems and `IoT` devices, allowing for minimal memory usage without compromising functionality.
To enable `no_std`, use the `--no-default-features` flag:
```shell
$ cargo add can_types --no-default-features
```
### `no_std` limitations:
- This mode does not support generating `String` or string slice hexadecimal representations of the provided data structures.
See `Conversion` for more details.
# Examples
## Hex to J1939 Identifier
```rust
let id_a = Id::<J1939>::try_from_hex("0CF00400")?;

assert_eq!(3, id_a.priority());
assert_eq!(SourceAddr::Some(0), id_a.source_address());
assert_eq!(Some(Addr::PrimaryEngineController), id_a.source_address().lookup());
```
## Decode J1939 PGN
```rust
let id_a = Id::<J1939>::try_from_hex("18FEF200")?;

assert_eq!(CommunicationMode::Broadcast, id_a.pgn().communication_mode());
assert_eq!(GroupExtension::Some(242), id_a.pgn().group_extension());
 
let id_b = Id::<J1939>::try_from_hex("0C00290B")?;
        
// SA 11 = Brakes
assert_eq!(SourceAddr::Some(11), id_b.source_address());
assert_eq!(Some(Addr::Brakes), id_b.source_address().lookup());
assert_eq!(PduFormat::Pdu1(0), id_b.pgn().pdu_format());
assert_eq!(CommunicationMode::P2P, id_b.pgn().communication_mode());
assert_eq!(GroupExtension::None, id_b.pgn().group_extension());

// DA 41 = Retarder, Exhaust, Engine
assert_eq!(DestinationAddr::Some(41), id_b.pgn().destination_address());
assert_eq!(Some(Addr::RetarderExhaustEngine1), id_b.pgn().destination_address().lookup());
```