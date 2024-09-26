use num::bigint::{BigInt};

pub fn calculate_factorial(x: i32) -> BigInt {
    let mut factorial = BigInt::from(1);
        for i in 1..=x {
            factorial *= i;
        }
        factorial
}
