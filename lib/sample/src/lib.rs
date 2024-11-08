use vm::{
    run_func_M,
    load_module,
    run_create_struct,
};
use move_core_types::{
    value::{MoveValue, MoveStruct, MoveStructLayout}
};
use std::env;

use std::ffi::{CStr, CString, c_void};
use std::os::raw::c_char;
use std::any::{Any};

pub mod vm;

extern "C" {
    fn DoSomething(str: GoString) -> GoInterface;
}

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}

#[repr(C)]
struct GoInterface {
    t: *mut c_void,
    v: *mut c_void,
}

#[no_mangle]
pub extern "C" fn test() {
    let c_path = CString::new("test").expect("CString::new failed");
    let ptr = c_path.as_ptr();
    let go_string = GoString {
        a: ptr,
        b: c_path.as_bytes().len() as i64,
    };
    let msg = unsafe { DoSomething(go_string) };
    assert!(!msg.v.is_null());
    // to_string_lossy() returns a `Cow<str>`, but that's sufficient for printing.
    let cstr = unsafe {CStr::from_ptr(msg.v as *const _)}.to_string_lossy();
    println!("result: {}", cstr);
}

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
    return run_func_M(format!("{}/lib/sample/src/build/fib/bytecode_modules/M.mv", env::current_dir().expect("valid cwd").display()), "recur_fib", n);
}

#[no_mangle]
pub extern "C" fn moveRecFib(a: libc::c_int) -> libc::c_int {
    return recursive_fib(a as u64) as libc::c_int;
}

fn imperative_fib(n: u64) -> u64 {
    return run_func_M(format!("{}/lib/sample/src/build/fib/bytecode_modules/M.mv", env::current_dir().expect("valid cwd").display()), "imper_fib", n);
}

#[no_mangle]
pub extern "C" fn moveImpFib(a: libc::c_int) -> libc::c_int {
    return imperative_fib(a as u64) as libc::c_int;
}

#[no_mangle]
pub extern "C" fn movevmload() {
    return load_module(format!("{}/lib/sample/src/build/fib/bytecode_modules/M.mv", env::current_dir().expect("valid cwd").display()));
}

#[no_mangle]
pub extern "C" fn movevm_createstruct() {
    let result = run_create_struct();
    let end;
    match result {
        MoveValue::Struct(v) => {
            match v {
                MoveStruct::WithTypes { type_, mut fields } => {
                    end = fields;
                }
                _ => {},
            }
        }
        _ => {}
    }
    return;
}
