use bigint::uint::*;

fn main() {
    let n: u64 = 1;
    let x1: U512 = 1_u64 as U512;
    //let x2: U512 = U512::From(1);
    //let r: u64 = fib(n, x1, x2);
    //println!("DONE: {}", r);
}

// Slow method
fn fib(n: u64, x1: U512, x2: U512) -> u64 {
    let x3: U512 = x1 + x2;
    let n: u64 = n + 1;
    println!("x^{} = {}", n, x2 );
    if n == 1000 {
        return n;
    } else {
        return fib(n, x2, x3);
    }
}

fn fib_1(n: u32, x1: u64, x2: u64) -> u64 {
    let x3: u64 = x1 + x2;
    let n: u32 = n + 1;
    println!("x^{} = {}", n, x2 );
    if n == 14 {
        return x2
    } else {
        return fib(n, x2, x3);
    }
}

// Exponentiation by squaring for increased speed
fn fib_fast(n: u64) -> u64 {
    if n == 1 {
        // if n is 1
        return n;
    } else if (n % 2) == 0 {
        // if n is even
        let sn: u64 = u64::pow(n, 2);
        let hn: u32 = (n as u32) / 2;
        return u64::pow(sn, hn);
    } else {
        // if n is odd
        let sn: u64 = u64::pow(n, 2);
        let hn: u32 = ((n as u32) - 1) / 2;
        return n * u64::pow(sn, hn);
    }
}
