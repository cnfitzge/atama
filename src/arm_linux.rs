use core::intrinsics;
use core::mem;

unsafe fn __kuser_cmpxchg(oldval: u32, newval: u32, ptr: *mut u32) -> bool {
    let f: extern "C" fn(u32, u32, *mut u32) -> u32 = mem::transmute(0xffff0fc0usize as *const ());
    f(oldval, newval, ptr) == 0
}
unsafe fn __kuser_memory_barrier() {
    let f: extern "C" fn() = mem::transmute(0xffff0fa0usize as *const ());
    f();
}
