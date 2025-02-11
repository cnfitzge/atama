
#[cfg(test)]
extern crate core;

#[macro_use]
mod macros;

pub mod float;
pub mod int;

pub mod math;
pub mod mem;

#[cfg(target_arch = "arm")]
pub mod arm;

#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
pub mod aarch64;

#[cfg(all(target_arch = "aarch64", target_os = "linux", not(feature = "no-asm"),))]
pub mod aarch64_linux;

#[cfg(all(
    kernel_user_helpers,
    any(target_os = "linux", target_os = "android"),
    target_arch = "arm"
))]
pub mod arm_linux;

#[cfg(target_arch = "hexagon")]
pub mod hexagon;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub mod riscv;

#[cfg(target_arch = "x86")]
pub mod x86;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod probestack;
