// This is a module that facilitates Crust-style programming - https://github.com/tsoding/crust
use crate::crust::libc::*;
use core::panic::PanicInfo;
use core::ffi::*;

#[macro_use]
pub mod libc {
    use core::ffi::*;

    pub type FILE = c_void;

    extern "C" {
        pub static stdin: *mut FILE;
        pub static stdout: *mut FILE;
        pub static stderr: *mut FILE;
        pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
        pub fn strlen(s: *const c_char) -> usize;
        pub fn abort() -> !;
        pub fn strdup(s: *const c_char) -> *mut c_char;
    }

    #[macro_export]
    macro_rules! printf {
        ($fmt:literal $($args:tt)*) => {{
            use core::ffi::c_int;
            extern "C" {
                #[link_name = "printf"]
                pub fn printf_raw(fmt: *const c_char, ...) -> c_int;
            }
            printf_raw($fmt.as_ptr() $($args)*)
        }};
    }

    #[macro_export]
    macro_rules! fprintf {
        ($stream:expr, $fmt:literal $($args:tt)*) => {{
            use core::ffi::c_int;
            extern "C" {
                #[link_name = "fprintf"]
                pub fn fprintf_raw(stream: *mut crate::crust::libc::FILE, fmt: *const c_char, ...) -> c_int;
            }
            fprintf_raw($stream, $fmt.as_ptr() $($args)*)
        }};
    }

    // count is the amount of items, not bytes
    pub unsafe fn realloc_items<T>(ptr: *mut T, count: usize) -> *mut T {
        extern "C" {
            #[link_name = "realloc"]
            fn realloc_raw(ptr: *mut c_void, size: usize) -> *mut c_void;
        }
        realloc_raw(ptr as *mut c_void, size_of::<T>()*count) as *mut T
    }

    pub unsafe fn free<T>(ptr: *mut T) {
        extern "C" {
            #[link_name = "free"]
            fn free_raw(ptr: *mut c_void);
        }
        free_raw(ptr as *mut c_void);
    }
}

#[panic_handler]
pub unsafe fn panic_handler(info: &PanicInfo) -> ! {
    // TODO: What's the best way to implement the panic handler within the Crust spirit
    //   PanicInfo must be passed by reference.
    if let Some(location) = info.location() {
        fprintf!(stderr, c"%.*s:%d: ", location.file().len(), location.file().as_ptr(), location.line());
    }
    fprintf!(stderr, c"panicked");
    if let Some(message) = info.message().as_str() {
        fprintf!(stderr, c": %.*s", message.len(), message.as_ptr());
    }
    fprintf!(stderr, c"\n");
    abort()
}

#[export_name="main"]
pub unsafe extern "C" fn crust_entry_point(argc: i32, argv: *mut*mut c_char) -> i32 {
    crate::main(argc, argv)
}
