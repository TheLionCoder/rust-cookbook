use std::f64::consts::PI;

use num::Complex;

pub fn create_complex_numbers() {
    let complex_int: Complex<u32> = Complex::new(10, 20);
    let complex_float: Complex<f32> = Complex::new(10.1, 20.1);

    println!("Complex Integer: {}", complex_int);
    println!("Complex Float: {}", complex_float);
}

pub fn add_complex_numbers() {
    let complex_a: Complex<f32> = Complex::new(10.0, 20.);
    let complex_b: Complex<f32> = Complex::new(3.1, -4.2);

    let sum: Complex<f32> = complex_a + complex_b;

    println!("Sum: {}", sum)
}

pub fn calculate_complex_mathematical_functions() {
    let x : Complex<f64> = Complex::new(0., 2. * PI);

    eprintln!("e^(2i * pi) = {}", x.exp());

}