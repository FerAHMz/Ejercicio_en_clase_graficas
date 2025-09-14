use raylib::prelude::*;
use crate::materials::Material;

// Función para aplicar iluminación avanzada basada en material
pub fn calculate_material_lighting(
    material: &Material,
    surface_position: Vector3,
    surface_normal: Vector3,
    light_position: Vector3,
    camera_position: Vector3,
    ambient_intensity: f32,
    diffuse_intensity: f32,
) -> Color {
    // Vector de la superficie hacia la luz
    let light_direction = (light_position - surface_position).normalized();
    
    // Vector hacia la cámara para reflexión especular
    let view_direction = (camera_position - surface_position).normalized();
    
    // Reflexión perfecta del vector de luz
    let reflect_direction = light_direction - surface_normal * (2.0 * surface_normal.dot(light_direction));
    
    // Calcular componentes de iluminación
    let diffuse_factor = surface_normal.dot(light_direction).max(0.0);
    let specular_factor = view_direction.dot(reflect_direction).max(0.0).powf(material.shininess);
    
    // Combinar componentes
    let ambient = ambient_intensity;
    let diffuse = diffuse_intensity * diffuse_factor;
    let specular = material.specular * specular_factor * diffuse_intensity;
    
    let total_lighting = (ambient + diffuse + specular).min(1.0);
    
    // Aplicar color del material
    Color::new(
        (material.albedo.x * 255.0 * total_lighting) as u8,
        (material.albedo.y * 255.0 * total_lighting) as u8,
        (material.albedo.z * 255.0 * total_lighting) as u8,
        ((1.0 - material.transparency) * 255.0) as u8,
    )
}