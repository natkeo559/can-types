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

use crate::identifier::{IdExtended, IdStandard};
use crate::payload::{PduData, PduName};

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
    /// - Implementation dependent
    fn try_into_bits(self) -> Result<T, Self::Error>;

    /// Convert `self` into a hexadecimal string
    /// # Errors
    /// - Implementation dependent
    /// # Requires
    /// - `alloc`
    #[cfg(feature = "alloc")]
    fn try_into_hex(self) -> Result<String, Self::Error>;

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

    /// Convert an integer of type `T` into `Self`
    fn from_bits(bits: T) -> Self;

    /// Convert a hexadecimal string slice into `Self`
    fn from_hex(hex_str: &str) -> Self;
}

impl From<PduData> for PduName {
    fn from(value: PduData) -> Self {
        Self::from_bits(value.into_bits())
    }
}

impl From<PduName> for PduData {
    fn from(value: PduName) -> Self {
        Self::from_bits(value.into_bits())
    }
}

impl From<IdStandard> for IdExtended {
    fn from(value: IdStandard) -> Self {
        Self::from_bits(value.into_bits().into())
    }
}

#[cfg(test)]
mod impl_tests {
    use crate::prelude::{PduData, PduName};

    use super::*;

    #[test]
    fn test_data_from() {
        let name_a = PduName::from_hex("FFFF82DF1AFFFFFF");
        let data_a = PduData::from(name_a);

        assert_eq!(PduData::from_hex("FFFF82DF1AFFFFFF"), data_a);
    }

    #[test]
    fn test_name_from() {
        let data_a: PduData = PduData::from_hex("FFFF82DF1AFFFFFF");
        let name_a: PduName = PduName::from(data_a);

        assert_eq!(PduName::from_hex("FFFF82DF1AFFFFFF"), name_a);
    }

    #[test]
    fn test_extended_from() {
        let id_std_a = IdStandard::from_hex("00F");
        let id_ext_a = IdExtended::from(id_std_a);

        assert_eq!(IdExtended::from_hex("0000000F"), id_ext_a);
    }
}
