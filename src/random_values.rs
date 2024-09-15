use rand_distr::{Distribution, Normal, NormalError, Standard};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rand::rngs::ThreadRng;

#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized> (&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y
        }
    }
}


pub(crate) fn generate_random_integers() {
    let mut rng: ThreadRng = thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);

}

pub(crate) fn generate_random_range_numbers() {
    let mut rng : ThreadRng = thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

pub(crate) fn generate_distributed_random_numbers() -> Result<(), NormalError> {
    let mut rng: ThreadRng = thread_rng();
    let normal: Normal<f64> = Normal::new(2.0, 3.0)?;
    let v: f64 = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}

pub(crate) fn generate_custom_random_numbers() {
   let mut rng: ThreadRng = thread_rng();
    let rand_tuple: (i32, bool,  f64) = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

pub(crate) fn create_random_password_alphanumeric() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("Random Password: {}", rand_string);
}

pub(crate) fn create_random_password_user_defined() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKacbde01234&%";
    const PASSWORD_LENGTH: usize = 30;

    let mut rng: ThreadRng = thread_rng();
    let password: String = (0..PASSWORD_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("Random Password: {}", password);
}