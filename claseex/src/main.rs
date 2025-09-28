use raylib::prelude::*;

mod materials;
mod cubes;
mod math_utils;
mod raytracer;
mod camera;
mod skybox;
mod ray;

use cubes::{create_world, create_raytraced_scene};
use raytracer::RayTracer;
use camera::CameraController;
use materials::{Material, TextureSampler};

fn main() {
    // Configuración inicial de la ventana (resolución reducida para raytracing)
    let width = 800;
    let height = 600;
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Raytraced Minecraft Diorama - Reflections & Refractions")
        .build();

    // Configurar controlador de cámara
    let mut camera_controller = CameraController::new();

    // Inicializar raytracer con configuración optimizada para interacción fluida
    let mut raytracer = RayTracer::with_quality(width as u32, height as u32, 2); // Calidad baja para velocidad
    let mut current_quality = 2;

    // Crear y cargar texture sampler con imágenes PNG reales
    let mut texture_sampler = TextureSampler::new();
    let texture_files = vec![
        "assets/iron_block.png",
        "assets/diamond_block.png", 
        "assets/grass_carried.png",              // Textura superior del césped
        "assets/grass_side_carried.png",         // Textura lateral césped+tierra
        "assets/dirt.png",
        "assets/water_still.png",
        "assets/glass.png"
    ];
    
    println!("Loading textures...");
    for texture_file in &texture_files {
        if let Err(e) = texture_sampler.load_texture(texture_file) {
            println!("Warning: Failed to load {}: {}", texture_file, e);
        }
    }
    println!("Textures loaded!");

    // Crear materiales para raytracing
    let materials = vec![
        Material::iron(),      // índice 0
        Material::diamond(),   // índice 1
        Material::grass(),     // índice 2
        Material::dirt(),      // índice 3
        Material::water(),     // índice 4
        Material::glass(),     // índice 5
    ];

    // Crear escena de raytracing
    let (scene, _) = create_raytraced_scene(&materials);

    // Crear imagen para raytracing
    let mut raytraced_image = Image::gen_image_color(width, height, Color::BLACK);
    let mut raytraced_texture = rl.load_texture_from_image(&thread, &raytraced_image).unwrap();

    // Variables de control
    let mut use_raytracing = true;
    let mut time = 0.0f32;

    // Configurar FPS reducido para raytracing
    rl.set_target_fps(30);

    // Variables para control de rendering optimizadas
    let mut last_render_time = 0.0f32;
    let mut last_interaction_time = 0.0f32;
    let mut render_interval = 1.0 / 20.0; // Más frecuente para mejor interacción (0.05 segundos)

    // Loop principal
    while !rl.window_should_close() {
        // Actualizar tiempo
        let frame_time = rl.get_frame_time();
        time += frame_time;
        
        // Actualizar controlador de cámara
        camera_controller.update(&mut rl, frame_time);

        // Control para cambiar entre raytracing y rasterizado
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            use_raytracing = !use_raytracing;
        }

        // Controles dinámicos de calidad del raytracing
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) && current_quality != 1 {
            raytracer = RayTracer::with_quality(width as u32, height as u32, 1);
            current_quality = 1;
            println!("Quality set to: Minimum (fastest)");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) && current_quality != 2 {
            raytracer = RayTracer::with_quality(width as u32, height as u32, 2);
            current_quality = 2;
            println!("Quality set to: Low (fast)");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) && current_quality != 3 {
            raytracer = RayTracer::with_quality(width as u32, height as u32, 3);
            current_quality = 3;
            println!("Quality set to: Medium (balanced)");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) && current_quality != 4 {
            raytracer = RayTracer::with_quality(width as u32, height as u32, 4);
            current_quality = 4;
            println!("Quality set to: High (slow)");
        }

        let camera = camera_controller.get_camera();
        let fps = rl.get_fps();

        // Detectar interacción del usuario para ajustar frecuencia de renderizado
        let is_interacting = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) ||
                            rl.get_mouse_wheel_move() != 0.0 ||
                            rl.is_key_down(KeyboardKey::KEY_W) ||
                            rl.is_key_down(KeyboardKey::KEY_S) ||
                            rl.is_key_down(KeyboardKey::KEY_A) ||
                            rl.is_key_down(KeyboardKey::KEY_D);
        
        if is_interacting {
            last_interaction_time = time;
            render_interval = 1.0 / 30.0; // Renderizar más frecuente durante interacción
        } else if (time - last_interaction_time) > 2.0 {
            render_interval = 1.0 / 15.0; // Frecuencia normal cuando no hay interacción
        }

        // Renderizar raytracing con intervalo adaptativo
        if use_raytracing && (time - last_render_time) > render_interval {
            println!("Rendering raytraced frame... (this may take a moment)");
            
            let pixels = raytracer.render(
                &scene,
                &materials,
                &texture_sampler,
                camera.position,
                camera.target,
                camera.up,
                45.0,
            );

            // Actualizar la textura con los nuevos píxeles
            for (i, color) in pixels.iter().enumerate() {
                let x = (i % (width as usize)) as i32;
                let y = (i / (width as usize)) as i32;
                raytraced_image.draw_pixel(x, y, *color);
            }
            
            raytraced_texture = rl.load_texture_from_image(&thread, &raytraced_image).unwrap();
            last_render_time = time;
            println!("Raytracing complete!");
        }

        // Inicio del renderizado
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(20, 20, 40, 255));

        if use_raytracing {
            // Mostrar imagen raytraceada
            d.draw_texture(&raytraced_texture, 0, 0, Color::WHITE);
            
            // Overlay de información
            d.draw_text("RAYTRACING MODE", 10, 10, 24, Color::YELLOW);
            d.draw_text("Press 'R' to toggle rasterization", 10, 40, 16, Color::WHITE);
            d.draw_text("Rendering in progress...", 10, 60, 14, Color::LIGHTGRAY);
            
        } else {
            // Modo rasterizado legacy para comparación
            {
                let mut d3d = d.begin_mode3D(camera);
                
                // Crear mundo legacy para comparación
                let world = create_world();
                
                for block in &world {
                    d3d.draw_cube(
                        block.position,
                        2.0, 2.0, 2.0,
                        Color::new(
                            (block.material.albedo.x * 255.0) as u8,
                            (block.material.albedo.y * 255.0) as u8,
                            (block.material.albedo.z * 255.0) as u8,
                            255
                        )
                    );
                }
            }
            
            d.draw_text("RASTERIZATION MODE", 10, 10, 24, Color::GREEN);
            d.draw_text("Press 'R' to toggle raytracing", 10, 40, 16, Color::WHITE);
        }

        // Controles
        d.draw_text("Controls:", 10, height - 140, 16, Color::WHITE);
        d.draw_text("- Click + Drag: Rotate camera", 10, height - 120, 14, Color::LIGHTGRAY);
        d.draw_text("- Mouse Wheel: Zoom", 10, height - 100, 14, Color::LIGHTGRAY);
        d.draw_text("- WASD: Manual control", 10, height - 80, 14, Color::LIGHTGRAY);
        d.draw_text("- SPACE: Auto-rotate", 10, height - 60, 14, Color::LIGHTGRAY);
        d.draw_text("- R: Toggle raytracing", 10, height - 40, 14, Color::LIGHTGRAY);
        
        // Mostrar FPS
        d.draw_text(&format!("FPS: {}", fps), 10, height - 20, 14, Color::WHITE);
    }
}