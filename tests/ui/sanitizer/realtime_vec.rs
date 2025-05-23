//@ needs-sanitizer-support
//@ needs-sanitizer-realtime
//
//@ compile-flags: -Z sanitizer=realtime
//
//@ run-fail
//@ error-pattern: Intercepted call to real-time unsafe function `malloc` in real-time context!

extern "C" {
    pub fn __rtsan_realtime_enter();
    pub fn __rtsan_realtime_exit();
    pub fn __rtsan_disable();
    pub fn __rtsan_enable();
    pub fn __rtsan_ensure_initialized();
    pub fn __rtsan_notify_blocking_call(blocking_function_name: *const core::ffi::c_char);
}

fn main() {
    unsafe {
        __rtsan_ensure_initialized();
        __rtsan_realtime_enter();
    }
    let vec = vec![0, 1, 2];
    unsafe {
        __rtsan_realtime_exit();
    }
    println!("alloc not detected")
}
