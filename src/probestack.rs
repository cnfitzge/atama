#[cfg(all(target_os = "uefi", target_arch = "x86_64"))]
macro_rules! define_rust_probestack {
    ($body: expr) => {
        concat!(
            "
            .globl __rust_probestack
        __rust_probestack:
            ",
            $body
        )
    };
}

// Same as above, but for Mach-O. Note that the triple underscore
// is deliberate
#[cfg(target_vendor = "apple")]
macro_rules! define_rust_probestack {
    ($body: expr) => {
        concat!(
            "
            .globl ___rust_probestack
        ___rust_probestack:
            ",
            $body
        )
    };
}
