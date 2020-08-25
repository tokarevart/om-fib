use om_fib::*;

fn main() {
    let range = 0.0..1.0;
    let eps = 1e-4;
    let f = |x| x * x;
    let n = epsilon_to_n(range.clone(), eps);
    let n = n as usize;
    println!("  n : {}", n);

    let fibs = Fibs::new(n + 2);
    let x = search(range, fibs, n, f);
    println!("  x : {}", x);
    println!("f(x): {}", f(x));
}
