//! All proxy structures for communicating using AT-SPI.
//! Each proxy uses a different interface for communication.

#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code, rustdoc::all)]
#![allow(clippy::multiple_crate_versions)]

pub use atspi_common as common;

pub mod accessible;
pub mod action;
pub mod application;
pub mod bus;
pub mod cache;
pub mod collection;
pub mod component;
pub mod device_event_controller;
pub mod device_event_listener;
pub mod document;
pub mod editable_text;
pub mod proxy_ext;
pub use common::{events, AtspiError, CoordType, Interface, InterfaceSet};

pub mod hyperlink;
pub mod hypertext;
pub mod image;
pub mod registry;
pub mod selection;
pub mod socket;
pub mod table;
pub mod table_cell;
pub mod text;
pub mod value;
