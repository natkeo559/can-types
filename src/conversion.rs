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
    use crate::alloc::string::String;
}

use crate::identifier::Id;
use crate::prelude::{Data, Name, Pdu};
use crate::protocol::{Can2A, Can2B};

/// A trait for types that can be converted to and from bitfield representations (`bits`)
/// of integers and hexadecimal string slices (hex).
///
/// #### `alloc` features:
/// - `into_hex`
pub trait Conversion<T>
where
    Self: Sized,
{
    type Error;

    /// Convert an integer of type `T` into `Self`
    fn from_bits(bits: T) -> Self;

    /// Convert a hexadecimal string slice into `Self`
    fn from_hex(hex_str: &str) -> Self;

    /// Convert an integer of type `T` into `Self`
    /// # Errors
    /// - Implementation dependent
    fn try_from_bits(bits: T) -> Result<Self, Self::Error>;

    /// Convert a hexadecimal string slice into `Self`
    /// # Errors
    /// - Implementation dependent
    fn try_from_hex(hex_str: &str) -> Result<Self, Self::Error>;

    /// Convert `self` into an integer of type `T`
    fn into_bits(self) -> T;

    /// Convert `self` into a hexadecimal string
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String;
}

impl From<Pdu<Data>> for Pdu<Name> {
    fn from(value: Pdu<Data>) -> Self {
        Self::from_bits(value.into_bits())
    }
}

impl From<Pdu<Name>> for Pdu<Data> {
    fn from(value: Pdu<Name>) -> Self {
        Self::from_bits(value.into_bits())
    }
}

impl From<Id<Can2A>> for Id<Can2B> {
    fn from(value: Id<Can2A>) -> Self {
        Self::from_bits(value.into_bits().into())
    }
}

#[cfg(test)]
mod impl_tests {
    use crate::prelude::{Data, Name, Pdu};

    use super::*;

    #[test]
    fn test_data_from() {
        let name_a = Pdu::<Name>::from_hex("FFFF82DF1AFFFFFF");
        let data_a = Pdu::<Data>::from(name_a);

        assert_eq!(Pdu::<Data>::from_hex("FFFF82DF1AFFFFFF"), data_a);
    }

    #[test]
    fn test_name_from() {
        let data_a = Pdu::<Data>::from_hex("FFFF82DF1AFFFFFF");
        let name_a = Pdu::<Name>::from(data_a);

        assert_eq!(Pdu::<Name>::from_hex("FFFF82DF1AFFFFFF"), name_a);
    }

    #[test]
    fn test_extended_from() {
        let id_std_a = Id::<Can2A>::from_hex("00F");
        let id_ext_a = Id::<Can2B>::from(id_std_a);

        assert_eq!(Id::<Can2B>::from_hex("0000000F"), id_ext_a);
    }
}
