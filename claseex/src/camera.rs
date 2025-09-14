use raylib::prelude::*;

pub struct CameraController {
    pub camera: Camera3D,
    pub auto_rotate: bool,
    
    // Sistema de coordenadas esféricas para rotación orbital
    pub distance: f32,        // Distancia al objetivo
    pub theta: f32,           // Ángulo horizontal (radianes)
    pub phi: f32,             // Ángulo vertical (radianes)
    pub min_distance: f32,    // Límite mínimo de zoom
    pub max_distance: f32,    // Límite máximo de zoom
    pub target: Vector3,      // Punto al que mira la cámara
    
    // Sensibilidades de control
    pub rotation_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub auto_rotation_speed: f32,
}

impl CameraController {
    pub fn new() -> Self {
        let target = Vector3::new(0.0, 0.0, 0.0);
        let distance = 25.0;
        let theta = 0.0;
        let phi = std::f32::consts::PI * 0.3; // 54 grados desde arriba
        
        let mut controller = Self {
            camera: Camera3D::perspective(
                Vector3::new(0.0, 0.0, 0.0), // Se actualizará con update_position
                target,
                Vector3::new(0.0, 1.0, 0.0),
                45.0,
            ),
            auto_rotate: true,
            distance,
            theta,
            phi,
            min_distance: 2.0,
            max_distance: 40.0, // Reducido de 50 a 40 para evitar skybox
            target,
            rotation_sensitivity: 0.01,
            zoom_sensitivity: 0.5,
            auto_rotation_speed: 0.005,
        };
        
        controller.update_position();
        controller
    }
    
    /// Actualiza la posición de la cámara usando coordenadas esféricas
    pub fn update_position(&mut self) {
        // Convertir coordenadas esféricas (distance, theta, phi) a cartesianas (x, y, z)
        let x = self.distance * self.phi.sin() * self.theta.cos();
        let y = self.distance * self.phi.cos();
        let z = self.distance * self.phi.sin() * self.theta.sin();
        
        self.camera.position = self.target + Vector3::new(x, y, z);
        self.camera.target = self.target;
    }
    
    /// Rota la cámara usando deltas de ángulos
    pub fn rotate(&mut self, delta_theta: f32, delta_phi: f32) {
        self.theta += delta_theta;  // Rotación horizontal (izquierda/derecha)
        self.phi += delta_phi;      // Rotación vertical (arriba/abajo)
        
        // Prevenir gimbal lock limitando phi entre 0.1 y π-0.1
        self.phi = self.phi.clamp(0.1, std::f32::consts::PI - 0.1);
        
        self.update_position(); // Recalcular posición cartesiana
    }
    
    /// Controla el zoom (acercar/alejar)
    pub fn zoom(&mut self, delta: f32) {
        self.distance += delta;
        // Mantener distancia dentro de límites seguros
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
        self.update_position();
    }
    
    
    pub fn update(&mut self, rl: &mut RaylibHandle, _frame_time: f32) {
        // Rotación automática continua
        if self.auto_rotate {
            self.rotate(self.auto_rotation_speed, 0.0); // Solo rotación horizontal
        }
        
        // Control de rotación manual con mouse (click izquierdo + arrastre)
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_delta = rl.get_mouse_delta();
            if mouse_delta.x != 0.0 || mouse_delta.y != 0.0 {
                self.rotate(
                    mouse_delta.x * self.rotation_sensitivity,  // Sensibilidad horizontal
                    mouse_delta.y * self.rotation_sensitivity,  // Sensibilidad vertical
                );
                self.auto_rotate = false; // Desactivar auto-rotación al interactuar
            }
        }
        
        // Control de zoom con rueda del mouse
        let wheel_move = rl.get_mouse_wheel_move();
        if wheel_move != 0.0 {
            self.zoom(-wheel_move * self.zoom_sensitivity); // Dirección intuitiva
        }
        
        // Reactivar rotación automática con SPACE
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.auto_rotate = !self.auto_rotate;
        }
        
        // Control manual adicional con teclas A/D para compatibilidad
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.rotate(-0.02, 0.0);
            self.auto_rotate = false;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.rotate(0.02, 0.0);
            self.auto_rotate = false;
        }
        
        // Control manual de zoom con teclas W/S
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.zoom(-0.5);
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.zoom(0.5);
        }
    }
    
    pub fn get_rotated_camera(&self) -> Camera3D {
        // Retornar la cámara actual ya que la posición se actualiza automáticamente
        self.camera
    }
}