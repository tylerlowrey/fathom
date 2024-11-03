use std::fmt::Debug;
use bevy::math::{Mat4, Vec3, Vec4};

fn main() {
    let identity_matrix: Mat4 = Mat4::IDENTITY;
    println!("Mat4 = {:?}", identity_matrix);
    let v0 = Vec4::new(1.0, 0.0, 0.0, 1.0);
    let v1 = Vec4::new(0.0, 1.0, 0.0, 1.0);
    let v2 = Vec4::new(0.0, 0.0, 1.0, 1.0);
    let v3 = Vec4::new(0.0, 0.0, 0.0, 1.0);
    let m1 = Mat4::from_cols(v0, v1, v2, v3);
    print_matrix(&m1);
    let scale = Mat4::from_scale(Vec3::new(0.5, 0.5, 0.5));
    print_matrix(&scale);
    let translation = Mat4::from_translation(Vec3::new(1.0, 0.5, 1.5));
    print_matrix(&translation);
    let rotation_x = Mat4::from_rotation_x(30.0);
    let rotation_y = Mat4::from_rotation_y(60.0);
    let transform = translation * rotation_x * rotation_y;
    println!();
    println!("TRANSFORM");
    println!("============");
    println!();
    print_matrix(&transform);
    println!();
    println!("INVERSE TRANSFORM");
    println!("============");
    println!();
    print_matrix(&transform.inverse());
    println!();

    let vec4 = Vec4::new(4.0, 2.0, 6.0, 1.0);
    let c1 = Vec4::new(3.0, 0.0, 0.0, 0.0);
    let c2 = Vec4::new(0.0, 5.0, 0.0, 0.0);
    let c3 = Vec4::new(0.0, 0.0, 7.0, 0.0);
    let c4 = Vec4::new(0.0, 0.0, 0.0, 9.0);
    let mat4 = Mat4::from_cols(c1, c2, c3, c4);
    let result = mat4 * vec4;
    println!("x = {:?}", result.x);
    println!("y = {:?}", result.y);
    println!("z = {:?}", result.z);
    println!("w = {:?}", result.w);
    println!()
}

fn print_matrix(matrix: &Mat4) {
    println!("x = {:?}", matrix.x_axis);
    println!("y = {:?}", matrix.y_axis);
    println!("z = {:?}", matrix.z_axis);
    println!("w = {:?}", matrix.w_axis);
}