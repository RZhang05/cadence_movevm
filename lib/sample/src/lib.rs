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

pub mod vm;

extern "C" {
    fn DoSomething(str: GoString) -> GoInterface;

    fn CreateComposite(
        moveLoc: GoString, 
        moveKind: u64, 
        moveQualifiedIdentifier: GoString, 
        moveAddress: GoString,
    ) -> u64;

    fn GetMember(key: u64, fieldName: GoString) -> GoInterface;

    fn EmptyFunc();

    fn SetMember(key: u64, fieldName: GoString, value: *const c_void);
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

fn create_go_string(c_str: &CString) -> GoString {
    let ptr = c_str.as_ptr();
    let go_string = GoString {
        a: ptr,
        b: c_str.as_bytes().len() as i64,
    };
    return go_string
}

#[no_mangle]
pub extern "C" fn test_composite_conversion() {
    // clean up?
    let c_iden = CString::new("foo").expect("CString::new failed");
    let go_iden = create_go_string(&c_iden);
    let c_addr = CString::new("0x1").expect("CString::new failed");
    let go_addr = create_go_string(&c_addr);
    let go_loc = create_go_string(&c_addr);
    let c_fieldname = CString::new("a").expect("CString::new failed");
    let go_fieldname = create_go_string(&c_fieldname);
    let go_fieldname2 = create_go_string(&c_fieldname);
    let c_val = CString::new("some random string").expect("CString::new failed");
    let go_val = create_go_string(&c_val);
    let go_ptr: *const GoString = &go_val;
    let rawptr = go_ptr as *const c_void;

    // foo {
    //      a: "some random string"
    // }
    let tmp = unsafe{ CreateComposite(go_loc, 0, go_iden, go_addr) };
    unsafe{ SetMember(tmp, go_fieldname, rawptr) };
    let result = unsafe { GetMember(tmp, go_fieldname2) };
    
    // assert!(!result.v.is_null());
    // // to_string_lossy() returns a `Cow<str>`, but that's sufficient for printing.
    // let cstr = unsafe {CStr::from_ptr(result.v as *const _)}.to_string_lossy();
    // println!("result: {}", cstr);
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

#[no_mangle]
pub extern "C" fn moveEmptyExtern() {
    unsafe{ EmptyFunc() };
}
