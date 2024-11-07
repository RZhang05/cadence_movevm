module 0x2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a::M {
    public fun recur_fib(value: u64): u64 {
        if (value < 2) {
            return value
        };
        recur_fib(value-1) + recur_fib(value-2)
    }

    public fun imper_fib(value: u64): u64 {
        let fib1 = 1;
        let fib2 = 1;
        let fibo = fib1;
        let i = 2;
        while (i < value) {
            fibo = fib1 + fib2;
            fib1 = fib2;
            fib2 = fibo;
            i = i + 1;
        };
        return fibo
    }
}
