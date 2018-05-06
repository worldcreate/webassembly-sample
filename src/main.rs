mod sum;
use sum::fib;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    fib(40);
    println!("{} ms.", now.elapsed().subsec_nanos() / 1000 / 1000);
}

