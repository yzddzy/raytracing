use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Sphere {
    center: Vec3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let oc = r.origin() - self.center.clone();
        let a = r.direction().squared_length();
        let half_b = oc.clone() * r.direction();
        let c = oc.clone().squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.clone().p - self.center.clone()) / self.radius;
        rec.set_face_normal(r, outward_normal);
        return true
    }
}