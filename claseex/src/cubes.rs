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

// Función para crear el mundo/diorama creativo y llamativo para raytracing
pub fn create_world() -> Vec<Block> {
    let mut world = Vec::new();
    
    // ===============================
    // 🏔️ BASE NATURAL CON TERRAZA ESCALONADA
    // ===============================
    
    // Nivel base de tierra (-2) - Círculo grande
    for x in -5..=5 {
        for z in -5..=5 {
            let dist_sq = x * x + z * z;
            if dist_sq <= 25 { // Radio de 5 bloques
                world.push(Block::new(
                    Vector3::new(x as f32 * 2.0, -2.0, z as f32 * 2.0),
                    Material::dirt()
                ));
            }
        }
    }
    
    // Nivel intermedio de tierra (-1) - Círculo medio
    for x in -3..=3 {
        for z in -3..=3 {
            let dist_sq = x * x + z * z;
            if dist_sq <= 9 { // Radio de 3 bloques
                world.push(Block::new(
                    Vector3::new(x as f32 * 2.0, 0.0, z as f32 * 2.0),
                    Material::dirt()
                ));
            }
        }
    }
    
    // Superficie de césped (nivel 0) - Área central
    for x in -2..=2 {
        for z in -2..=2 {
            let dist_sq = x * x + z * z;
            if dist_sq <= 4 { // Radio de 2 bloques
                world.push(Block::new(
                    Vector3::new(x as f32 * 2.0, 2.0, z as f32 * 2.0),
                    Material::grass()
                ));
            }
        }
    }
    
    // ===============================
    // 🏰 CASTILLO DE HIERRO MEDIEVAL
    // ===============================
    
    // Torre principal (esquina noroeste)
    for y in 1..=4 {
        world.push(Block::new(
            Vector3::new(-8.0, y as f32 * 2.0, -8.0),
            Material::iron()
        ));
    }
    
    // Torres secundarias
    world.push(Block::new(Vector3::new(-8.0, 2.0, -4.0), Material::iron())); // Torre este
    world.push(Block::new(Vector3::new(-8.0, 4.0, -4.0), Material::iron()));
    
    world.push(Block::new(Vector3::new(-4.0, 2.0, -8.0), Material::iron())); // Torre sur
    world.push(Block::new(Vector3::new(-4.0, 4.0, -8.0), Material::iron()));
    
    // Muralla conectora
    world.push(Block::new(Vector3::new(-6.0, 2.0, -8.0), Material::iron()));
    world.push(Block::new(Vector3::new(-8.0, 2.0, -6.0), Material::iron()));
    
    // Almenas (detalles en la cima)
    world.push(Block::new(Vector3::new(-8.0, 10.0, -8.0), Material::iron()));
    world.push(Block::new(Vector3::new(-4.0, 6.0, -8.0), Material::iron()));
    
    // ===============================
    // 💎 TEMPLO DE DIAMANTE MÍSTICO
    // ===============================
    
    // Base del templo (cruz)
    world.push(Block::new(Vector3::new(6.0, 2.0, 6.0), Material::diamond()));   // Centro
    world.push(Block::new(Vector3::new(8.0, 2.0, 6.0), Material::diamond()));   // Este
    world.push(Block::new(Vector3::new(4.0, 2.0, 6.0), Material::diamond()));   // Oeste
    world.push(Block::new(Vector3::new(6.0, 2.0, 8.0), Material::diamond()));   // Norte
    world.push(Block::new(Vector3::new(6.0, 2.0, 4.0), Material::diamond()));   // Sur
    
    // Segundo nivel (pirámide)
    world.push(Block::new(Vector3::new(6.0, 4.0, 6.0), Material::diamond()));   // Centro
    world.push(Block::new(Vector3::new(7.0, 4.0, 6.0), Material::diamond()));   // Este
    world.push(Block::new(Vector3::new(5.0, 4.0, 6.0), Material::diamond()));   // Oeste
    world.push(Block::new(Vector3::new(6.0, 4.0, 7.0), Material::diamond()));   // Norte
    world.push(Block::new(Vector3::new(6.0, 4.0, 5.0), Material::diamond()));   // Sur
    
    // Cúspide brillante
    world.push(Block::new(Vector3::new(6.0, 6.0, 6.0), Material::diamond()));
    world.push(Block::new(Vector3::new(6.0, 8.0, 6.0), Material::diamond()));
    
    // ===============================
    // 🌊 LAGO SERPENTEANTE Y CASCADA
    // ===============================
    
    // Lago principal (forma de L)
    world.push(Block::new(Vector3::new(0.0, 1.0, 8.0), Material::water()));
    world.push(Block::new(Vector3::new(-2.0, 1.0, 8.0), Material::water()));
    world.push(Block::new(Vector3::new(2.0, 1.0, 8.0), Material::water()));
    world.push(Block::new(Vector3::new(4.0, 1.0, 8.0), Material::water()));
    
    // Brazo del lago hacia el sur
    world.push(Block::new(Vector3::new(4.0, 1.0, 6.0), Material::water()));
    world.push(Block::new(Vector3::new(4.0, 1.0, 4.0), Material::water()));
    
    // Cascada (bloques de agua elevados)
    world.push(Block::new(Vector3::new(0.0, 3.0, 8.0), Material::water()));
    world.push(Block::new(Vector3::new(0.0, 5.0, 8.0), Material::water()));
    
    // ===============================
    // 🏛️ OBSERVATORIO DE VIDRIO
    // ===============================
    
    // Base del observatorio
    world.push(Block::new(Vector3::new(-6.0, 2.0, 2.0), Material::glass()));
    world.push(Block::new(Vector3::new(-4.0, 2.0, 2.0), Material::glass()));
    world.push(Block::new(Vector3::new(-6.0, 2.0, 4.0), Material::glass()));
    world.push(Block::new(Vector3::new(-4.0, 2.0, 4.0), Material::glass()));
    
    // Paredes de vidrio (estructura tipo invernadero)
    world.push(Block::new(Vector3::new(-6.0, 4.0, 2.0), Material::glass()));
    world.push(Block::new(Vector3::new(-4.0, 4.0, 2.0), Material::glass()));
    world.push(Block::new(Vector3::new(-6.0, 4.0, 4.0), Material::glass()));
    world.push(Block::new(Vector3::new(-4.0, 4.0, 4.0), Material::glass()));
    
    // Cúpula de vidrio
    world.push(Block::new(Vector3::new(-5.0, 6.0, 3.0), Material::glass()));
    world.push(Block::new(Vector3::new(-5.0, 8.0, 3.0), Material::glass()));
    
    // Torre de observación alta
    world.push(Block::new(Vector3::new(-5.0, 10.0, 3.0), Material::glass()));
    
    // ===============================
    // 🌉 PUENTES Y CONEXIONES
    // ===============================
    
    // Puente de hierro sobre el agua
    world.push(Block::new(Vector3::new(-1.0, 3.0, 8.0), Material::iron()));
    world.push(Block::new(Vector3::new(1.0, 3.0, 8.0), Material::iron()));
    
    // Camino de césped decorativo
    world.push(Block::new(Vector3::new(0.0, 4.0, 0.0), Material::grass()));
    world.push(Block::new(Vector3::new(2.0, 4.0, 2.0), Material::grass()));
    world.push(Block::new(Vector3::new(-2.0, 4.0, -2.0), Material::grass()));
    
    // ===============================
    // ✨ ELEMENTOS DECORATIVOS FLOTANTES
    // ===============================
    
    // Cristales de diamante flotantes (efectos mágicos)
    world.push(Block::new(Vector3::new(8.0, 6.0, 8.0), Material::diamond()));
    world.push(Block::new(Vector3::new(-8.0, 8.0, 8.0), Material::diamond()));
    world.push(Block::new(Vector3::new(8.0, 10.0, -8.0), Material::diamond()));
    
    // Esferas de agua suspendidas (magia acuática)
    world.push(Block::new(Vector3::new(0.0, 7.0, 0.0), Material::water()));
    world.push(Block::new(Vector3::new(3.0, 9.0, -3.0), Material::water()));
    
    // Ventanas de vidrio flotantes
    world.push(Block::new(Vector3::new(-3.0, 7.0, 8.0), Material::glass()));
    world.push(Block::new(Vector3::new(8.0, 5.0, 3.0), Material::glass()));
    
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
        ("assets/grass_carried.png", 2),    // Césped usa grass_carried como textura principal
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