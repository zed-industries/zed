/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Test utilities for time and sleep

mod controlled_sleep;
pub use controlled_sleep::{controlled_time_and_sleep, CapturedSleep, ControlledSleep, SleepGate};

mod instant_sleep;
pub use instant_sleep::{instant_time_and_sleep, InstantSleep};

mod manual_time;
pub use manual_time::ManualTimeSource;

pub mod tick_advance_sleep;
