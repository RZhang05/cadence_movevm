use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn echo(message: *const libc::c_char) {
    let message_cstr = unsafe { CStr::from_ptr(message) };
    let message = message_cstr.to_str().unwrap();
    println!("{}", message);
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

#[no_mangle]
pub extern "C" fn fib(a: libc::c_int) -> libc::c_int {
    return fibonacci(a as u32) as libc::c_int;
}

fn recursive_fib(n: u32) -> u32 {
    // TODO invoke movevm
    return 0
}

#[no_mangle]
pub extern "C" fn cdcfib(a: libc::c_int) -> libc::c_int {
    return recursive_fib(a as u32) as libc::c_int;
}
