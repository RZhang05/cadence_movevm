package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lsample
#include "./lib/sample.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

func main() {
	str1 := C.CString("begin test")
	defer C.free(unsafe.Pointer(str1))

	C.echo(str1)
	println(C.fib(10))
	println(C.cdcfib(10))
}