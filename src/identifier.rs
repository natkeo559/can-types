// Copyright (c) 2024 Nathan H. Keough
//
// This work is dual-licensed under MIT OR Apache 2.0 (or any later version).
// You may choose between one of them if you use this work.
//
// For further detail, please refer to the individual licenses located at the root of this crate.

//! Defines the `Id` type representing a Controller Area Network (CAN) identifier
//! specific to a protocol.
//!
//! Generics are employed here for flexibility and performance benefits. By parameterizing [`Id`]
//! over different protocol types (P) that conform to the [`IsProtocol`] trait, Rust's
//! monomorphization ensures efficient code generation at compile-time while avoiding namespace
//! pollution from dozens of individual types.

/// Marks a type, relating it to a specific protocol.
pub trait IsProtocol {}

/// Represents a Controller Area Network (CAN) identifier of a specific protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id<P: IsProtocol>(pub(crate) P);
