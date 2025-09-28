use raylib::prelude::*;
use rayon::prelude::*;
use crate::materials::{Material, TextureSampler};
use crate::ray::{Ray, HitRecord, HittableList, Hittable};
use crate::skybox::Skybox;

pub struct RayTracer {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub skybox: Skybox,
}

impl RayTracer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            samples_per_pixel: 2, // Muy reducido para interacción fluida
            max_depth: 6,         // Reducido significativamente para performance
            skybox: Skybox::new(),
        }
    }

    /// Crear raytracer con calidad específica
    pub fn with_quality(width: u32, height: u32, quality_level: u8) -> Self {
        let (samples, depth) = match quality_level {
            1 => (1, 4),   // Calidad mínima - máxima velocidad
            2 => (2, 6),   // Calidad baja - buena velocidad
            3 => (4, 8),   // Calidad media - velocidad moderada
            4 => (8, 10),  // Alta calidad - velocidad reducida
            _ => (2, 6),   // Por defecto
        };

        Self {
            width,
            height,
            samples_per_pixel: samples,
            max_depth: depth,
            skybox: Skybox::new(),
        }
    }

    pub fn render(
        &self,
        scene: &HittableList,
        materials: &[Material],
        texture_sampler: &TextureSampler,
        camera_pos: Vector3,
        camera_target: Vector3,
        camera_up: Vector3,
        fov: f32,
    ) -> Vec<Color> {
        let aspect_ratio = self.width as f32 / self.height as f32;
        let viewport_height = 2.0 * (fov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (camera_pos - camera_target).normalized();
        let u = camera_up.cross(w).normalized();
        let v = w.cross(u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = camera_pos - horizontal * 0.5 - vertical * 0.5 - w;

        // Optimización con paralelización mejorada - paralelizar por chunks de pixels
        let total_pixels = self.width * self.height;
        let chunk_size = (total_pixels as usize / rayon::current_num_threads()).max(64); // Chunks adaptativos
        
        let pixels: Vec<Color> = (0..total_pixels)
            .into_par_iter()
            .with_min_len(chunk_size) // Optimización: chunks más grandes
            .map(|pixel_index| {
                let i = pixel_index % self.width;
                let j = pixel_index / self.width;
                
                let mut color = Vector3::zero();
                
                // Anti-aliasing con samples reducidos para mejor performance
                let samples = if self.samples_per_pixel > 4 { 4 } else { self.samples_per_pixel };
                for _ in 0..samples {
                    let u_offset = (i as f32 + fastrand::f32()) / self.width as f32;
                    let v_offset = ((self.height - 1 - j) as f32 + fastrand::f32()) / self.height as f32;
                    
                    let ray_direction = lower_left_corner + horizontal * u_offset + vertical * v_offset - camera_pos;
                    let ray = Ray::new(camera_pos, ray_direction);
                    
                    color = color + self.ray_color(&ray, scene, materials, texture_sampler, self.max_depth);
                }
                
                color = color / samples as f32;
                
                // Gamma correction optimizada
                let sqrt_x = color.x.sqrt();
                let sqrt_y = color.y.sqrt();
                let sqrt_z = color.z.sqrt();
                
                // Clamp y convertir a Color en una sola operación
                Color::new(
                    (sqrt_x.clamp(0.0, 1.0) * 255.0) as u8,
                    (sqrt_y.clamp(0.0, 1.0) * 255.0) as u8,
                    (sqrt_z.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                )
            })
            .collect();

        pixels
    }

    fn ray_color(
        &self,
        ray: &Ray,
        scene: &HittableList,
        materials: &[Material],
        texture_sampler: &TextureSampler,
        depth: u32,
    ) -> Vector3 {
        // Reducir profundidad máxima para mejor performance
        if depth == 0 || depth > 8 {
            return Vector3::zero();
        }

        let mut rec = HitRecord::new();
        
        // Early termination con probabilidad para rayos profundos
        if depth < 3 && fastrand::f32() < 0.1 {
            return Vector3::zero();
        }
        
        if scene.hit(ray, 0.001, f32::INFINITY, &mut rec) {
            let material = &materials[rec.material_index];
            
            // Añadir emisión del material
            let mut color = material.emission;
            
            let mut attenuation = Vector3::zero();
            let mut scattered = Ray::new(Vector3::zero(), Vector3::zero());
            
            if material.scatter(ray, &rec, &mut attenuation, &mut scattered, texture_sampler) {
                // Optimización: reducir multiplicaciones recursivas
                let scattered_color = self.ray_color(&scattered, scene, materials, texture_sampler, depth - 1);
                color = color + Vector3::new(
                    attenuation.x * scattered_color.x,
                    attenuation.y * scattered_color.y,
                    attenuation.z * scattered_color.z,
                );
            }
            
            return color;
        }

        // Color del skybox si no hay intersección (usando texturas)
        let skybox_color = self.skybox.sample(ray.direction, texture_sampler);
        Vector3::new(
            skybox_color.r as f32 / 255.0,
            skybox_color.g as f32 / 255.0,
            skybox_color.b as f32 / 255.0,
        )
    }
}

// Función legacy mantenida para compatibilidad
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