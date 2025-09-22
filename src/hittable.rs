use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = Vec3::dot(r.direction, self.normal) < 0.0;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
