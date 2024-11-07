use criterion::{black_box, Criterion, criterion_group, criterion_main};

use move_vm_runtime::{move_vm::MoveVM, AsUnsyncModuleStorage};
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
use move_vm_runtime::module_traversal::{TraversalContext, TraversalStorage};
use move_vm_runtime::RuntimeEnvironment;

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
    let fun_name = Identifier::new(fun_name).unwrap();

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
                &fun_name,
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
);

criterion_main!(benches,);
