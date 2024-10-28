# cadence_movevm

An investigation into using MoveVM within Go for the purpose of Cadence compilation.

## Benchmarks (current)

```
BenchmarkGoFib-22                1000000              1296 ns/op               0 B/op          0 allocs/op
BenchmarkRustFib-22              1498485               810.7 ns/op             0 B/op          0 allocs/op
BenchmarkCadenceFib-22              3189            352105 ns/op               0 B/op          0 allocs/op
```
Although instantiation of the VM and file reading is included above, it should be a small portion of the benchmark. Commenting out the invocation of the function yields only 3576ns/op. A useful comparison is the existing work with a vm on rust which is run again below (https://github.com/SupunS/cadence_vm/tree/rust_go_binding).

```
BenchmarkGoFib-22                 915919              1258 ns/op               0 B/op          0 allocs/op
BenchmarkRustFib-22              1560951               773.2 ns/op             0 B/op          0 allocs/op
BenchmarkCadenceFib-22             13777             86520 ns/op               0 B/op          0 allocs/op
```
