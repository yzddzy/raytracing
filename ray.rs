use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]

pub struct Ray{
    pub orig:Vec3,
    pub dir:Vec3,
}

impl Ray {
    pub fn new(orig:Vec3,dir:Vec3)->Self{
        Self {orig ,dir}
    }
    pub fn origin(&self)->Vec3{
        self.orig.clone()
    }
    pub fn direction(&self)->Vec3{
        self.dir.clone()
    }
    pub fn at(&self,t:f64)->Vec3{
        self.orig.clone()+self.dir.clone()*t
    } 
}
