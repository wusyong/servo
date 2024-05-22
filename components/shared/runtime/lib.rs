/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![crate_name = "runtime"]
#![crate_type = "rlib"]
#![deny(unsafe_code)]

//! This module contains types and traits of async runtime and channel used in
//! the rest of Servo.

// TODO move net/async_runtime.rs to here
// pub mod async_runtime;

pub mod channel;
pub use channel::channel;
