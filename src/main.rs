fn main() {
    println!("Fibonacci Sequence Generator!");
    
    loop {
        println!("Calculate the sequence to n:");
        let mut n = String::new();

        std::io::stdin().read_line(&mut n).expect("No n given");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let mut fib_seq = String::new();
        let start_fast = std::time::Instant::now();
        for number in 0..n {
            fib_seq = format!("{} {}", fib_seq, fast_fib(number));
        }
        let sec_fast = start_fast.elapsed().as_micros();
        println!("Fast calculation took {} us", sec_fast);
        println!("The fibonacci squence up to n={} is: {}", n, fib_seq);
        let mut fib_seq_slow = String::new();
        let start_slow = std::time::Instant::now();
        for number in 0..n {
            fib_seq_slow = format!("{} {}", fib_seq_slow, fib(number));
        }
        let sec_slow = start_slow.elapsed().as_micros();
        println!("Slow calculation took {} us", sec_slow);
        println!("The (slow) fibonacci squence up to n={} is: {}", n, fib_seq_slow);
    }
}

fn fib(n: u32) -> u32 {
    let fib_num: u32;
    if n == 0 {
        fib_num = 0;
    } else if n == 1 {
        fib_num = 1;
    } else {
        fib_num = fib(n - 2) + fib(n - 1);
    };
    fib_num
}

fn fast_fib(n: u32) -> u32 {
    _fib(n).0
}

fn _fib(n: u32) -> (u32, u32) {
    let fib_tup: (u32, u32);
    if n == 0 {
        fib_tup = (0, 1);
    } else {
        let (a, b) = _fib(n / 2);
        let c = a * (b * 2 - a);
        let d = a * a + b * b;
        if n % 2 == 0 {
            fib_tup = (c, d);
        } else {
            fib_tup = (d, c + d);
        };
    }
    fib_tup
}