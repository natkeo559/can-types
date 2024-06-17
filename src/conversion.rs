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

if_alloc! {
    use crate::prelude::String;
}

/// A trait for types that can be converted to and from bitfield representations (`bits`)
/// of integers and hexadecimal string slices (hex).
///
/// #### `alloc` features:
/// - `try_into_hex`
/// - `into_hex`
pub trait Conversion<T>
where
    Self: Sized,
{
    type Error;

    /// Convert `self` into an integer of type `T`
    /// # Errors
    /// - Implementation dependant
    fn try_into_bits(self) -> Result<T, Self::Error>;

    /// Convert `self` into a hexadecimal string
    /// # Errors
    /// - Implementation dependant
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn try_into_hex(self) -> Result<String, Self::Error>;

    /// Convert an integer of type `T` into `Self`
    /// # Errors
    /// - Implementation dependant
    fn try_from_bits(bits: T) -> Result<Self, Self::Error>;

    /// Convert a hexadecimal string slice into `Self`
    /// # Errors
    /// - Implementation dependant
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error>;

    /// Convert `self` into an integer of type `T`
    fn into_bits(self) -> T;

    /// Convert `self` into a hexadecimal string
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String;

    /// Convert an integer of type `T` into `Self`
    fn from_bits(bits: T) -> Self;

    /// Convert a hexadecimal string slice into `Self`
    fn from_hex(hex_str: &str) -> Self;
}
