#![cfg(not(feature = "no-asm"))]
#![allow(unused_imports)]

use core::intrinsics;

// iOS symbols have a leading underscore.
#[cfg(target_os = "ios")]
macro_rules! bl {
    ($func:literal) => {
        concat!("bl _", $func)
    };
}
#[cfg(not(target_os = "ios"))]
macro_rules! bl {
    ($func:literal) => {
        concat!("bl ", $func)
    };
}
