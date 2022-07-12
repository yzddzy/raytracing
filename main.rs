
mod vec3;
pub use vec3::Vec3;
mod ray;
pub use ray::Ray;
type Point3=Vec3;
type Color=Vec3;
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
                pixel_color = ray_color(&r);
            
            write_color(pixel_color);
        }
    }
}
pub fn ray_color(r:&Ray)->Vec3{
    let unit_direction= r.direction().unit();
    let t = (unit_direction.y + 1.0)*0.5;
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
    
}
fn write_color(pixel_color:Vec3) {
    let ir =( 255.999 * pixel_color.x) as u32;
    let ig =( 255.999 * pixel_color.y) as u32;
    let ib =( 255.999 * pixel_color.z) as u32;
    print!("{} {} {}\n", ir, ig, ib);
}