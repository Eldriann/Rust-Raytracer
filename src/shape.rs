use serde::{Serialize, Deserialize};
use crate::vertors::Vector3;
use crate::rendering::{Camera, SHADOW_BIAS};
use crate::traits::Intersectable;
use std::mem::swap;

pub type Point = Vector3;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn compute_prime_ray(x: u32, y: u32, camera: Camera) -> Ray {
        camera.compute_prime_ray(x, y)
    }

    pub fn compute_reflection_ray(normal: Vector3, old_direction: Vector3, point: Point) -> Ray {
        Ray {
            origin: point + (normal * SHADOW_BIAS),
            direction: old_direction - (normal * 2.0 * old_direction.dot(&normal))
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Hit {
    pub distance: f64,
    pub point: Point,
    pub normal: Vector3
}

impl Hit {
    pub fn new(distance: f64, point: Point, normal: Vector3) -> Hit {
        Hit { distance, point, normal }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub origin: Point,
    pub radius: f64
}

impl Sphere {
    pub fn new(origin: Point, radius: f64) -> Sphere {
        Sphere { origin, radius }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let ray_origin_to_sphere = self.origin - ray.origin;
        let ray_origin_to_sphere_proj = ray_origin_to_sphere.dot(&ray.direction);
        let sphere_center_to_proj_squared = ray_origin_to_sphere.dot(&ray_origin_to_sphere) - ray_origin_to_sphere_proj * ray_origin_to_sphere_proj;
        if sphere_center_to_proj_squared > self.radius * self.radius {
            return None; // Does not hit
        }
        let thickness = (self.radius * self.radius - sphere_center_to_proj_squared).sqrt();
        let mut intersect_0 = ray_origin_to_sphere_proj - thickness;
        let mut intersect_1 = ray_origin_to_sphere_proj + thickness;

        if intersect_0 > intersect_1 {
            swap(&mut intersect_0, &mut intersect_1);
        }

        if intersect_0 < 0.0 && intersect_1 < 0.0 {
            None
        } else if intersect_0 >= 0.0 {
            let hit_point = ray.origin + ray.direction * intersect_0;
            let normal = (hit_point - self.origin).normalize();
            Some(Hit::new(intersect_0, hit_point, normal))
        } else {
            let hit_point = ray.origin + ray.direction * intersect_1;
            let normal = (hit_point - self.origin).normalize();
            Some(Hit::new(intersect_1, hit_point, normal))
        }

    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Plane {
    pub point: Point,
    pub normal: Vector3
}

impl Plane {
    pub fn new(point: Point, normal: Vector3) -> Plane {
        Plane { point, normal }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let denom = self.normal.dot(&ray.direction);
        if denom > 0.0 {
            let origin_to_plane = self.point - ray.origin;
            let distance = origin_to_plane.dot(&self.normal) / denom;
            if distance >= 0.0 {
                return Some(Hit::new(distance, ray.origin + ray.direction * distance, -self.normal));
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Shape {
    SPHERE(Sphere),
    PLANE(Plane)
}

impl Intersectable for Shape {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Shape::SPHERE(s) => s.intersect(ray),
            Shape::PLANE(p) => p.intersect(ray)
        }
    }
}