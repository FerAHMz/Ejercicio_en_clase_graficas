use raylib::prelude::*;

// Función para rotar un vector en 3D
fn rotate_vector(v: Vector3, angle_x: f32, angle_y: f32, angle_z: f32) -> Vector3 {
    let mut result = v;
    
    // Rotación alrededor del eje Y
    let cos_y = angle_y.cos();
    let sin_y = angle_y.sin();
    let temp_x = result.x * cos_y - result.z * sin_y;
    let temp_z = result.x * sin_y + result.z * cos_y;
    result.x = temp_x;
    result.z = temp_z;
    
    // Rotación alrededor del eje X
    let cos_x = angle_x.cos();
    let sin_x = angle_x.sin();
    let temp_y = result.y * cos_x - result.z * sin_x;
    let temp_z2 = result.y * sin_x + result.z * cos_x;
    result.y = temp_y;
    result.z = temp_z2;
    
    // Rotación alrededor del eje Z
    let cos_z = angle_z.cos();
    let sin_z = angle_z.sin();
    let temp_x2 = result.x * cos_z - result.y * sin_z;
    let temp_y2 = result.x * sin_z + result.y * cos_z;
    result.x = temp_x2;
    result.y = temp_y2;
    
    result
}

// Función para calcular iluminación difusa
fn calculate_diffuse_lighting(
    surface_position: Vector3,
    surface_normal: Vector3,
    light_position: Vector3,
    base_color: Color,
    ambient_intensity: f32,
    diffuse_intensity: f32,
) -> Color {
    // Vector de la superficie hacia la luz
    let light_direction = (light_position - surface_position).normalized();
    
    // Calcular el producto punto entre la normal de la superficie y la dirección de la luz
    let dot_product = surface_normal.dot(light_direction).max(0.0);
    
    // Calcular la iluminación total (ambiente + difusa)
    let lighting = ambient_intensity + (diffuse_intensity * dot_product);
    let lighting = lighting.min(1.0);
    
    // Aplicar la iluminación al color base
    Color::new(
        (base_color.r as f32 * lighting) as u8,
        (base_color.g as f32 * lighting) as u8,
        (base_color.b as f32 * lighting) as u8,
        base_color.a,
    )
}

fn main() {
    // Configuración inicial de la ventana
    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("Diorama del Cubo Simple - Rotación y Cámara")
        .build();

    // Configurar la cámara 3D
    let mut camera = Camera3D::perspective(
        Vector3::new(5.0, 5.0, 5.0),  // Posición de la cámara
        Vector3::new(0.0, 0.0, 0.0),  // Punto al que mira
        Vector3::new(0.0, 1.0, 0.0),  // Vector up
        45.0,                         // FOV
    );

    // Crear una textura procedural para el cubo
    let image = Image::gen_image_checked(64, 64, 8, 8, Color::WHITE, Color::GRAY);
    let _cube_texture = rl.load_texture_from_image(&thread, &image);

    // Variables para la animación del cubo
    let mut rotation_x = 0.0f32;
    let mut rotation_y = 0.0f32;
    let mut rotation_z = 0.0f32;

    // Variables para la luz difusa
    let light_position = Vector3::new(3.0, 4.0, 2.0); // Posición fija de la luz
    let ambient_intensity = 0.3; // Intensidad de luz ambiente
    let diffuse_intensity = 0.7; // Intensidad de luz difusa

    // Configurar FPS
    rl.set_target_fps(60);

    // Loop principal
    while !rl.window_should_close() {
        // Actualizar rotaciones del cubo
        rotation_x += 20.0 * rl.get_frame_time(); // Rotación en X
        rotation_y += 30.0 * rl.get_frame_time(); // Rotación en Y
        rotation_z += 25.0 * rl.get_frame_time(); // Rotación en Z

        // Control de cámara: acercar/alejar con rueda del mouse, rotar con mouse
        rl.update_camera(&mut camera, CameraMode::CAMERA_ORBITAL);

        // Control adicional de zoom con teclas
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            let direction = (camera.target - camera.position).normalized();
            camera.position = camera.position + direction * 2.0 * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            let direction = (camera.target - camera.position).normalized();
            camera.position = camera.position - direction * 2.0 * rl.get_frame_time();
        }

        // Inicio del renderizado
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(50, 50, 50, 255)); // Fondo gris oscuro

        {
            // Modo 3D
            let mut d3d = d.begin_mode3D(camera);

            // Dibujar un plano como base (suelo)
            d3d.draw_plane(
                Vector3::new(0.0, -2.0, 0.0),
                Vector2::new(10.0, 10.0),
                Color::new(100, 100, 100, 255), // Gris
            );

            // === CUBO PRINCIPAL CON ROTACIÓN E ILUMINACIÓN DIFUSA ===
            
            // Aplicar transformaciones manuales para la rotación
            // Primero dibujamos la sombra del cubo en el plano
            d3d.draw_cube(
                Vector3::new(0.5, -1.99, 0.5), // Sombra ligeramente desplazada
                2.2, 0.01, 2.2,
                Color::new(20, 20, 20, 180), // Sombra oscura semi-transparente
            );

            // Dibujar el cubo principal con color fijo azul
            let cube_position = Vector3::new(0.0, 0.0, 0.0);
            let base_cube_color = Color::new(100, 150, 255, 255); // Azul base
            
            // Calcular iluminación para diferentes caras del cubo (aplicando rotación a las normales)
            // Cara frontal (normal hacia +Z)
            let front_normal = rotate_vector(Vector3::new(0.0, 0.0, 1.0), rotation_x.to_radians(), rotation_y.to_radians(), rotation_z.to_radians());
            let front_color = calculate_diffuse_lighting(
                cube_position,
                front_normal,
                light_position,
                base_cube_color,
                ambient_intensity,
                diffuse_intensity,
            );
            
            // Cara superior (normal hacia +Y)
            let top_normal = rotate_vector(Vector3::new(0.0, 1.0, 0.0), rotation_x.to_radians(), rotation_y.to_radians(), rotation_z.to_radians());
            let top_color = calculate_diffuse_lighting(
                cube_position,
                top_normal,
                light_position,
                base_cube_color,
                ambient_intensity,
                diffuse_intensity,
            );
            
            // Cara derecha (normal hacia +X)
            let right_normal = rotate_vector(Vector3::new(1.0, 0.0, 0.0), rotation_x.to_radians(), rotation_y.to_radians(), rotation_z.to_radians());
            let right_color = calculate_diffuse_lighting(
                cube_position,
                right_normal,
                light_position,
                base_cube_color,
                ambient_intensity,
                diffuse_intensity,
            );
            
            // Dibujar el cubo principal (usaremos el color promedio para simplicidad)
            let avg_lighting = (front_color.r as f32 + top_color.r as f32 + right_color.r as f32) / (3.0 * 255.0);
            let lit_cube_color = Color::new(
                (base_cube_color.r as f32 * avg_lighting) as u8,
                (base_cube_color.g as f32 * avg_lighting) as u8,
                (base_cube_color.b as f32 * avg_lighting) as u8,
                255,
            );
            
            d3d.draw_cube(
                cube_position,
                2.0, 2.0, 2.0,
                lit_cube_color,
            );

            // Dibujar las aristas del cubo para mayor definición
            d3d.draw_cube_wires(
                cube_position,
                2.0, 2.0, 2.0,
                Color::BLACK,
            );

        }
    }
}
