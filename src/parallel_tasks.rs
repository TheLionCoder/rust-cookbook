use ::std::fs::create_dir_all;
use core::default::Default;
use glob::{glob_with, MatchOptions};
use image::{imageops::FilterType, DynamicImage};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::error::Error;
use std::path::{Path, PathBuf};

struct Person {
    age: u8,
}

pub fn mutate_elements_in_parallel() {
    let mut arr: [i16; 4] = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr)
}

pub fn match_patterns_in_parallel() {
    let mut vec: Vec<i16> = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8));
    assert!(vec.par_iter().all(|n| *n <= 8));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8));
    assert!(!vec.par_iter().all(|n| *n <= 8));
}

pub fn search_items_given_predicate_parallel() {
    let v: Vec<i8> = vec![6, 2, 1, 9, 3, 8, 11];

    let f1: Option<&i8> = v.par_iter().find_any(|&&x| x == 9);
    let f2: Option<&i8> = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3: Option<&i8> = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}

pub fn sort_vector_in_parallel() {
    let mut vec: Vec<String> = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5).map(|_| rng.sample(&Alphanumeric) as char).collect()
    });
    vec.par_sort_unstable();
}

pub fn map_reduce_in_parallel() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];
    let num_over_30: f32 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u8 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);
    println!("The sum over 30 is {}", &sum_over_30);

    let alt_sum_30: u8 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    println!("The alt sum over 30 is {}", &alt_sum_30);

    let avg_over_30: f32 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30: f32 = alt_sum_30 as f32 / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < f32::EPSILON);
    eprintln!("The average age of people older than 30 is {}", avg_over_30);
}

pub fn generate_jpg_thumbnails() -> Result<(), Box<dyn Error>> {
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("./assets/images/*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        return Err("No .jgp files found".into());
    }

    let thumb_dir: &str = "./assets/thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| make_thumbnail(path, thumb_dir, 300).map_err(|_e| path.display().to_string()))
        .filter_map(Result::err)
        .collect();

    image_failures
        .iter()
        .for_each(|x| println!("Failed to process image {}", x));
    Ok(())
}

fn make_thumbnail<PA, PB>(
    original: PA,
    thumb_dir: PB,
    longest_edge: u32,
) -> Result<(), Box<dyn Error>>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img: DynamicImage = image::open(original.as_ref())?;
    let file_path: PathBuf = thumb_dir.as_ref().join(original);

    Ok(img
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}
