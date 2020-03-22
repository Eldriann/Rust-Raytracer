use serde::{Serialize, Deserialize};
use crate::shape::{Shape, Ray, Hit, Point};
use crate::vertors::Vector3;
use image::{ImageBuffer, RgbaImage, Rgba, Pixel, ImageError};
use crate::traits::{Intersectable, LightEmitter};

pub const SHADOW_BIAS: f64 = 1e-13;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0, a: 255 }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(self.r, self.g, self.b, self.a)
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        if (self.r as u16) + (rhs.r as u16) < 255 {
            self.r += rhs.r;
        } else {
            self.r = 255;
        }
        if (self.g as u16) + (rhs.g as u16) < 255 {
            self.g += rhs.g;
        } else {
            self.g = 255;
        }
        if (self.b as u16) + (rhs.b as u16) < 255 {
            self.b += rhs.b;
        } else {
            self.b = 255;
        }
        if (self.a as u16) + (rhs.a as u16) < 255 {
            self.a += rhs.a;
        } else {
            self.a = 255;
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    pub base_color: Color,
    pub albedo: f64,
    pub reflectiveness: f64
}

impl Material {
    pub fn new(base_color: Color, albedo: f64, reflectiveness: f64) -> Material {
        Material { base_color, albedo, reflectiveness }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub brightness: f64,
    pub color: Color
}

impl DirectionalLight {
    pub fn new(direction: Vector3, brightness: f64, color: Color) -> DirectionalLight {
        DirectionalLight { direction, brightness, color }
    }
}

impl LightEmitter for DirectionalLight {
    fn get_direction(&self, _point: Point) -> Vector3 {
        -self.direction.normalize()
    }

    fn get_brightness(&self, _point: Point) -> f64 {
        self.brightness
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_distance(&self, _point: Point) -> f64 {
        std::f64::INFINITY
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PointLight {
    pub position: Point,
    pub brightness: f64,
    pub color: Color
}

impl PointLight {
    pub fn new(position: Point, brightness: f64, color: Color) -> PointLight {
        PointLight { position, brightness, color }
    }
}

impl LightEmitter for PointLight {
    fn get_direction(&self, point: Point) -> Vector3 {
        (self.position - point).normalize()
    }

    fn get_brightness(&self, point: Point) -> f64 {
        let light_distance_sq = (self.position - point).length_sq();
        self.brightness / (4.0 * std::f64::consts::PI * light_distance_sq)
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_distance(&self, point: Point) -> f64 {
        (self.position - point).length()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Light {
    POINT(PointLight),
    DIRECTIONAL(DirectionalLight)
}

impl LightEmitter for Light {
    fn get_direction(&self, point: Point) -> Vector3 {
        match self {
            Light::POINT(l) => l.get_direction(point),
            Light::DIRECTIONAL(l) => l.get_direction(point),
        }
    }

    fn get_brightness(&self, point: Point) -> f64 {
        match self {
            Light::POINT(l) => l.get_brightness(point),
            Light::DIRECTIONAL(l) => l.get_brightness(point),
        }
    }

    fn get_color(&self) -> Color {
        match self {
            Light::POINT(l) => l.get_color(),
            Light::DIRECTIONAL(l) => l.get_color(),
        }
    }

    fn get_distance(&self, point: Point) -> f64 {
        match self {
            Light::POINT(l) => l.get_distance(point),
            Light::DIRECTIONAL(l) => l.get_distance(point),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Renderable {
    pub shape: Shape,
    pub material: Material
}

impl Renderable {
    pub fn new(shape: Shape, material: Material) -> Renderable {
        Renderable { shape, material }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: f64) -> Camera {
        Camera { width, height, fov }
    }

    pub fn compute_prime_ray(&self, pixel_x_screen_space: u32, pixel_y_screen_space: u32) -> Ray {
        assert!(self.width >= self.height);
        assert!(pixel_x_screen_space <= self.width);
        assert!(pixel_y_screen_space <= self.height);
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        let dir_x = (((pixel_x_screen_space as f64 + 0.5) / self.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let dir_y = (1.0 - ((pixel_y_screen_space as f64 + 0.5) / self.height as f64) * 2.0) * fov_adjustment;

        Ray::new(Vector3::zero(), Vector3::new(dir_x, dir_y, -1.0).normalize())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub elements: Vec<Renderable>,
    pub lights: Vec<Light>,
    pub sky_color: Color
}

impl Scene {
    pub fn new(camera: Camera, elements: Vec<Renderable>, lights: Vec<Light>, sky_color: Color) -> Scene {
        Scene { camera, elements, lights, sky_color }
    }

    pub fn trace(&self, ray: &Ray) -> Option<(Renderable, Hit)> {
        let mut min_distance = std::f64::MAX;
        let mut object: Option<(Renderable, Hit)> = None;
        for &renderable in self.elements.iter() {
            if let Some(hit) = renderable.shape.intersect(ray) {
                if min_distance > hit.distance {
                    min_distance = hit.distance;
                    object = Some((renderable, hit));
                }
            }
        }
        object
    }

    pub fn get_color(&self, ray: &Ray, hit_obj: Option<(Renderable, Hit)>, depth: u8, max_depth: u8) -> Color {
        if let Some((renderable, hit)) = hit_obj {
            if depth >= max_depth {
                return Color::black();
            }
            let mut color_r: f64 = 0.0;
            let mut color_g: f64 = 0.0;
            let mut color_b: f64 = 0.0;
            let amount_reflected = renderable.material.albedo / std::f64::consts::PI;
            for light in self.lights.iter() {
                let light_direction = light.get_direction(hit.point);
                let mut light_brightness = light.get_brightness(hit.point);
                let light_ray = Ray::new(hit.point + (hit.normal * SHADOW_BIAS), light_direction);
                if let Some((_, hit_light)) = self.trace(&light_ray) {
                    if (hit_light.distance <= light.get_distance(hit.point)) {
                        light_brightness = 0.0;
                    }
                }
                let light_power = (hit.normal.dot(&light_direction)).max(0.0) * light_brightness;
                color_r += (((light.get_color().r as f64) / 255.0) * light_power * amount_reflected) * ((renderable.material.base_color.r as f64) / 255.0);
                color_g += (((light.get_color().g as f64) / 255.0) * light_power * amount_reflected) * ((renderable.material.base_color.g as f64) / 255.0);
                color_b += (((light.get_color().b as f64) / 255.0) * light_power * amount_reflected) * ((renderable.material.base_color.b as f64) / 255.0);
            }
            color_r = color_r.min(1.0).max(0.0);
            color_g = color_g.min(1.0).max(0.0);
            color_b = color_b.min(1.0).max(0.0);

            if renderable.material.reflectiveness > 0.0 {
                let reflection_ray = Ray::compute_reflection_ray(hit.normal, ray.direction, hit.point);
                color_r = color_r * (1.0 - renderable.material.reflectiveness);
                color_g = color_g * (1.0 - renderable.material.reflectiveness);
                color_b = color_b * (1.0 - renderable.material.reflectiveness);
                let mut color = Color::new((color_r * 255.0) as u8, (color_g * 255.0) as u8, (color_b * 255.0) as u8, 255);
                let new_obj = self.trace(&reflection_ray);
                color += self.get_color(&reflection_ray, new_obj, depth + 1, max_depth);
                color
            } else {
                Color::new((color_r * 255.0) as u8, (color_g * 255.0) as u8, (color_b * 255.0) as u8, 255)
            }
        } else {
            self.sky_color
        }
    }
}

pub fn render(nb_pass: u8, scene: Scene, output_path: String) -> Result<(), ImageError> {
    let mut image: RgbaImage = ImageBuffer::new(scene.camera.width, scene.camera.height);
    for pixel_x in 0..scene.camera.width {
        for pixel_y in 0..scene.camera.height {
            let ray = Ray::compute_prime_ray(pixel_x, pixel_y, scene.camera);
            let object = scene.trace(&ray);
            let color = scene.get_color(&ray, object, 0, nb_pass);
            image.put_pixel(pixel_x, pixel_y, color.to_rgba())
        }
    }
    image.save(output_path)
}