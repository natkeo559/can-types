// Copyright (C) 2024  Nathan H. Keough
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
