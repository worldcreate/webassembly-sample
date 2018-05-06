

#[no_mangle]
pub fn fib(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 1,
        n => fib(n - 1) + fib(n - 2)
    }
}