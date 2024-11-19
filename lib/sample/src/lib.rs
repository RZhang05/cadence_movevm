use vm::{
    run_func_M,
    load_module,
    run_create_struct,
    run_composite,
};
use move_core_types::{
    value::{MoveValue, MoveStruct, MoveStructLayout}
};
use std::env;

pub mod vm;

#[no_mangle]
pub extern "C" fn test_composite_conversion() {
    run_composite();
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
