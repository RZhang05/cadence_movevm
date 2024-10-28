package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lsample
#include "./lib/sample.h"
#include <stdlib.h>
*/
import "C"
import _"unsafe"

func Fib(n int) int {
	return int(C.fib(C.int(n)))
}

func CadenceFib(n int) int {
	return int(C.cdcfib(C.int(n)))
}

func main() {
	println(C.fib(10))
	println(C.cdcfib(10))
}
