// MIT License
// 
// Copyright (c) 2024 Nathan H. Keough
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Defines the standard conversion between units of data provided in this crate.

use crate::{
    payload::{Data, Name, Pdu},
    protocol::{can2_a::identifier::IdCan2A, can2_b::identifier::IdCan2B},
};

if_alloc! {
    use crate::alloc::string::String;
}

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

impl From<IdCan2A> for IdCan2B {
    fn from(value: IdCan2A) -> Self {
        Self::from_bits(value.into_bits().into())
    }
}

#[cfg(test)]
mod impl_tests {
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
        let id_std_a = IdCan2A::from_hex("00F");
        let id_ext_a = IdCan2B::from(id_std_a);

        assert_eq!(IdCan2B::from_hex("0000000F"), id_ext_a);
    }
}
