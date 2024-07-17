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

//! # CAN Protocols Supported by this Crate.
//!
//! ## [CAN2.0 A](crate::protocol::can2_a)
//!
//! **Description:**
//! CAN2.0 A is the original specification of the CAN protocol, which defines a communication system
//! for vehicles and industrial automation. It provides a robust and efficient way for microcontrollers
//! and devices to communicate with each other in real-time.
//!
//! - **Data Frame Format:** CAN2.0 A specifies an 11-bit identifier for message frames.
//! - **Standardization:** It is widely used and supports basic communication needs.
//! - **Application:** Commonly used in applications where a smaller identifier field suffices, such as
//!   simpler automotive or industrial systems.
//!
//! **Source Document:**
//! - *ISO 11898-1*
//!
//! ## [CAN2.0 B](crate::protocol::can2_b)
//!
//! **Description:**
//! CAN2.0 B is an extension of the CAN2.0 A specification and introduces a more flexible frame format
//! to accommodate larger networks and more complex systems.
//!
//! - **Data Frame Format:** CAN2.0 B adds support for a 29-bit identifier, known as Extended Frame Format,
//!   in addition to the 11-bit identifier of CAN2.0 A.
//! - **Standardization:** This extension allows for a larger number of unique identifiers and is backward
//!   compatible with CAN2.0 A.
//! - **Application:** Useful in scenarios where a larger range of identifiers is needed, such as more
//!   complex automotive systems or larger industrial networks.
//!
//! **Source Document:**
//! - *ISO 11898-1*
//!
//! ## [J1939](crate::protocol::j1939)
//!
//! **Description:**
//! J1939 is a higher-layer protocol based on CAN2.0, specifically designed for heavy-duty vehicles and
//! off-road equipment. It builds upon the CAN2.0 B physical layer and frame format but adds additional
//! features tailored to the needs of commercial vehicles.
//!
//! - **Data Frame Format:** Utilizes the Extended Frame Format (29-bit identifier) of CAN2.0 B.
//! - **Standardization:** Includes specifications for message formats, data encoding, and communication
//!   parameters tailored for truck and bus applications.
//! - **Application:** Commonly used in the trucking industry and off-highway vehicles for diagnostics,
//!   control, and communication among different vehicle systems.
//!
//! **Source Document:**
//! - *SAE J1939-01*
//! - *SAE J1939-21*
//! - *SAE J1939-71*

pub mod can2_a;
pub mod can2_b;
pub mod j1939;
