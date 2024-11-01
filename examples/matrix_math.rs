use std::fmt::Debug;
use cgmath::{Matrix4, SquareMatrix, Vector3, Vector4};

fn main() {
    let identity_matrix: Matrix4<f32> = Matrix4::identity();
    println!("Matrix4 = {:?}", identity_matrix);

    let v0 = Vector4::new(1, 0, 0, 1);
    let v1 = Vector4::new(0, 1, 0, 1);
    let v2 = Vector4::new(0, 0, 1, 1);
    let v3 = Vector4::new(0, 0, 0, 1);
    let m1 = Matrix4::from_cols(v0, v1, v2, v3);
    print_matrix(&m1);
    let scale = Matrix4::from_scale(0.5);
    print_matrix(&scale);
    let translation = Matrix4::from_translation(Vector3::new(1.0, 0.5, 1.5));
    print_matrix(&translation);

}

fn print_matrix<T: Debug>(matrix: &Matrix4<T>) {
    println!("x = {:?}", matrix.x);
    println!("y = {:?}", matrix.y);
    println!("z = {:?}", matrix.z);
    println!("w = {:?}", matrix.w);
}