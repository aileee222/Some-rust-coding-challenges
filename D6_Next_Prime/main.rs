fn isPrime(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }

    let limit = (f64::from(n).sqrt() as u32) + 1;
    let mut i = 3;
    while i <= limit {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

fn NextPrime(number: u32) -> u32 {
    let mut count: u32 = number;
    while !isPrime(count) {
        count += 1;
    }
    return count;
}

fn main() {
    let number = 12;
    let number2 = 24;
    let number3 = 11;

    println!("{}", NextPrime(number));
    println!("{}", NextPrime(number2));
    println!("{}", NextPrime(number3));
}