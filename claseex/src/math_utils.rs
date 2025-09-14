use raylib::prelude::*;

// Función para rotar un vector en 3D
pub fn rotate_vector(v: Vector3, angle_x: f32, angle_y: f32, angle_z: f32) -> Vector3 {
    let mut result = v;
    
    // Rotación alrededor del eje Y
    let cos_y = angle_y.cos();
    let sin_y = angle_y.sin();
    let temp_x = result.x * cos_y - result.z * sin_y;
    let temp_z = result.x * sin_y + result.z * cos_y;
    result.x = temp_x;
    result.z = temp_z;
    
    // Rotación alrededor del eje X
    let cos_x = angle_x.cos();
    let sin_x = angle_x.sin();
    let temp_y = result.y * cos_x - result.z * sin_x;
    let temp_z2 = result.y * sin_x + result.z * cos_x;
    result.y = temp_y;
    result.z = temp_z2;
    
    // Rotación alrededor del eje Z
    let cos_z = angle_z.cos();
    let sin_z = angle_z.sin();
    let temp_x2 = result.x * cos_z - result.y * sin_z;
    let temp_y2 = result.x * sin_z + result.y * cos_z;
    result.x = temp_x2;
    result.y = temp_y2;
    
    result
}