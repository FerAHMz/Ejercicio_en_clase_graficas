use raylib::prelude::*;
use crate::materials::Material;
use crate::ray::{HittableList, Cube};

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

// Función para crear el mundo/diorama optimizado para raytracing
pub fn create_world() -> Vec<Block> {
    let mut world = Vec::new();
    
    // === BASE DE TIERRA REDUCIDA ===
    for x in -4..=4 {
        for z in -4..=4 {
            // Suelo de tierra más compacto
            world.push(Block::new(
                Vector3::new(x as f32 * 2.0, -2.0, z as f32 * 2.0),
                Material::dirt()
            ));
        }
    }
    
    // === CÉSPED EN LA SUPERFICIE ===
    for x in -3..=3 {
        for z in -3..=3 {
            if (x * x + z * z) < 9 { // Área circular más pequeña
                world.push(Block::new(
                    Vector3::new(x as f32 * 2.0, 0.0, z as f32 * 2.0),
                    Material::grass()
                ));
            }
        }
    }
    
    // === TORRE DE HIERRO REDUCIDA ===
    for y in 1..=3 {
        world.push(Block::new(
            Vector3::new(-6.0, y as f32 * 2.0, -6.0),
            Material::iron()
        ));
        world.push(Block::new(
            Vector3::new(-4.0, y as f32 * 2.0, -6.0),
            Material::iron()
        ));
    }
    
    // Techo de hierro
    world.push(Block::new(
        Vector3::new(-6.0, 8.0, -6.0),
        Material::iron()
    ));
    world.push(Block::new(
        Vector3::new(-4.0, 8.0, -6.0),
        Material::iron()
    ));
    
    // === PIRÁMIDE DE DIAMANTE REDUCIDA ===
    // Nivel 0 (base)
    world.push(Block::new(
        Vector3::new(6.0, 2.0, 6.0),
        Material::diamond()
    ));
    world.push(Block::new(
        Vector3::new(4.0, 2.0, 6.0),
        Material::diamond()
    ));
    world.push(Block::new(
        Vector3::new(6.0, 2.0, 4.0),
        Material::diamond()
    ));
    world.push(Block::new(
        Vector3::new(4.0, 2.0, 4.0),
        Material::diamond()
    ));
    
    // Nivel 1 (cima)
    world.push(Block::new(
        Vector3::new(5.0, 4.0, 5.0),
        Material::diamond()
    ));
    
    // === LAGO DE AGUA REDUCIDO ===
    world.push(Block::new(
        Vector3::new(0.0, 1.0, 6.0),
        Material::water()
    ));
    world.push(Block::new(
        Vector3::new(-2.0, 1.0, 6.0),
        Material::water()
    ));
    world.push(Block::new(
        Vector3::new(2.0, 1.0, 6.0),
        Material::water()
    ));
    
    // === ESTRUCTURA DE VIDRIO REDUCIDA ===
    // Paredes de vidrio
    world.push(Block::new(
        Vector3::new(-6.0, 2.0, -4.0),
        Material::glass()
    ));
    world.push(Block::new(
        Vector3::new(-6.0, 4.0, -4.0),
        Material::glass()
    ));
    world.push(Block::new(
        Vector3::new(-6.0, 2.0, -2.0),
        Material::glass()
    ));
    world.push(Block::new(
        Vector3::new(-6.0, 4.0, -2.0),
        Material::glass()
    ));
    
    world
}

// Función para convertir el mundo de bloques en una escena de raytracing
pub fn create_raytraced_scene(materials: &[Material]) -> (HittableList, Vec<Material>) {
    let mut scene = HittableList::new();
    let world = create_world();
    
    // Crear un mapeo de materiales a índices
    let material_map = vec![
        ("assets/iron_block.png", 0),
        ("assets/diamond_block.png", 1),
        ("assets/grass_top.png", 2),
        ("assets/dirt.png", 3),
        ("assets/water_still.png", 4),
        ("assets/glass.png", 5),
    ].into_iter().collect::<std::collections::HashMap<&str, usize>>();
    
    // Convertir cada bloque a un cubo raytreable
    for block in world {
        let material_index = material_map.get(block.material.texture.as_str()).unwrap_or(&0);
        let cube = Cube::new(
            block.position,
            Vector3::new(2.0, 2.0, 2.0) * block.scale,
            *material_index,
        );
        scene.add(Box::new(cube));
    }
    
    (scene, materials.to_vec())
}