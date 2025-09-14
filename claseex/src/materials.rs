use raylib::prelude::*;

#[derive(Clone)]
pub struct Material {
    pub texture: String,
    pub albedo: Vector3,           // Color base (RGB 0-1)
    pub specular: f32,            // Reflectividad especular (0-1)
    pub transparency: f32,        // Transparencia (0=opaco, 1=transparente)
    pub reflectivity: f32,        // Reflectividad (0-1)
    pub refraction_index: f32,    // Índice de refracción
    pub shininess: f32,           // Brillo especular
}

impl Material {
    pub fn new(texture: String, albedo: Vector3, specular: f32, transparency: f32, reflectivity: f32, refraction_index: f32, shininess: f32) -> Self {
        Self {
            texture,
            albedo,
            specular,
            transparency,
            reflectivity,
            refraction_index,
            shininess,
        }
    }
    
    // Materiales predefinidos
    pub fn iron() -> Self {
        Self::new(
            "assets/iron_block.png".to_string(),
            Vector3::new(0.7, 0.7, 0.8),  // Gris metálico
            0.9,   // Alta especularidad
            0.0,   // Opaco
            0.8,   // Alta reflectividad
            1.0,   // Sin refracción
            128.0  // Muy brillante
        )
    }
    
    pub fn diamond() -> Self {
        Self::new(
            "assets/diamond_block.png".to_string(),
            Vector3::new(0.8, 0.9, 1.0),  // Azul claro brillante
            0.95,  // Muy alta especularidad
            0.0,   // Opaco
            0.9,   // Muy alta reflectividad
            1.0,   // Sin refracción
            256.0  // Extremadamente brillante
        )
    }
    
    pub fn grass() -> Self {
        Self::new(
            "assets/grass_top.png".to_string(),
            Vector3::new(0.4, 0.8, 0.2),  // Verde natural
            0.1,   // Baja especularidad
            0.0,   // Opaco
            0.0,   // Sin reflectividad
            1.0,   // Sin refracción
            8.0    // Poco brillante
        )
    }
    
    pub fn dirt() -> Self {
        Self::new(
            "assets/dirt.png".to_string(),
            Vector3::new(0.6, 0.4, 0.2),  // Marrón tierra
            0.0,   // Sin especularidad
            0.0,   // Opaco
            0.0,   // Sin reflectividad
            1.0,   // Sin refracción
            1.0    // Mate
        )
    }
    
    pub fn water() -> Self {
        Self::new(
            "assets/water_still.png".to_string(),
            Vector3::new(0.2, 0.4, 0.8),  // Azul agua
            0.3,   // Especularidad moderada
            0.7,   // Bastante transparente
            0.4,   // Reflectividad moderada
            1.33,  // Índice de refracción del agua
            32.0   // Moderadamente brillante
        )
    }
    
    pub fn glass() -> Self {
        Self::new(
            "assets/glass.png".to_string(),
            Vector3::new(0.9, 0.9, 0.9),  // Casi blanco
            0.8,   // Alta especularidad
            0.9,   // Muy transparente
            0.1,   // Baja reflectividad
            1.52,  // Índice de refracción del vidrio
            64.0   // Brillante
        )
    }
}