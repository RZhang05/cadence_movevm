// NOTE: You could use https://michael-f-bryan.github.io/rust-ffi-guide/cbindgen.html to generate
// this header automatically from your Rust code.  But for now, we'll just write it by hand.

typedef struct {
} Foo;

int fib(int a);
int moveRecFib(int a);
int moveImpFib(int a);
void movevmload();
Foo movevm_createstruct();
void test_composite_conversion();