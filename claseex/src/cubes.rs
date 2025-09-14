use raylib::prelude::*;
use crate::materials::Material;

// Estructura para representar un bloque en el mundo
#[derive(Clone)]
pub struct Block {
    pub position: Vector3,
    pub material: Material,
    pub scale: Vector3,
}

impl Block {
    pub fn new(position: Vector3, material: Material) -> Self {
        Self {
            position,
            material,
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
    
    pub fn with_scale(mut self, scale: Vector3) -> Self {
        self.scale = scale;
        self
    }
}

// Función para crear el mundo/diorama
pub fn create_world() -> Vec<Block> {
    let mut world = Vec::new();
    
    // === BASE DE TIERRA ===
    for x in -8..=8 {
        for z in -8..=8 {
            // Suelo de tierra con algunas variaciones de altura
            let height = if (x + z) % 3 == 0 { -3.0 } else { -4.0 };
            world.push(Block::new(
                Vector3::new(x as f32 * 2.0, height, z as f32 * 2.0),
                Material::dirt()
            ));
        }
    }
    
    // === CÉSPED EN LA SUPERFICIE ===
    for x in -7..=7 {
        for z in -7..=7 {
            if (x * x + z * z) < 36 { // Área circular de césped
                world.push(Block::new(
                    Vector3::new(x as f32 * 2.0, -2.0, z as f32 * 2.0),
                    Material::grass()
                ));
            }
        }
    }
    
    // === ESTRUCTURA DE HIERRO (Torre) ===
    for y in 0..=6 {
        world.push(Block::new(
            Vector3::new(-8.0, y as f32 * 2.0, -8.0),
            Material::iron()
        ));
        world.push(Block::new(
            Vector3::new(-8.0, y as f32 * 2.0, -6.0),
            Material::iron()
        ));
        world.push(Block::new(
            Vector3::new(-6.0, y as f32 * 2.0, -8.0),
            Material::iron()
        ));
        world.push(Block::new(
            Vector3::new(-6.0, y as f32 * 2.0, -6.0),
            Material::iron()
        ));
    }
    
    // Techo de la torre de hierro
    for x in -8..=-6 {
        for z in -8..=-6 {
            world.push(Block::new(
                Vector3::new(x as f32 * 2.0, 14.0, z as f32 * 2.0),
                Material::iron()
            ));
        }
    }
    
    // === ESTRUCTURA DE DIAMANTE (Pirámide) ===
    for level in 0..=3 {
        let size = 3 - level;
        for x in -size..=size {
            for z in -size..=size {
                world.push(Block::new(
                    Vector3::new(8.0 + x as f32 * 2.0, level as f32 * 2.0, 8.0 + z as f32 * 2.0),
                    Material::diamond()
                ));
            }
        }
    }
    
    // === LAGO DE AGUA ===
    for x in -2..=2 {
        for z in 2..=6 {
            world.push(Block::new(
                Vector3::new(x as f32 * 2.0, -1.0, z as f32 * 2.0),
                Material::water()
            ));
        }
    }
    
    // === CASA DE VIDRIO ===
    // Paredes
    for y in 0..=3 {
        // Pared frontal
        for x in -3..=3 {
            if y != 1 || (x != 0 && x != 1) { // Dejar espacio para puerta
                world.push(Block::new(
                    Vector3::new(x as f32 * 2.0, y as f32 * 2.0, -12.0),
                    Material::glass()
                ));
            }
        }
        // Pared trasera
        for x in -3..=3 {
            world.push(Block::new(
                Vector3::new(x as f32 * 2.0, y as f32 * 2.0, -16.0),
                Material::glass()
            ));
        }
        // Paredes laterales
        for z in -16..=-12 {
            world.push(Block::new(
                Vector3::new(-6.0, y as f32 * 2.0, z as f32 * 2.0),
                Material::glass()
            ));
            world.push(Block::new(
                Vector3::new(6.0, y as f32 * 2.0, z as f32 * 2.0),
                Material::glass()
            ));
        }
    }
    
    // Techo de vidrio
    for x in -3..=3 {
        for z in -16..=-12 {
            world.push(Block::new(
                Vector3::new(x as f32 * 2.0, 8.0, z as f32 * 2.0),
                Material::glass()
            ));
        }
    }
    
    world
}