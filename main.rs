
mod vec3;
pub use vec3::Vec3;
mod ray;
pub use ray::Ray;
type Point3=Vec3;
type Color=Vec3;
mod hittable;
pub use hittable::*;
mod sphere;
use std::f64::consts::PI;
use std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}
fn main(){
    const aspect_ratio:f64 = 16.0 / 9.0;
    const image_width:u32 = 400;
    const image_height:u32 =(image_width as f64/ aspect_ratio)as u32;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone() - horizontal.clone()/2.0 - vertical.clone()/2.0 - Vec3::new(0.0, 0.0, focal_length);
    let mut world = hittable::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(Vec3::new(0.0, 0.0, -1.0),0.5,)));
    world.add(Box::new(sphere::Sphere::new(Vec3::new(0.0, -100.5, -1.0),100.0,)));
    println!("P3");
    println!{"{} {}",image_width, image_height };
    println!{"255"};
    for j in (0..image_height).rev(){
        for i in (0..image_width){
            let mut pixel_color = Vec3::zero();
            
                let u = (i as f64 ) / (image_width as f64);
                let v = (j as f64 ) / (image_height as f64);
                let r=Ray{
                    orig:origin.clone(), 
                    dir:lower_left_corner.clone() + horizontal.clone()*u + vertical.clone()*v - origin.clone(),
                };
                pixel_color = ray_color(&r,&world);
            
            write_color(pixel_color);
        }
    }
}
pub fn ray_color(r:&Ray,world:&hittable::HittableList)->Vec3{
    let mut rec = hit_record::new(Vec3::zero(),Vec3::zero(),0.0,false);
    if world.hit((*r).clone(),0.0,INFINITY,&mut rec){
       
        return (rec.normal+Vec3::new(1.0, 1.0, 1.0))*0.5;
    }
    else
    {let unit_direction=r.direction().unit();
    let t = 0.5*(unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t}
    
}
fn write_color(pixel_color:Vec3) {
    let ir =( 255.999 * pixel_color.x) as u32;
    let ig =( 255.999 * pixel_color.y) as u32;
    let ib =( 255.999 * pixel_color.z) as u32;
    print!("{} {} {}\n", ir, ig, ib);
}
pub fn hit_sphere(center:Vec3,radius:f64,r:&Ray) -> f64 {
    let oc:Vec3 = r.origin() - center.clone();
    let a = r.direction().squared_length();
    let half_b =  oc.clone()*r.direction();
    let c = oc.clone().squared_length() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return -1.0;
    }else{
       return (-half_b - discriminant.sqrt())  / a;
    }
}