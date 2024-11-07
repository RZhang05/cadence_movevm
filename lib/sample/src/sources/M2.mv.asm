// Move bytecode v7
module 2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a.M2 {
struct Foo {
	dummy_field: bool
}

public create_struct(): Foo /* def_idx: 0 */ {
B0:
	0: LdFalse
	1: Pack[0](Foo)
	2: Ret
}
public empty_fun() /* def_idx: 1 */ {
B0:
	0: Ret
}
}