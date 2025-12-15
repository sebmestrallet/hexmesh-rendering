/// Compute the determinant of a 2x2 matrix
/// [[ a11 a12 ]
///    a21 a22 ]]
pub fn det2x2(a11: &f32, a12: &f32, a21: &f32, a22: &f32) -> f32 {
    a11*a22-a12*a21
}