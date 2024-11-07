use move_vm_runtime::{
    move_vm::MoveVM, 
    AsUnsyncModuleStorage,
    module_traversal::{TraversalContext, TraversalStorage},
    RuntimeEnvironment,
    session::SerializedReturnValues,
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

pub fn run_func_M(file_path: String, fun_name: &str, param: u64) -> u64 {
    let bytecode =
        fs::read(file_path).expect("Unable to read bytecode file");

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

    let args = serialize_values(&vec![MoveValue::U64(param)]);
    let traversal_storage = TraversalStorage::new();

    let SerializedReturnValues {
        return_values,
        mutable_reference_outputs: _,
    } = sess.
    execute_function_bypass_visibility(
        &module.self_id(),
        &fun,
        vec![],
        args.clone(),
        &mut UnmeteredGasMeter,
        &mut TraversalContext::new(&traversal_storage),
        &module_storage,
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

pub fn load_module(file_path: String) {
    let bytecode =
    fs::read(file_path).expect("Unable to read bytecode file");

    let module = CompiledModule::deserialize(&bytecode).expect("success"); // deserialize (aptos) instead of deserialize_with_defaults (sui)
    let param = 14;

    // function to call
    let _fun_name = Identifier::new("test").unwrap();

    // basic code cache
    let mut storage = InMemoryStorage::new();

    let runtime_environment = RuntimeEnvironment::new(vec![]);
    let vm = MoveVM::new_with_runtime_environment(&runtime_environment);

    let module_id = module.self_id();
    let mut module_blob = vec![];
    module.serialize(&mut module_blob).unwrap();

    storage.add_module_bytes(module_id.address(), module_id.name(), module_blob.into());
    let _module_storage = storage.as_unsync_module_storage(runtime_environment);
    let mut _sess = vm.new_session(&storage);

    let _args = serialize_values(&vec![MoveValue::U64(param)]);
    let _traversal_storage = TraversalStorage::new();
}

pub fn run_create_struct() -> MoveValue {
    let FILE_PATH: String = format!("{}/lib/sample/src/build/fib/bytecode_modules/M2.mv", env::current_dir().expect("valid cwd").display());
    
    let bytecode =
        fs::read(FILE_PATH).expect("Unable to read bytecode file");

    let module = CompiledModule::deserialize(&bytecode).expect("success"); // deserialize (aptos) instead of deserialize_with_defaults (sui)

    // function to call
    let fun = Identifier::new("create_struct").unwrap();

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

    let args = serialize_values(&vec![]);
    let traversal_storage = TraversalStorage::new();

    let SerializedReturnValues {
        return_values,
        mutable_reference_outputs: _,
    } = sess.
    execute_function_bypass_visibility(
        &module.self_id(),
        &fun,
        vec![],
        args.clone(),
        &mut UnmeteredGasMeter,
        &mut TraversalContext::new(&traversal_storage),
        &module_storage,
    ).unwrap();

    let result = MoveValue::simple_deserialize(&return_values[0].0, &return_values[0].1)
        .unwrap();

    return result;
}
