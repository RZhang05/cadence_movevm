use criterion::{black_box, Criterion, criterion_group, criterion_main};

use move_vm_runtime::{
    move_vm::MoveVM, 
    AsUnsyncModuleStorage,
    module_traversal::{TraversalContext, TraversalStorage},
    RuntimeEnvironment,
};
use move_core_types::{
    account_address::AccountAddress,
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
use move_stdlib::natives;

pub fn bench_composite(c: &mut Criterion) {
    let FILE_PATH: String = format!("{}/src/build/fib/bytecode_modules/M2.mv", env::current_dir().expect("valid cwd").display());
    
    let bytecode =
        fs::read(FILE_PATH).expect("Unable to read bytecode file");

    let module = CompiledModule::deserialize(&bytecode).expect("success"); // deserialize (aptos) instead of deserialize_with_defaults (sui)

    // function to call
    let fun = Identifier::new("test_full_composite").unwrap();

    // basic code cache
    let mut storage = InMemoryStorage::new();

    let natives = move_stdlib::natives::all_natives(
        AccountAddress::from_hex_literal("0x1").unwrap(),
        move_stdlib::natives::GasParameters::zeros(),
    );
    let runtime_environment = RuntimeEnvironment::new(natives);
    let vm = MoveVM::new_with_runtime_environment(&runtime_environment);

    let module_id = module.self_id();
    let mut module_blob = vec![];
    module.serialize(&mut module_blob).unwrap();

    storage.add_module_bytes(module_id.address(), module_id.name(), module_blob.into());

    // add native dependencies, this should not be a factor on performance in reality, but may significantly impact performance here
    let paths = fs::read_dir(format!("{}/src/build/fib/bytecode_modules/dependencies/MoveStdlib/", env::current_dir().expect("valid cwd").display())).unwrap();

    for path in paths {
        let native_bytecode =
        fs::read(path.unwrap().path()).expect("Unable to read bytecode file");
    
        let native_module = CompiledModule::deserialize(&native_bytecode).expect("success");

        let native_module_id = native_module.self_id();
        let mut native_blob = vec![];
        native_module.serialize(&mut native_blob).unwrap();

        storage.add_module_bytes(native_module_id.address(), native_module_id.name(), native_blob.into());
    }

    let module_storage = storage.as_unsync_module_storage(runtime_environment);
    let mut sess = vm.new_session(&storage);

    let args = serialize_values(&vec![]);
    let traversal_storage = TraversalStorage::new();

    c.bench_function("bench composite", |b| {
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
