use std::ffi::CStr;
use vm::run_module;

pub mod vm;

fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

#[no_mangle]
pub extern "C" fn fib(a: libc::c_int) -> libc::c_int {
    return fibonacci(a as u64) as libc::c_int;
}

fn recursive_fib(n: u64) -> u64 {
    return run_module("/home/dev/work/cadence_movevm/lib/sample/src/build/fib/bytecode_modules/M.mv", n);
}

#[no_mangle]
pub extern "C" fn cdcfib(a: libc::c_int) -> libc::c_int {
    return recursive_fib(a as u64) as libc::c_int;
}
