use raylib::prelude::*;
use crate::ray::{Ray, HitRecord};
use std::collections::HashMap;
use image::{DynamicImage, GenericImageView};

const EPS: f32 = 1e-4;

#[inline]
fn spawn_ray(point: Vector3, normal: Vector3, dir: Vector3) -> Ray {
    // rec.normal SIEMPRE apunta contra el rayo incidente (por set_face_normal)
    // Si el nuevo rayo va en el MISMO semiespacio que 'normal', empuja en +normal; si no, en -normal.
    let sign = if dir.dot(normal) > 0.0 { 1.0 } else { -1.0 };
    Ray::new(point + normal * (EPS * sign), dir)
}

#[derive(Clone)]
pub struct Material {
    pub texture: String,
    pub texture_top: Option<String>,    // Textura para cara superior (opcional)
    pub texture_bottom: Option<String>, // Textura para cara inferior (opcional)
    pub texture_sides: Option<String>,  // Textura para caras laterales (opcional)
    pub albedo: Vector3,           // Color base (RGB 0-1)
    pub specular: f32,            // Reflectividad especular (0-1)
    pub transparency: f32,        // Transparencia (0=opaco, 1=transparente)
    pub reflectivity: f32,        // Reflectividad (0-1)
    pub refraction_index: f32,    // Índice de refracción
    pub shininess: f32,           // Brillo especular
    pub emission: Vector3,        // Color de emisión para materiales que brillan
}

// Estructura para samplear texturas PNG reales en raytracing
pub struct TextureSampler {
    pub textures: HashMap<String, DynamicImage>,
}

impl Material {
    pub fn new(texture: String, albedo: Vector3, specular: f32, transparency: f32, reflectivity: f32, refraction_index: f32, shininess: f32) -> Self {
        Self {
            texture,
            texture_top: None,
            texture_bottom: None,
            texture_sides: None,
            albedo,
            specular,
            transparency,
            reflectivity,
            refraction_index,
            shininess,
            emission: Vector3::zero(),
        }
    }

    pub fn with_emission(mut self, emission: Vector3) -> Self {
        self.emission = emission;
        self
    }

    // Método para crear materiales con texturas por cara
    pub fn with_face_textures(
        main_texture: String, 
        top_texture: Option<String>,
        bottom_texture: Option<String>, 
        sides_texture: Option<String>,
        albedo: Vector3, 
        specular: f32, 
        transparency: f32, 
        reflectivity: f32, 
        refraction_index: f32, 
        shininess: f32
    ) -> Self {
        Self {
            texture: main_texture,
            texture_top: top_texture,
            texture_bottom: bottom_texture,
            texture_sides: sides_texture,
            albedo,
            specular,
            transparency,
            reflectivity,
            refraction_index,
            shininess,
            emission: Vector3::zero(),
        }
    }

    // Función para obtener la textura apropiada según la normal de la superficie
    pub fn get_texture_for_normal(&self, normal: Vector3) -> &String {
        // Determinar qué cara del cubo basándose en la normal
        let abs_normal = Vector3::new(normal.x.abs(), normal.y.abs(), normal.z.abs());
        
        if abs_normal.y > abs_normal.x && abs_normal.y > abs_normal.z {
            // Normal principalmente en Y
            if normal.y > 0.0 {
                // Cara superior (+Y)
                self.texture_top.as_ref().unwrap_or(&self.texture)
            } else {
                // Cara inferior (-Y)
                self.texture_bottom.as_ref().unwrap_or(&self.texture)
            }
        } else {
            // Caras laterales (X o Z dominante)
            self.texture_sides.as_ref().unwrap_or(&self.texture)
        }
    }

    // Función para calcular el scatter de un rayo con textura
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray, texture_sampler: &TextureSampler) -> bool {
        // Si el material es principalmente reflectivo
        if self.reflectivity > 0.5 && self.transparency < 0.5 {
            self.reflect(ray_in, rec, attenuation, scattered, texture_sampler)
        } 
        // Si el material es principalmente transparente (refracción)
        else if self.transparency > 0.5 {
            self.refract(ray_in, rec, attenuation, scattered, texture_sampler)
        }
        // Material difuso
        else {
            self.diffuse_scatter(ray_in, rec, attenuation, scattered, texture_sampler)
        }
    }

    fn reflect(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray, texture_sampler: &TextureSampler) -> bool {
        let dir = Self::reflect_vector(ray_in.direction.normalized(), rec.normal);
        *scattered = spawn_ray(rec.point, rec.normal, dir);

        // Considera reflejo blanco solo para materiales altamente transparentes (vidrio/agua)
        let use_white_reflection = self.transparency >= 0.9;

        if use_white_reflection {
            *attenuation = Vector3::new(0.98, 0.98, 0.98);
        } else {
            let texture_path = self.get_texture_for_normal(rec.normal);
            let tc = texture_sampler.sample_texture(texture_path, rec.u, rec.v);
            let tex = Vector3::new(tc.r as f32 / 255.0, tc.g as f32 / 255.0, tc.b as f32 / 255.0);
            let reflect_factor = 0.7 + self.reflectivity * 0.3;
            *attenuation = tex * reflect_factor;
        }
        true
    }

    fn refract(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray, texture_sampler: &TextureSampler) -> bool {
        // 1) Textura y alpha
        let texture_path = self.get_texture_for_normal(rec.normal);
        let tex = texture_sampler.sample_texture(texture_path, rec.u, rec.v);
        let tex_a = (tex.a as f32 / 255.0).clamp(0.0, 1.0);

        // 2) "Huecos" del PNG (líneas de vidrio dejan marco, fondo transparente pasa el rayo)
        if tex_a < 0.05 {
            // Pasa como aire (evita teñir con RGB del PNG)
            let dir = ray_in.direction;
            *scattered = spawn_ray(rec.point, rec.normal, dir);
            *attenuation = Vector3::new(1.0, 1.0, 1.0);
            return true;
        }

        // 3) Fresnel + refracción/reflectancia
        let eta = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_dir = ray_in.direction.normalized();
        let cos_theta = (-unit_dir).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = eta * sin_theta > 1.0;

        let fresnel = Self::reflectance(cos_theta, eta);
        let do_reflect = cannot_refract || fastrand::f32() < fresnel;

        let dir = if do_reflect {
            Self::reflect_vector(unit_dir, rec.normal)
        } else {
            Self::refract_vector(unit_dir, rec.normal, eta)
        };

        // 4) Usar spawn_ray para offset automático basado en dirección
        *scattered = spawn_ray(rec.point, rec.normal, dir);

        // 5) Atenuación neutra (sin teñir con RGB)
        let absorption = 0.005; // Reducido para evitar oscurecimiento excesivo
        *attenuation = Vector3::new(1.0 - absorption, 1.0 - absorption, 1.0 - absorption);

        true
    }

    fn diffuse_scatter(&self, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray, texture_sampler: &TextureSampler) -> bool {
        let mut scatter_direction = rec.normal + Self::random_unit_vector();

        // Catch degenerate scatter direction
        if Self::near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.point, scatter_direction);
        
        // Obtener la textura correcta según la normal de la superficie
        let texture_path = self.get_texture_for_normal(rec.normal);
        
        // Samplear textura para materiales difusos
        let texture_color = texture_sampler.sample_texture(texture_path, rec.u, rec.v);
        *attenuation = Vector3::new(
            texture_color.r as f32 / 255.0,
            texture_color.g as f32 / 255.0,
            texture_color.b as f32 / 255.0,
        );
        true
    }

    fn reflect_vector(v: Vector3, n: Vector3) -> Vector3 {
        v - n * (2.0 * v.dot(n))
    }

    fn refract_vector(uv: Vector3, n: Vector3, etai_over_etat: f32) -> Vector3 {
        let cos_theta = (-uv).dot(n).min(1.0);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let length_sq = r_out_perp.x * r_out_perp.x + r_out_perp.y * r_out_perp.y + r_out_perp.z * r_out_perp.z;
        let r_out_parallel = n * (-(1.0 - length_sq).abs().sqrt());
        r_out_perp + r_out_parallel
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Schlick's approximation
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }

    fn random_unit_vector() -> Vector3 {
        let a = fastrand::f32() * 2.0 * std::f32::consts::PI;
        let z = fastrand::f32() * 2.0 - 1.0;
        let r = (1.0 - z * z).sqrt();
        Vector3::new(r * a.cos(), r * a.sin(), z)
    }

    fn near_zero(v: Vector3) -> bool {
        let s = 1e-8;
        v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
    }
    
    // Materiales predefinidos mejorados para raytracing
    pub fn iron() -> Self {
        Self::new(
            "assets/iron_block.png".to_string(),
            Vector3::new(1.0, 1.0, 1.0),    // Blanco puro - usar solo textura PNG
            0.6,   // Especularidad moderada
            0.0,   // Opaco
            0.6,   // Reflectividad moderada
            1.0,   // Sin refracción
            32.0   // Brillo moderado
        )
    }
    
    pub fn diamond() -> Self {
        Self::new(
            "assets/diamond_block.png".to_string(),
            Vector3::new(1.0, 1.0, 1.0),    // Blanco puro - usar solo textura PNG
            0.9,   // Alta especularidad
            0.1,   // Ligeramente transparente
            0.8,   // Alta reflectividad
            2.42,  // Índice de refracción del diamante
            128.0  // Muy brillante
        )
    }
    
    pub fn grass() -> Self {
        Self::with_face_textures(
            "assets/grass_carried.png".to_string(),                   // Textura principal (fallback)
            Some("assets/grass_carried.png".to_string()),             // Cara superior: césped verde
            Some("assets/dirt.png".to_string()),                      // Cara inferior: tierra
            Some("assets/grass_side_carried.png".to_string()),        // Caras laterales: césped+tierra
            Vector3::new(1.0, 1.0, 1.0),    // Blanco puro - usar solo texturas PNG
            0.1,   // Baja especularidad
            0.0,   // Opaco
            0.05,  // Muy poca reflectividad
            1.0,   // Sin refracción
            4.0    // Poco brillante
        )
    }
    
    pub fn dirt() -> Self {
        Self::with_face_textures(
            "assets/dirt.png".to_string(),                            // Textura principal
            Some("assets/grass_carried.png".to_string()),             // Cara superior: césped (para bloques de tierra con césped)
            Some("assets/dirt.png".to_string()),                      // Cara inferior: tierra
            Some("assets/grass_side_carried.png".to_string()),        // Caras laterales: césped+tierra
            Vector3::new(1.0, 1.0, 1.0),    // Blanco puro - usar solo textura PNG
            0.02,  // Muy poca especularidad
            0.0,   // Opaco
            0.02,  // Muy poca reflectividad
            1.0,   // Sin refracción
            1.0    // Muy mate
        )
    }
    
    pub fn water() -> Self {
        Self::new(
            "assets/water_still.png".to_string(),
            Vector3::new(1.0, 1.0, 1.0),    // Blanco puro - usar solo textura PNG
            0.3,   // Especularidad moderada (aumentada)
            0.95,  // Muy transparente para alpha channel cutout  
            0.05,  // Reflectividad muy baja para dieléctricos
            1.33,  // Índice de refracción del agua
            16.0   // Brillo aumentado
        )
    }
    
    pub fn glass() -> Self {
        Self::new(
            "assets/glass.png".to_string(),
            Vector3::new(1.0, 1.0, 1.0),    // Blanco puro - usar solo textura PNG
            0.2,   // Especularidad aumentada
            0.95,  // Muy transparente para alpha channel cutout
            0.05,  // Reflectividad muy baja para dieléctricos
            1.52,  // Índice de refracción del vidrio
            8.0    // Brillo moderado
        )
    }
}

impl TextureSampler {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading texture: {}", path);
        
        // SOLO cargar imágenes PNG reales, sin fallbacks
        match image::open(path) {
            Ok(img) => {
                println!("Successfully loaded PNG texture: {} ({}x{})", path, img.width(), img.height());
                self.textures.insert(path.to_string(), img);
                Ok(())
            }
            Err(e) => {
                eprintln!("CRITICAL ERROR: Failed to load required PNG texture {}: {}", path, e);
                eprintln!("Make sure the texture file exists in the assets folder!");
                Err(Box::new(e))
            }
        }
    }

    fn create_fallback_texture(&self, path: &str) -> DynamicImage {
        use image::{ImageBuffer, Rgba};
        
        let size = 64u32;
        let mut imgbuf = ImageBuffer::new(size, size);
        
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let color = match path {
                "assets/iron_block.png" => {
                    // Patrón metálico más claro
                    let is_grid = (x % 8 == 0) || (y % 8 == 0);
                    if is_grid {
                        [130, 130, 140, 255]  // Líneas de rejilla más claras
                    } else {
                        let noise = ((x * 7 + y * 11) % 16) as f32 / 16.0;
                        let brightness = (160.0 + noise * 40.0) as u8;  // Base más clara
                        [brightness, brightness, brightness + 10, 255]
                    }
                },
                "assets/diamond_block.png" => {
                    // Patrón brillante azulado
                    let center_x = size as f32 / 2.0;
                    let center_y = size as f32 / 2.0;
                    let dist = ((x as f32 - center_x).abs() + (y as f32 - center_y).abs()) / center_x;
                    let sparkle = ((x * 13 + y * 17) % 32) as f32 / 32.0;
                    let intensity = (1.0 - dist).max(0.3) + sparkle * 0.4;
                    [
                        (160.0 + intensity * 95.0) as u8,
                        (200.0 + intensity * 55.0) as u8,
                        (230.0 + intensity * 25.0) as u8,
                        255
                    ]
                },
                "assets/grass_carried.png" => {
                    // Patrón verde césped
                    let grass_blade = ((x + y * 2) % 4 == 0) && ((x * 3 + y) % 7 < 2);
                    let dirt_spot = ((x * 5 + y * 7) % 23 < 3);
                    
                    if dirt_spot {
                        [101, 67, 33, 255]
                    } else if grass_blade {
                        [34, 139, 34, 255]
                    } else {
                        let variation = ((x * 7 + y * 11) % 16) as f32 / 16.0;
                        let green = (107.0 + variation * 30.0) as u8;
                        [50, green, 40, 255]
                    }
                },
                "assets/dirt.png" => {
                    // Patrón de tierra más marrón
                    let rock = (x * 11 + y * 13) % 29 < 2;
                    let clump = (x * 3 + y * 7) % 13 < 4;
                    
                    if rock {
                        [110, 85, 65, 255]  // Piedras marrones
                    } else if clump {
                        [120, 75, 45, 255]  // Terrones de tierra marrón
                    } else {
                        let variation = ((x * 5 + y * 9) % 12) as f32 / 12.0;
                        let brown_base = 130.0 + variation * 20.0;
                        let brown = brown_base as u8;
                        [brown, (brown_base * 0.70) as u8, (brown_base * 0.35) as u8, 255]
                    }
                },
                "assets/water_still.png" => {
                    // Patrón de agua
                    let wave = (((x + y) as f32 * 0.2).sin() + 1.0) * 0.5;
                    let blue_intensity = (60.0 + wave * 80.0) as u8;
                    [(wave * 30.0) as u8, (wave * 60.0) as u8, blue_intensity, 180]
                },
                "assets/glass.png" => {
                    // Patrón de vidrio
                    let glass_noise = ((x + y) % 8) as f32 / 8.0;
                    let transparency = (220.0 + glass_noise * 20.0) as u8;
                    [transparency, transparency, transparency + 10, 150]
                },
                "assets/grass_side_carried.png" => {
                    // Patrón de césped lateral (tierra abajo, césped arriba)
                    let is_grass_part = y < size / 2; // Mitad superior es césped
                    let grass_blade = ((x + y * 2) % 3 == 0) && ((x * 2 + y) % 5 < 2);
                    let dirt_part = y >= size / 2; // Mitad inferior es tierra
                    
                    if is_grass_part {
                        if grass_blade {
                            [34, 139, 34, 255]  // Verde césped intenso
                        } else {
                            let variation = ((x * 7 + y * 11) % 16) as f32 / 16.0;
                            let green = (85.0 + variation * 40.0) as u8;
                            [40, green, 35, 255]
                        }
                    } else if dirt_part {
                        let variation = ((x * 5 + y * 9) % 12) as f32 / 12.0;
                        let brown = (110.0 + variation * 20.0) as u8;
                        [brown, (brown as f32 * 0.75) as u8, (brown as f32 * 0.50) as u8, 255]
                    } else {
                        [85, 107, 47, 255] // Verde oliva por defecto
                    }
                },
                _ => [255, 255, 255, 255],
            };
            
            *pixel = Rgba(color);
        }
        
        DynamicImage::ImageRgba8(imgbuf)
    }

    pub fn sample_texture(&self, texture_path: &str, u: f32, v: f32) -> Color {
        // SOLO usar texturas PNG reales, sin fallbacks
        if let Some(img) = self.textures.get(texture_path) {
            let width = img.width();
            let height = img.height();
            
            // CORRECCIÓN: Invertir verticalmente la textura grass_side_carried para orientación correcta
            let corrected_v = if texture_path == "assets/grass_side_carried.png" {
                1.0 - v  // Invertir coordenada V para mostrar césped arriba, tierra abajo
            } else {
                v
            };
            
            // Manejar texturas animadas como el agua (16x512 significa 32 frames de 16x16)
            let (actual_width, actual_height, frame_offset) = if texture_path.contains("water") && height > width {
                let frame_size = width; // Cada frame es 16x16
                let num_frames = height / frame_size;
                
                // Simular animación con tiempo (usar coordenadas para variar el frame)
                let time_factor = (u + corrected_v) * 10.0; // Factor de tiempo simulado
                let frame_index = (time_factor as u32) % num_frames;
                
                (width, frame_size, frame_index * frame_size)
            } else {
                (width, height, 0)
            };
            
            // Calcular coordenadas del pixel con wrapping usando coordenada V corregida
            let x = ((u * actual_width as f32) as u32).min(actual_width - 1);
            let y = ((corrected_v * actual_height as f32) as u32).min(actual_height - 1) + frame_offset;
            
            // Asegurar que y esté dentro de los límites de la imagen
            let y = y.min(height - 1);
            
            // Obtener el pixel de la imagen PNG real
            let pixel = img.get_pixel(x, y);
            
            return Color::new(pixel[0], pixel[1], pixel[2], pixel[3]);
        }
        
        // Si no hay textura PNG, devolver color por defecto sin fallback
        eprintln!("ERROR: Texture not found: {}", texture_path);
        Color::WHITE
    }
}