/// Changes the visibility to `pub` if feature "public-test-deps" is set
#[cfg(feature = "public-test-deps")]
macro_rules! public_test_dep {
    {$(#[$($meta:meta)*])* pub(crate) $ident:ident $($tokens:tt)*} => {
        $(#[$($meta)*])* pub $ident $($tokens)*
    };
}
