use raylib::prelude::*;
use crate::materials::TextureSampler;

/// Estructura para el skybox con texturas mejorado para raytracing
#[derive(Clone)]
pub struct Skybox {
    pub top_texture: String,     // Textura del cielo superior
    pub horizon_texture: String, // Textura del horizonte
    pub bottom_texture: String,  // Textura del cielo inferior
    pub sun_direction: Vector3,  // Dirección del sol
    pub sun_color: Color,        // Color del sol
    pub sun_size: f32,           // Tamaño del sol
    pub cloud_density: f32,      // Densidad de nubes
    pub fallback_top: Color,     // Color fallback si no hay textura
    pub fallback_horizon: Color, // Color fallback horizonte
    pub fallback_bottom: Color,  // Color fallback inferior
}

impl Skybox {
    /// Crear un nuevo skybox con texturas
    pub fn new() -> Self {
        Self {
            // Usar texturas más apropiadas para un skybox natural
            top_texture: "assets/diamond_block.png".to_string(),    // Cielo brillante como diamante (azul claro)
            horizon_texture: "assets/iron_block.png".to_string(),   // Horizonte metálico neutro
            bottom_texture: "assets/grass_top.png".to_string(),     // Verde césped en la parte inferior
            sun_direction: Vector3::new(0.4, 0.7, 0.3).normalized(), // Sol más alto
            sun_color: Color::new(255, 255, 224, 255),     // Amarillo sol más brillante
            sun_size: 0.015,                               // Sol un poco más pequeño
            cloud_density: 0.3,                           // Densidad moderada de nubes
            // Colores fallback si las texturas no cargan
            fallback_top: Color::new(135, 206, 250, 255),     // Azul cielo
            fallback_horizon: Color::new(255, 248, 220, 255), // Color cálido del horizonte
            fallback_bottom: Color::new(175, 238, 238, 255),  // Azul pálido inferior
        }
    }
    
    /// Muestrear color del skybox con gradiente natural (sin texturas problemáticas)
    pub fn sample(&self, direction: Vector3, _texture_sampler: &TextureSampler) -> Color {
        let dir = direction.normalized();
        
        // Calcular el gradiente vertical para skybox natural
        let t = (dir.y + 1.0) * 0.5; // Mapear de [-1, 1] a [0, 1]
        
        let sky_color = if t > 0.7 {
            // Hemisferio superior: azul cielo claro
            let intensity = (t - 0.7) / 0.3; // [0.7, 1.0] -> [0, 1]
            Color::new(
                (135.0 + intensity * 50.0) as u8,  // Azul más claro hacia arriba
                (206.0 + intensity * 25.0) as u8,  
                (250.0 + intensity * 5.0) as u8,   
                255
            )
        } else if t > 0.3 {
            // Zona del horizonte: transición suave
            let intensity = (t - 0.3) / 0.4; // [0.3, 0.7] -> [0, 1]
            
            Color::new(
                (255.0 * (1.0 - intensity) + 135.0 * intensity) as u8,  // De amarillo a azul
                (248.0 * (1.0 - intensity) + 206.0 * intensity) as u8,  
                (220.0 * (1.0 - intensity) + 250.0 * intensity) as u8,  
                255
            )
        } else {
            // Hemisferio inferior: más cálido cerca del suelo
            let intensity = t / 0.3; // [0, 0.3] -> [0, 1]
            
            Color::new(
                (200.0 + intensity * 55.0) as u8,  // Tonos más cálidos abajo
                (200.0 + intensity * 48.0) as u8,  
                (180.0 + intensity * 40.0) as u8,  
                255
            )
        };
        
        // Añadir sol
        let sun_dot = dir.dot(self.sun_direction);
        if sun_dot > (1.0 - self.sun_size) {
            let sun_intensity = ((sun_dot - (1.0 - self.sun_size)) / self.sun_size).powf(2.0);
            Color::new(
                (sky_color.r as f32 * (1.0 - sun_intensity) + self.sun_color.r as f32 * sun_intensity) as u8,
                (sky_color.g as f32 * (1.0 - sun_intensity) + self.sun_color.g as f32 * sun_intensity) as u8,
                (sky_color.b as f32 * (1.0 - sun_intensity) + self.sun_color.b as f32 * sun_intensity) as u8,
                255
            )
        } else {
            // Añadir resplandor del sol
            let glow_size = self.sun_size * 3.0;
            if sun_dot > (1.0 - glow_size) {
                let glow_intensity = ((sun_dot - (1.0 - glow_size)) / glow_size).powf(0.5) * 0.3;
                Color::new(
                    (sky_color.r as f32 * (1.0 - glow_intensity) + self.sun_color.r as f32 * glow_intensity) as u8,
                    (sky_color.g as f32 * (1.0 - glow_intensity) + self.sun_color.g as f32 * glow_intensity) as u8,
                    (sky_color.b as f32 * (1.0 - glow_intensity) + self.sun_color.b as f32 * glow_intensity) as u8,
                    255
                )
            } else {
                sky_color
            }
        }
    }

    /// Versión fallback sin TextureSampler (para compatibilidad)
    pub fn sample_fallback(&self, direction: Vector3) -> Color {
        let dir = direction.normalized();
        let t = (dir.y + 1.0) * 0.5;
        
        if t > 0.5 {
            let upper_t = (t - 0.5) * 2.0;
            Color::new(
                (self.fallback_horizon.r as f32 * (1.0 - upper_t) + self.fallback_top.r as f32 * upper_t) as u8,
                (self.fallback_horizon.g as f32 * (1.0 - upper_t) + self.fallback_top.g as f32 * upper_t) as u8,
                (self.fallback_horizon.b as f32 * (1.0 - upper_t) + self.fallback_top.b as f32 * upper_t) as u8,
                255
            )
        } else {
            let lower_t = t * 2.0;
            Color::new(
                (self.fallback_bottom.r as f32 * (1.0 - lower_t) + self.fallback_horizon.r as f32 * lower_t) as u8,
                (self.fallback_bottom.g as f32 * (1.0 - lower_t) + self.fallback_horizon.g as f32 * lower_t) as u8,
                (self.fallback_bottom.b as f32 * (1.0 - lower_t) + self.fallback_horizon.b as f32 * lower_t) as u8,
                255
            )
        }
    }
}

/// Instancia global del skybox usando OnceCell para seguridad
use std::sync::OnceLock;
static SKYBOX_INSTANCE: OnceLock<Skybox> = OnceLock::new();

/// Inicializar el skybox global
pub fn init_skybox() {
    let _ = SKYBOX_INSTANCE.set(Skybox::new());
}

/// Obtener color del skybox para una dirección con TextureSampler
pub fn get_skybox_color(direction: Vector3, texture_sampler: &TextureSampler) -> Color {
    if let Some(skybox) = SKYBOX_INSTANCE.get() {
        skybox.sample(direction, texture_sampler)
    } else {
        Color::BLACK
    }
}

/// Obtener color del skybox para una dirección (fallback sin texturas)
pub fn get_skybox_color_fallback(direction: Vector3) -> Color {
    if let Some(skybox) = SKYBOX_INSTANCE.get() {
        skybox.sample_fallback(direction)
    } else {
        Color::BLACK
    }
}

/// Función para dibujar el skybox usando el sistema procedural simplificado
pub fn draw_skybox(d3d: &mut RaylibMode3D<RaylibDrawHandle>) {
    // Asegurar que el skybox esté inicializado
    if SKYBOX_INSTANCE.get().is_none() {
        init_skybox();
    }
    
    // Skybox MUCHO más grande para evitar z-fighting
    // Con zoom máximo de 40, el skybox debe estar al menos a 200+ unidades
    let skybox_size = 400.0; // 4x más grande
    
    // Simplemente dibujar las caras principales del skybox
    // Cara superior (cielo) - usar colores fallback para el modo rasterización
    let top_color = get_skybox_color_fallback(Vector3::new(0.0, 1.0, 0.0));
    d3d.draw_cube(
        Vector3::new(0.0, skybox_size / 2.0, 0.0),
        skybox_size, 4.0, skybox_size, // Caras más gruesas
        top_color
    );
    
    // Horizonte (nivel del ojo)
    let horizon_color = get_skybox_color_fallback(Vector3::new(1.0, 0.0, 0.0));
    d3d.draw_cube(
        Vector3::new(0.0, 0.0, 0.0),
        skybox_size, skybox_size / 2.0, skybox_size,
        horizon_color
    );
    
    // Cara inferior
    let bottom_color = get_skybox_color_fallback(Vector3::new(0.0, -1.0, 0.0));
    d3d.draw_cube(
        Vector3::new(0.0, -skybox_size / 2.0, 0.0),
        skybox_size, 4.0, skybox_size, // Caras más gruesas
        bottom_color
    );
    
    // Añadir el sol como una esfera brillante en su posición
    if let Some(skybox) = SKYBOX_INSTANCE.get() {
        let sun_pos = skybox.sun_direction * (skybox_size * 0.7); // Más cerca del centro
        d3d.draw_sphere(sun_pos, 3.0, skybox.sun_color); // Sol un poco más grande
    }
}