use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub mat: Box<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, mat: impl Material + 'a) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            mat: Box::new(mat),
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.lengh_squared();
        let h = Vec3::dot(r.direction, oc);
        let c = oc.lengh_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        let mut rec = HitRecord::new(p, normal, t, &*self.mat);
        rec.set_face_normal(r);

        Some(rec)
    }
}
