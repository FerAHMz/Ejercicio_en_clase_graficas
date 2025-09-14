use raylib::prelude::*;
use std::collections::HashMap;

mod materials;
mod cubes;
mod math_utils;
mod raytracer;
mod camera;
mod skybox;

use cubes::create_world;
use raytracer::calculate_material_lighting;
use camera::CameraController;
use skybox::{draw_skybox, init_skybox};

fn main() {
    // Configuración inicial de la ventana
    let (mut rl, thread) = raylib::init()
        .size(1200, 900)
        .title("Diorama Minecraft Avanzado - Múltiples Materiales")
        .build();

    // Configurar controlador de cámara
    let mut camera_controller = CameraController::new();

    // Inicializar skybox procedural
    init_skybox();

    // Cargar todas las texturas
    let mut textures = HashMap::new();
    let texture_files = vec![
        "assets/iron_block.png",
        "assets/diamond_block.png", 
        "assets/grass_top.png",
        "assets/dirt.png",
        "assets/water_still.png",
        "assets/glass.png"
    ];
    
    for texture_file in texture_files {
        let texture = rl.load_texture(&thread, texture_file)
            .expect(&format!("Failed to load texture: {}", texture_file));
        textures.insert(texture_file.to_string(), texture);
    }
    
    // Crear modelos para cada material
    let mut models = HashMap::new();
    for (texture_path, texture) in &textures {
        // Crear un nuevo mesh para cada modelo
        let cube_mesh = Mesh::gen_mesh_cube(&thread, 2.0, 2.0, 2.0);
        let weak_mesh = unsafe { cube_mesh.make_weak() };
        let mut model = rl.load_model_from_mesh(&thread, weak_mesh).unwrap();
        model.materials_mut()[0].maps_mut()[0].texture = **texture;
        models.insert(texture_path.clone(), model);
    }

    // Crear el mundo
    let world = create_world();

    // Variables para la animación
    let mut time = 0.0f32;

    // Variables para la luz
    let ambient_intensity = 0.2;
    let diffuse_intensity = 0.8;

    // Configurar FPS
    rl.set_target_fps(60);

    // Loop principal
    while !rl.window_should_close() {
        // Actualizar tiempo
        let frame_time = rl.get_frame_time();
        time += frame_time;
        
        // Actualizar controlador de cámara
        camera_controller.update(&mut rl, frame_time);

        // Posición dinámica de la luz (simula el movimiento del sol)
        let light_position = Vector3::new(
            10.0 * (time * 0.5).cos(),
            15.0 + 5.0 * (time * 0.3).sin(),
            10.0 * (time * 0.5).sin(),
        );

        // Inicio del renderizado
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(135, 206, 235, 255)); // Color del cielo

        {
            // Obtener cámara rotada para simular rotación del diorama
            let rotated_camera = camera_controller.get_rotated_camera();
            let mut d3d = d.begin_mode3D(rotated_camera);

            // Dibujar skybox
            draw_skybox(&mut d3d);

            // Dibujar todos los bloques del mundo EN SU POSICIÓN ORIGINAL
            for block in &world {
                let model = models.get(&block.material.texture).unwrap();
                
                // Calcular iluminación para este bloque
                let surface_normal = Vector3::new(0.0, 1.0, 0.0); // Normal superior
                let lighting_color = calculate_material_lighting(
                    &block.material,
                    block.position, // POSICIÓN ORIGINAL, SIN ROTAR
                    surface_normal,
                    light_position,
                    rotated_camera.position, // Usar la posición de cámara rotada
                    ambient_intensity,
                    diffuse_intensity,
                );
                
                // Aplicar transparencia
                let mut tint = lighting_color;
                if block.material.transparency > 0.0 {
                    tint.a = ((1.0 - block.material.transparency) * 255.0) as u8;
                }
                
                // Dibujar el modelo en su POSICIÓN ORIGINAL (la cámara está rotada)
                d3d.draw_model_ex(
                    model,
                    block.position, // POSICIÓN ORIGINAL
                    Vector3::new(0.0, 1.0, 0.0), // Eje de rotación
                    0.0,                          // SIN rotación del modelo
                    block.scale,                  // Escala
                    tint,
                );
                
                // Dibujar bordes para mejor definición
                if block.material.transparency < 0.5 {
                    d3d.draw_cube_wires(
                        block.position, // POSICIÓN ORIGINAL
                        block.scale.x * 2.0,
                        block.scale.y * 2.0, 
                        block.scale.z * 2.0,
                        Color::new(0, 0, 0, 50),
                    );
                }
            }

            // Dibujar indicador de la posición de la luz
            d3d.draw_sphere(light_position, 0.5, Color::YELLOW);
        }

        // UI overlay mejorada con controles actualizados
        d.draw_text("Diorama Minecraft", 10, 10, 20, Color::RED);
        d.draw_text("Controles (según documento MD):", 10, 40, 16, Color::RED);
        d.draw_text("- Click Izq + Arrastrar: Rotar cámara orbital", 10, 60, 14, Color::RED);
        d.draw_text("- Rueda del Mouse: Zoom (2.0-40.0 unidades)", 10, 80, 14, Color::RED);
        d.draw_text("- A/D: Rotación manual del diorama", 10, 100, 14, Color::RED);
        d.draw_text("- W/S: Zoom manual alternativo", 10, 120, 14, Color::RED);
        d.draw_text("- ESPACIO: Toggle auto-rotación orbital", 10, 140, 14, Color::RED);
    }
}