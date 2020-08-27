use om_fib::*;
use std::f64::consts::FRAC_PI_2;

fn main() {
    let f = |x: f64| x * x;
    let range = 0.0..1.0;
    let eps = 1e-4;
    let x = search(range, eps, f);
    println!("x  : {}", x);
    println!("x^2: {}", f(x));
    println!("");

    let f = |x: f64| -x.cos();
    let range = -FRAC_PI_2..FRAC_PI_2;
    let eps = 1e-4;
    let x = search(range, eps, f);
    println!("x      : {}", x);
    println!("-cos(x): {}", f(x));
    println!("");

    let f = |x: f64| x.exp();
    let range = -1.0..1.0;
    let eps = 1e-4;
    let x = search(range, eps, f);
    println!("x  : {}", x);
    println!("e^x: {}", f(x));
    println!("");
}
