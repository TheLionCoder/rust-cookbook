extern crate nalgebra;
extern crate serde_json;

use std::error::Error;
use approx::assert_abs_diff_eq;
use nalgebra::{DMatrix, Matrix3};
use ndarray::{Array, arr1, Array1, arr2, Array2, ArrayView1, Ix1, ArrayBase, OwnedRepr, array};

pub fn add_matrices() {
    let a: Array2<u8> = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b: Array2<u8> = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum: Array2<u8> = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);
}

pub fn multiply_matrices() {
    let a: Array2<u8> = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b: Array2<u8> = arr2(&[[6, 3], [5, 2], [4, 1]]);

    println!("{}", a.dot(&b));
}

pub fn multiply_scalar() {
    let scalar: u8 = 4;

    let vector: Array1<u8> = arr1(&[1, 2, 3]);

    let matrix: Array2<u8> = arr2(&[[4, 5, 6], [7, 8, 9]]);

    let new_vector: Array1<u8> = vector * scalar;
    println!("new vector: {}", new_vector);

    let new_matrix: Array<u8, Ix1> = matrix.dot(&new_vector);
    println!("new matrix: {}", new_matrix);

}

pub fn compare_vectors() {
    let a: ArrayBase<OwnedRepr<f64>, Ix1>= Array::from(vec![1., 2., 3., 4., 5.]);
    let b: ArrayBase<OwnedRepr<f64>, Ix1> = Array::from(vec![5., 4., 3. , 2., 1.]);
    let mut c: ArrayBase<OwnedRepr<f64>, Ix1> = Array::from(vec![1., 2., 3., 4., 5.]);
    let d: ArrayBase<OwnedRepr<f64>, Ix1> = Array::from(vec![5., 4., 3., 2., 1.]);

    let z:ArrayBase<OwnedRepr<f64>, Ix1> = a + b;
    let w: ArrayBase<OwnedRepr<f64>, Ix1> = &c + &d;

    assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

    println!("c: {}", c);
    c[0] = 10.;
    c[1] = 10.;

    print!("Mutated c: {} \n", c);

    assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.,]));
}

pub fn calculate_vector_norm() {
    let x: ArrayBase<OwnedRepr<f64>, Ix1> = array![1., 2., 3., 4., 5.,];
    println!("||x||_2 = {}", calculate_l2_norm(x.view()));
    println!("||x||_1 = {}", calculate_l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize_vector(x));
}

pub fn invert_matrix() {
    let m1: Matrix3<f64> = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0 );
    println!("Matrix m1: {}", m1);

    match m1.try_inverse() {
        Some(inverse) => println!("The inverse Matrix of m1 is {}", inverse),
        None => println!("Matrix m1 is not invertible.")
    }
}

pub fn des_serialize_matrix() -> Result<(), Box<dyn Error>> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix= DMatrix::from_row_slice(50, 100, &row_slice);

    let serialize_matrix = serde_json::to_string(&matrix)?;

    let deserialize_matrix: DMatrix<i32> = serde_json::from_str(&serialize_matrix)?;

    assert_eq!(deserialize_matrix, matrix);

    Ok(())
}

// Auxiliary functions
fn calculate_l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc: f64, elem:&f64| acc + elem.abs())
}

fn calculate_l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize_vector(mut x: Array1<f64>) -> Array1<f64> {
    let norm: f64 = calculate_l2_norm(x.view());
    x.mapv_inplace(|elem: f64| elem / norm);
    x
}
