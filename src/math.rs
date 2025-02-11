#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(clippy::all)]
#[path = "../libm/src/math/mod.rs"]
mod libm;

#[allow(unused_macros)]
macro_rules! no_mangle {
    ($(fn $fun:ident($($iid:ident : $ity:ty),+) -> $oty:ty;)+) => {
        intrinsics! {
            $(
                pub extern "C" fn $fun($($iid: $ity),+) -> $oty {
                    self::libm::$fun($($iid),+)
                }
            )+
        }
    }
}
