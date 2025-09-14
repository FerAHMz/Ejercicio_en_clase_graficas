use raylib::prelude::*;

/// Estructura para el skybox procedural simplificado
pub struct Skybox {
    pub top_color: Color,        // Color del cielo superior
    pub horizon_color: Color,    // Color del horizonte  
    pub bottom_color: Color,     // Color del cielo inferior
    pub sun_direction: Vector3,  // Dirección del sol
    pub sun_color: Color,        // Color del sol
    pub sun_size: f32,           // Tamaño del sol
}

impl Skybox {
    /// Crear un nuevo skybox con configuración predeterminada
    pub fn new() -> Self {
        Self {
            top_color: Color::new(127, 178, 255, 255),     // Azul claro (0.5, 0.7, 1.0)
            horizon_color: Color::new(204, 229, 255, 255), // Casi blanco (0.8, 0.9, 1.0)
            bottom_color: Color::new(153, 204, 229, 255),  // Azul pálido (0.6, 0.8, 0.9)
            sun_direction: Vector3::new(0.3, 0.6, 0.4).normalized(), // Dirección del sol normalizada
            sun_color: Color::new(255, 229, 178, 255),     // Amarillo cálido (1.0, 0.9, 0.7)
            sun_size: 0.02,                                // Tamaño pequeño del sol
        }
    }
    
    /// Muestrear color del skybox en una dirección específica
    pub fn sample(&self, direction: Vector3) -> Color {
        let dir = direction.normalized();
        
        // Calcular el gradiente vertical
        let t = (dir.y + 1.0) * 0.5; // Mapear de [-1, 1] a [0, 1]
        
        let sky_color = if t > 0.5 {
            // Hemisferio superior: interpolar entre horizonte y cielo
            let upper_t = (t - 0.5) * 2.0;
            self.horizon_color.lerp(self.top_color, upper_t)
        } else {
            // Hemisferio inferior: interpolar entre abajo y horizonte
            let lower_t = t * 2.0;
            self.bottom_color.lerp(self.horizon_color, lower_t)
        };
        
        // Añadir sol
        let sun_dot = dir.dot(self.sun_direction);
        if sun_dot > (1.0 - self.sun_size) {
            let sun_intensity = ((sun_dot - (1.0 - self.sun_size)) / self.sun_size).powf(2.0);
            sky_color.lerp(self.sun_color, sun_intensity)
        } else {
            // Añadir resplandor del sol
            let glow_size = self.sun_size * 3.0;
            if sun_dot > (1.0 - glow_size) {
                let glow_intensity = ((sun_dot - (1.0 - glow_size)) / glow_size).powf(0.5) * 0.3;
                sky_color.lerp(self.sun_color, glow_intensity)
            } else {
                sky_color
            }
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

/// Obtener color del skybox para una dirección
pub fn get_skybox_color(direction: Vector3) -> Color {
    if let Some(skybox) = SKYBOX_INSTANCE.get() {
        skybox.sample(direction)
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
    // Cara superior (cielo)
    let top_color = get_skybox_color(Vector3::new(0.0, 1.0, 0.0));
    d3d.draw_cube(
        Vector3::new(0.0, skybox_size / 2.0, 0.0),
        skybox_size, 4.0, skybox_size, // Caras más gruesas
        top_color
    );
    
    // Horizonte (nivel del ojo)
    let horizon_color = get_skybox_color(Vector3::new(1.0, 0.0, 0.0));
    d3d.draw_cube(
        Vector3::new(0.0, 0.0, 0.0),
        skybox_size, skybox_size / 2.0, skybox_size,
        horizon_color
    );
    
    // Cara inferior
    let bottom_color = get_skybox_color(Vector3::new(0.0, -1.0, 0.0));
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