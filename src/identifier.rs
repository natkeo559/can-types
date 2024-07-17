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
