package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lsample
#include "./lib/sample.h"
#include <stdlib.h>
*/
import "C"
import _"unsafe"

func MoveVMFib(n int) int {
	return int(C.movevmfib(C.int(n)))
}

func MoveVMLoad() {
	C.movevmload()
}

func main() {
	println(C.fib(14))
	println(C.movevmfib(14))
}
