// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: main
#![no_main]
#![no_std]

mod exceptions;
mod logger;
mod pl011;

use crate::pl011::Uart;
use core::panic::PanicInfo;
use log::{error, info, LevelFilter};
use psci::system_off;

/// Base address of the primary PL011 UART.
pub const PL011_BASE_ADDRESS: usize = 0x900_0000;

#[no_mangle]
extern "C" fn main(x0: u64, x1: u64, x2: u64, x3: u64) {
    // Safe because `PL011_BASE_ADDRESS` is the base address of a PL011 device,
    // and nothing else accesses that address range.
    let uart = unsafe { Uart::new(PL011_BASE_ADDRESS as *mut u32) };
    logger::init(uart, LevelFilter::Trace).unwrap();

    info!("main({:#x}, {:#x}, {:#x}, {:#x})", x0, x1, x2, x3);

    assert_eq!(x1, 42);

    system_off().unwrap();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    system_off().unwrap();
    loop {}
}
// ANCHOR_END: main
