pub fn nth_fibo(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    else {
        return n * nth_fibo(n - 1);
    }
}