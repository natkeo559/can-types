#[cfg(feature = "alloc")]
use crate::prelude::String;

/// A trait for types that can be converted to and from bitfield representations (`bits`)
/// of integers and hexadecimal string slices (`hex`).
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
    #[cfg(feature = "alloc")]
    fn into_hex(self) -> String;

    /// Convert an integer of type `T` into `Self`
    fn from_bits(bits: T) -> Self;

    /// Convert a hexadecimal string slice into `Self`
    fn from_hex(hex_str: &str) -> Self;
}
