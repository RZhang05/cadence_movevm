package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is sometimes necessary to fix linker errors about `dlsym`.

// WINDOWS SPECIFIC:
// https://github.com/rust-lang/rust/issues/117121

/*
#cgo LDFLAGS: -L./lib/ -lhello
#include "./lib/hello.h"
*/
import "C"

func main() {
	C.hello(C.CString("world"))
	C.whisper(C.CString("this is code from the static library"))
}
