use raylib::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
    pub material_index: usize,
    pub u: f32, // Coordenada de textura U
    pub v: f32, // Coordenada de textura V
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vector3::zero(),
            normal: Vector3::zero(),
            t: 0.0,
            front_face: true,
            material_index: 0,
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material_index: usize,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material_index: usize) -> Self {
        Self {
            center,
            radius,
            material_index,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let dir = ray.direction;
        let a = dir.x * dir.x + dir.y * dir.y + dir.z * dir.z;
        let half_b = oc.dot(ray.direction);
        let c = (oc.x * oc.x + oc.y * oc.y + oc.z * oc.z) - self.radius * self.radius;
        
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        
        let sqrtd = discriminant.sqrt();
        
        // Encontrar la raíz más cercana que esté en el rango aceptable
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        
        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material_index = self.material_index;
        
        // Calcular coordenadas UV para la esfera
        let theta = (-outward_normal.y).acos();
        let phi = (-outward_normal.z).atan2(outward_normal.x) + std::f32::consts::PI;
        rec.u = phi / (2.0 * std::f32::consts::PI);
        rec.v = theta / std::f32::consts::PI;
        
        true
    }
}

#[derive(Clone)]
pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub material_index: usize,
}

impl Cube {
    pub fn new(center: Vector3, size: Vector3, material_index: usize) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
            material_index,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut t_near = t_min;
        let mut t_far = t_max;
        let mut hit_normal = Vector3::zero();
        
        // Chequear intersección con cada par de planos paralelos
        for i in 0..3 {
            let ray_dir_comp = match i {
                0 => ray.direction.x,
                1 => ray.direction.y,
                _ => ray.direction.z,
            };
            
            let ray_orig_comp = match i {
                0 => ray.origin.x,
                1 => ray.origin.y,
                _ => ray.origin.z,
            };
            
            let min_comp = match i {
                0 => self.min.x,
                1 => self.min.y,
                _ => self.min.z,
            };
            
            let max_comp = match i {
                0 => self.max.x,
                1 => self.max.y,
                _ => self.max.z,
            };
            
            if ray_dir_comp.abs() < 1e-6 {
                // Rayo paralelo a los planos
                if ray_orig_comp < min_comp || ray_orig_comp > max_comp {
                    return false;
                }
            } else {
                let t1 = (min_comp - ray_orig_comp) / ray_dir_comp;
                let t2 = (max_comp - ray_orig_comp) / ray_dir_comp;
                
                let (t_min_plane, t_max_plane) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
                
                if t_min_plane > t_near {
                    t_near = t_min_plane;
                    // Determinar la normal de la cara golpeada
                    hit_normal = Vector3::zero();
                    match i {
                        0 => hit_normal.x = if ray_dir_comp > 0.0 { -1.0 } else { 1.0 },
                        1 => hit_normal.y = if ray_dir_comp > 0.0 { -1.0 } else { 1.0 },
                        _ => hit_normal.z = if ray_dir_comp > 0.0 { -1.0 } else { 1.0 },
                    }
                }
                
                if t_max_plane < t_far {
                    t_far = t_max_plane;
                }
                
                if t_near > t_far {
                    return false;
                }
            }
        }
        
        if t_near >= t_min && t_near <= t_max {
            rec.t = t_near;
            rec.point = ray.at(rec.t);
            rec.set_face_normal(ray, hit_normal);
            rec.material_index = self.material_index;
            
            // Calcular coordenadas UV para el cubo basadas en la cara golpeada
            let rel_point = rec.point - Vector3::new(
                (self.min.x + self.max.x) * 0.5,
                (self.min.y + self.max.y) * 0.5,
                (self.min.z + self.max.z) * 0.5,
            );
            
            // Determinar coordenadas UV basadas en la cara normal
            if hit_normal.x.abs() > 0.5 {
                // Cara X
                rec.u = (rel_point.z + (self.max.z - self.min.z) * 0.5) / (self.max.z - self.min.z);
                rec.v = (rel_point.y + (self.max.y - self.min.y) * 0.5) / (self.max.y - self.min.y);
            } else if hit_normal.y.abs() > 0.5 {
                // Cara Y (superior/inferior)
                rec.u = (rel_point.x + (self.max.x - self.min.x) * 0.5) / (self.max.x - self.min.x);
                rec.v = (rel_point.z + (self.max.z - self.min.z) * 0.5) / (self.max.z - self.min.z);
            } else {
                // Cara Z
                rec.u = (rel_point.x + (self.max.x - self.min.x) * 0.5) / (self.max.x - self.min.x);
                rec.v = (rel_point.y + (self.max.y - self.min.y) * 0.5) / (self.max.y - self.min.y);
            }
            
            // Asegurar que las coordenadas UV estén en [0,1]
            rec.u = rec.u.clamp(0.0, 1.0);
            rec.v = rec.v.clamp(0.0, 1.0);
            
            true
        } else {
            false
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
