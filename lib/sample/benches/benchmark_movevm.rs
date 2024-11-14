use criterion::{black_box, Criterion, criterion_group, criterion_main};

use move_vm_runtime::{
    move_vm::MoveVM, 
    AsUnsyncModuleStorage,
    module_traversal::{TraversalContext, TraversalStorage},
    RuntimeEnvironment,
};
use move_core_types::{
    identifier::Identifier,
    value::{serialize_values, MoveValue}, // value (aptos) instead of runtime_value (sui)
};
use move_vm_test_utils::InMemoryStorage;
use move_vm_types::gas::UnmeteredGasMeter;
use move_binary_format::file_format::CompiledModule;
use std::{
    fs,
    env,
};

// use std::ffi::{CString, c_void};
// use std::os::raw::c_char;

// use sample::vm;

// extern "C" {
//     fn CreateComposite(
//         moveLoc: GoString, 
//         moveKind: u64, 
//         moveQualifiedIdentifier: GoString, 
//         moveAddress: GoString,
//     ) -> u64;

//     fn GetMember(key: u64, fieldName: GoString) -> GoInterface;

//     fn SetMember(key: u64, fieldName: GoString, value: *const c_void);
// }


// #[repr(C)]
// struct GoString {
//     a: *const c_char,
//     b: i64,
// }

// #[repr(C)]
// struct GoInterface {
//     t: *mut c_void,
//     v: *mut c_void,
// }

// fn create_go_string(c_str: &CString) -> GoString {
//     let ptr = c_str.as_ptr();
//     let go_string = GoString {
//         a: ptr,
//         b: c_str.as_bytes().len() as i64,
//     };
//     return go_string
// }

// pub fn test_composite_conversion() {
//     let c_iden = CString::new("foo").expect("CString::new failed");
//     let go_iden = create_go_string(&c_iden);
//     let c_addr = CString::new("0x1").expect("CString::new failed");
//     let go_addr = create_go_string(&c_addr);
//     let go_loc = create_go_string(&c_addr);
//     let c_fieldname = CString::new("a").expect("CString::new failed");
//     let go_fieldname = create_go_string(&c_fieldname);
//     let go_fieldname2 = create_go_string(&c_fieldname);
//     let c_val = CString::new("some random string").expect("CString::new failed");
//     let go_val = create_go_string(&c_val);
//     let go_ptr: *const GoString = &go_val;
//     let rawptr = go_ptr as *const c_void;

//     // foo {
//     //      a: "some random string"
//     // }
//     let tmp = unsafe{ CreateComposite(go_loc, 0, go_iden, go_addr) };
//     unsafe{ SetMember(tmp, go_fieldname, rawptr) };
//     let _result = unsafe { GetMember(tmp, go_fieldname2) };
// }

// pub fn bench_composite(c: &mut Criterion) {
//     c.bench_function("bench composite", |b| {
//         b.iter(||
//             test_composite_conversion()
//         )
//     });
// }

pub fn bench_recursive_fib(c: &mut Criterion) {
    bench_fib(c, "recur_fib");
}

pub fn bench_imperative_fib(c: &mut Criterion) {
    bench_fib(c, "imper_fib");
}

pub fn bench_fib(c: &mut Criterion, fun_name: &str) {
    let FILE_PATH: String = format!("{}/src/build/fib/bytecode_modules/M.mv", env::current_dir().expect("valid cwd").display());
    const PARAM: u64 = 14;

    let bytecode =
        fs::read(FILE_PATH).expect("Unable to read bytecode file");

    let module = CompiledModule::deserialize(&bytecode).expect("success"); // deserialize (aptos) instead of deserialize_with_defaults (sui)

    // function to call
    let fun = Identifier::new(fun_name).unwrap();

    // basic code cache
    let mut storage = InMemoryStorage::new();

    let runtime_environment = RuntimeEnvironment::new(vec![]);
    let vm = MoveVM::new_with_runtime_environment(&runtime_environment);

    let module_id = module.self_id();
    let mut module_blob = vec![];
    module.serialize(&mut module_blob).unwrap();

    storage.add_module_bytes(module_id.address(), module_id.name(), module_blob.into());
    let module_storage = storage.as_unsync_module_storage(runtime_environment);
    let mut sess = vm.new_session(&storage);

    let args = serialize_values(&vec![MoveValue::U64(black_box(PARAM))]);
    let traversal_storage = TraversalStorage::new();

    c.bench_function("bench fib 14", |b| {
        b.iter(||
            sess.execute_function_bypass_visibility(
                &module.self_id(),
                &fun,
                vec![],
                args.clone(),
                &mut UnmeteredGasMeter,
                &mut TraversalContext::new(&traversal_storage),
                &module_storage,
            ))
    });
}

criterion_group!(
    benches,
    bench_recursive_fib,
    bench_imperative_fib,
    bench_composite,
);

criterion_main!(benches,);
