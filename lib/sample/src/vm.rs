use move_vm_runtime::{
    move_vm::MoveVM,
    session::SerializedReturnValues,
};
use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    value::MoveValue, // value (aptos) instead of runtime_value (sui)
};
use move_vm_test_utils::{ BlankStorage };
use move_vm_types::gas::UnmeteredGasMeter;
use move_binary_format::file_format::CompiledModule;
use std::fs;

const TEST_ADDR: AccountAddress = AccountAddress::new([42; AccountAddress::LENGTH]);

pub fn run_module(file_path: &str, param: u64) -> u64 {
    let bytecode =
        fs::read(file_path).expect("Unable to read bytecode file");

    let module = CompiledModule::deserialize(&bytecode).expect("success"); // deserialize (aptos) instead of deserialize_with_defaults (sui)
        
    // function to call
    let fun_name = Identifier::new("fib").unwrap();

    let storage = BlankStorage::new();

    let vm = MoveVM::new(vec![]).unwrap();
    let mut sess = vm.new_session(&storage);
    sess.publish_module(bytecode, TEST_ADDR, &mut UnmeteredGasMeter).expect("module must load");

    let args: Vec<_> = vec![MoveValue::U64(param)]
        .into_iter()
        .map(|val| val.simple_serialize().unwrap())
        .collect();

    let SerializedReturnValues {
        return_values,
        mutable_reference_outputs: _,
    } = sess
    .execute_function_bypass_visibility(
        &module.self_id(),
        &fun_name,
        vec![],
        args,
        &mut UnmeteredGasMeter,
    ).unwrap();

    let result = MoveValue::simple_deserialize(&return_values[0].0, &return_values[0].1)
        .unwrap();

    let end;
    match result {
        MoveValue::U64(v) => {end = v;}
        _ => {end = 1;}
    }
    return end;
}

pub fn load_module(file_path: &str) {
    let bytecode =
        fs::read(file_path).expect("Unable to read bytecode file");

    let _module = CompiledModule::deserialize(&bytecode).expect("success"); // deserialize (aptos) instead of deserialize_with_defaults (sui)
        
    // function to call
    let _fun_name = Identifier::new("fib").unwrap();

    let storage = BlankStorage::new();

    let vm = MoveVM::new(vec![]).unwrap();
    let mut sess = vm.new_session(&storage);
    sess.publish_module(bytecode, TEST_ADDR, &mut UnmeteredGasMeter).expect("module must load");

    let _args: Vec<_> = vec![MoveValue::U64(14)]
        .into_iter()
        .map(|val| val.simple_serialize().unwrap())
        .collect();
}
