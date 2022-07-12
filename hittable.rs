use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct hit_record {
    pub p: Vec3,      
    pub normal: Vec3, 
    pub t: f64,    
    pub front_face: bool,
}
impl hit_record{
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = (r.direction() * outward_normal.clone()) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool;
}
pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.list.push(object);
    }
    pub fn clear(&mut self) {
        self.list.clear();
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let mut temp_rec = hit_record::new(Vec3::zero(),Vec3::zero(),0.0,false);
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.list {
            if object.hit(ray.clone(), t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}