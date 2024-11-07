// Move bytecode v7
module 2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a.M {


public imper_fib(Arg0: u64): u64 /* def_idx: 0 */ {
L1:	loc0: u64
L2:	loc1: u64
L3:	loc2: u64
L4:	loc3: u64
B0:
	0: LdU64(1)
	1: StLoc[1](loc0: u64)
	2: LdU64(1)
	3: StLoc[2](loc1: u64)
	4: CopyLoc[1](loc0: u64)
	5: StLoc[3](loc2: u64)
	6: LdU64(2)
	7: StLoc[4](loc3: u64)
B1:
	8: CopyLoc[4](loc3: u64)
	9: CopyLoc[0](Arg0: u64)
	10: Lt
	11: BrFalse(26)
B2:
	12: Branch(13)
B3:
	13: MoveLoc[1](loc0: u64)
	14: CopyLoc[2](loc1: u64)
	15: Add
	16: StLoc[3](loc2: u64)
	17: MoveLoc[2](loc1: u64)
	18: StLoc[1](loc0: u64)
	19: CopyLoc[3](loc2: u64)
	20: StLoc[2](loc1: u64)
	21: MoveLoc[4](loc3: u64)
	22: LdU64(1)
	23: Add
	24: StLoc[4](loc3: u64)
	25: Branch(8)
B4:
	26: MoveLoc[3](loc2: u64)
	27: Ret
}
public recur_fib(Arg0: u64): u64 /* def_idx: 1 */ {
B0:
	0: CopyLoc[0](Arg0: u64)
	1: LdU64(2)
	2: Lt
	3: BrFalse(6)
B1:
	4: MoveLoc[0](Arg0: u64)
	5: Ret
B2:
	6: CopyLoc[0](Arg0: u64)
	7: LdU64(1)
	8: Sub
	9: Call recur_fib(u64): u64
	10: MoveLoc[0](Arg0: u64)
	11: LdU64(2)
	12: Sub
	13: Call recur_fib(u64): u64
	14: Add
	15: Ret
}
}