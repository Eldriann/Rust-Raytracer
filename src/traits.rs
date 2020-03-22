use crate::shape::{Ray, Hit, Point};
use crate::vertors::Vector3;
use crate::rendering::Color;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

pub trait LightEmitter {
    fn get_direction(&self, point: Point) -> Vector3;
    fn get_brightness(&self, point: Point) -> f64;
    fn get_color(&self) -> Color;
    fn get_distance(&self, point: Point) -> f64;
}