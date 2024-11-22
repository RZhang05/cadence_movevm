# cadence_movevm

An investigation into using MoveVM within Go for the purpose of Cadence compilation.

## Dependencies
This repo requires a local version of https://github.com/RZhang05/aptos-core/tree/composite-cadence in the same parent directory as this repo as specified in `lib/sample/Cargo.toml`.

## Build
This repo consists of a main Go library which calls into a nested Rust library which calls into a nested Go library. This means that there are some complications with linking and the build process. This was built on linux.

1. The innermost Go library cannot be static because this will cause multiple definition errors in the linker for cgo functions. 

### Mac

Changes required

- replace `libsample.so` with `libsample.dylib` in the `Makefile`
- potentially build `go_lib` first, the `Makefile` should handle this
- copy `libmain.dylib` to the root directory

## Benchmarks (current)

https://docs.google.com/spreadsheets/d/1YDnU5YkEs8il2slYY5la9HXc5xMfxzUJNEITadL5_iU/edit?usp=sharing

### Run the benchmarks
1. `git clone https://github.com/RZhang05/cadence_movevm.git`

### Go benchmarks
2. `cd cadence_movevm`
3. `make`

### Rust benchmarks
2. `cd cadence_movevm/lib/sample`
3. `cargo bench`
4. `cargo flamegraph -c "record -c 20 --call-graph dwarf -g"` (optional)