extern crate nalgebra as na;

use alga::linear::Transformation;
use na::{Id, Isometry3, Point3, Vector3};

/*
 * Applies `n` times the transformation `t` to the vector `v` and sum each
 * intermediate value.
 */
fn complicated_algorithm<T>(v: &Vector3<f32>, t: &T, n: usize) -> Vector3<f32>
where T: Transformation<Point3<f32>> {
    let mut result = *v;

    // Do lots of operations involving t.
    for _ in 0..n {
        result = v + t.transform_vector(&result);
    }

    result
}

/*
 * The two following calls are equivalent in term of result.
 */
fn main() {
    let v = Vector3::new(1.0, 2.0, 3.0);

    // The specialization generated by the compiler will do vector additions only.
    let result1 = complicated_algorithm(&v, &Id::new(), 100000);

    // The specialization generated by the compiler will also include matrix multiplications.
    let iso = Isometry3::identity();
    let result2 = complicated_algorithm(&v, &iso, 100000);

    // They both return the same result.
    assert!(result1 == Vector3::new(100001.0, 200002.0, 300003.0));
    assert!(result2 == Vector3::new(100001.0, 200002.0, 300003.0));
}
