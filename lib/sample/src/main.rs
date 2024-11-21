mod vm;

use vm::{
    run_func_M,
    create_move_composite,
};
use std::env;

fn main() {
    let n = 14;
    run_func_M(format!("{}/src/build/fib/bytecode_modules/M.mv", env::current_dir().expect("valid cwd").display()), "recur_fib", n);
}


