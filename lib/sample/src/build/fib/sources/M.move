module 0x42::M {
    public fun fib(value: u64): u64 {
        if (value < 2) {
            return value
        };
        fib(value-1) + fib(value-2)
    }
}
