extern crate rand;
use rand::Rng;
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
use std::rc::Rc;
mod camera;
pub use camera::Camera;
mod material;
pub use material ::*;
pub fn random_double() -> f64 {
    rand::random::<f64>()
}
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}
fn main(){
    const aspect_ratio:f64 = 3.0 / 2.0;
    const image_width:u32 = 1200;
    const image_height:u32 =(image_width as f64/ aspect_ratio)as u32;
    const samples_per_pixel:u32 = 500;
    const max_depth:u32 = 50;
    let lookfrom=Vec3::new(13.0,2.0,3.0);
    let lookat=Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);
    let mut world=random_scene();
    println!("P3");
    println!{"{} {}",image_width, image_height };
    println!{"255"};
    for j in (0..image_height).rev(){
        for i in (0..image_width){
            let mut pixel_color = Vec3::zero();
            for s in(0..samples_per_pixel){
            let u = (i as f64 + random_double()) / (image_width as f64);
            let v = (j as f64 + random_double()) / (image_height as f64);
            let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r,&world,max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
}
pub fn ray_color(r:&Ray,world:&hittable::HittableList,depth:u32)->Vec3{
    let mut rec = hit_record::new(Rc::new(Lambertian::new(Vec3::new(0.0,0.0,0.0))));
    if depth<=0{
        return Vec3::new(0.0,0.0,0.0);
    }
    if world.hit((*r).clone(),0.00001,INFINITY,&mut rec){
        let mut scattered = Ray::new(Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,0.0,0.0));
        let mut attenuation = Vec3::new(0.0,0.0,0.0);
        if rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered){
            return Vec3::elemul(attenuation, ray_color(&scattered, world, depth - 1));
        }
        return Vec3::new(0.0,0.0,0.0);
    }
    else
    {let unit_direction=r.direction().unit();
    let t = 0.5*(unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t}
    
}
pub fn write_color(pixel_color:Vec3,samples_per_pixel: u32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale*r).sqrt();
    g = (scale*g).sqrt();
    b = (scale*b).sqrt();
    let ir =( (256 as f64) * clamp(r, 0.0, 0.999)) as u32;
    let ig =( (256 as f64)* clamp(g, 0.0, 0.999)) as u32;
    let ib =( (256 as f64)* clamp(b, 0.0, 0.999)) as u32;
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
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
fn random_scene()->hittable::HittableList{
    let mut world = hittable::HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(sphere::Sphere::new(Vec3::new(0.0, -1000.0, 0.0),1000.0,material_ground.clone())));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center=Vec3::new((a as f64)+0.9*random_double(),0.2,(b as f64) + 0.9*random_double());

            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if (choose_mat < 0.8) {
                    let albedo =Vec3::elemul(Vec3::random(),Vec3::random());
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(sphere::Sphere::new(center.clone(),0.2,sphere_material.clone())));
                } else if (choose_mat < 0.95) {
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(sphere::Sphere::new(center.clone(),0.2,sphere_material.clone())));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(sphere::Sphere::new(center.clone(),0.2,sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(sphere::Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1.clone())));

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(sphere::Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2.clone())));

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(sphere::Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3.clone())));

    world
}
