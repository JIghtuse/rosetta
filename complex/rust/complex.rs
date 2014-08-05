extern crate num;
use num::complex::Complex;

fn main() {
    let a = Complex{re: -4.0f32, im: 5.0f32};
    let b = Complex{re: 1.0f32, im: 1.0f32};

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a + b);
    println!("a * b = {}", a * b);
    println!("1 / a = {}", Complex{re: 1.0, im: 0.0} / a);
    println!("-a = {}", -a);
    println!("conj a = {}", a.conj());
}
