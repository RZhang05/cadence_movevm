use criterion::{black_box, Criterion, criterion_group, criterion_main};

use move_vm_runtime::move_vm::MoveVM;
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

const TEST_ADDR: AccountAddress = AccountAddress::new([42; AccountAddress::LENGTH]);

pub fn bench_recursive_fib(c: &mut Criterion) {
    let FILE_PATH: String = format!("{}/src/build/fib/bytecode_modules/M.mv", env::current_dir().expect("valid cwd").display());
    const PARAM: u64 = 14;

    let bytecode =
        fs::read(FILE_PATH).expect("Unable to read bytecode file");

    let module = CompiledModule::deserialize(&bytecode).expect("success"); // deserialize (aptos) instead of deserialize_with_defaults (sui)
        
    // function to call
    let fun_name = Identifier::new("recur_fib").unwrap();

    // basic code cache
    let mut storage = InMemoryStorage::new();

    let vm = MoveVM::new(vec![]).unwrap();
    let mut sess = vm.new_session(&storage);
    sess.publish_module(bytecode, TEST_ADDR, &mut UnmeteredGasMeter).expect("module must load");

    let (changeset, _) = sess.finish().expect("failure getting write set");

    storage.apply(changeset).expect("failure applying write set");

    sess = vm.new_session(&storage);

    let args = serialize_values(&vec![MoveValue::U64(black_box(PARAM))]);

    c.bench_function("bench recursive fib 14", |b| {
        b.iter(|| sess.execute_function_bypass_visibility(
            &module.self_id(),
            &fun_name,
            vec![],
            args.clone(),
            &mut UnmeteredGasMeter,
        ))
    });

}

pub fn bench_imperative_fib(c: &mut Criterion) {
    const FILE_PATH: &str = "/home/dev/work/cadence_movevm/lib/sample/src/build/fib/bytecode_modules/M.mv";
    const PARAM: u64 = 14;

    let bytecode =
        fs::read(FILE_PATH).expect("Unable to read bytecode file");

    let module = CompiledModule::deserialize(&bytecode).expect("success"); // deserialize (aptos) instead of deserialize_with_defaults (sui)
        
    // function to call
    let fun_name = Identifier::new("imper_fib").unwrap();

    let mut storage = InMemoryStorage::new();

    let vm = MoveVM::new(vec![]).unwrap();
    let mut sess = vm.new_session(&storage);
    sess.publish_module(bytecode, TEST_ADDR, &mut UnmeteredGasMeter).expect("module must load");

    let (changeset, _) = sess.finish().expect("failure getting write set");

    storage.apply(changeset).expect("failure applying write set");

    sess = vm.new_session(&storage);

    let args = serialize_values(&vec![MoveValue::U64(black_box(PARAM))]);

    c.bench_function("bench imperative fib 14", |b| {
        b.iter(|| sess.execute_function_bypass_visibility(
            &module.self_id(),
            &fun_name,
            vec![],
            args.clone(),
            &mut UnmeteredGasMeter,
        ))
    });

}

criterion_group!(
    benches,
    bench_recursive_fib,
    bench_imperative_fib,
);

criterion_main!(benches,);
