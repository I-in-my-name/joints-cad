use nalgebra;
use libs::core::*;
use libs::pixels_lib::*;
mod libs{
    pub mod core;
    pub mod pixels_lib;
}

fn main() {
    println!("Hello, world!");
    let v: Point;
    libs::pixels_lib::game();
}

struct WorldSpace {
    all_independents: Vec<coordinate_object>
}
impl WorldSpace{
    fn new() -> Self{
        WorldSpace{
            all_independents: vec![
                coordinate_object::Line_object(Line::new(Point::new(100.0,0.0,0.0,1.0),Point::new(-100.0,0.0,0.0,1.0))),

                coordinate_object::Line_object(Line::new(Point::new(0.0,100.0,0.0,1.0),Point::new(0.0,-100.0,0.0,1.0))),
                
                coordinate_object::Line_object(Line::new(Point::new(0.0,0.0,100.0,1.0),Point::new(0.0,0.0,-100.0,1.0)))
            ],
        }
    }
    fn register_object()
    fn camera_visible_objects(camera: Camera) -> Vec<coordinate_object>{
        for object in all_independents.iter(){
            for point in object.getPoints().iter(){
                print!(Point{0.0,0.0,0.0,1.0});
                print!(camera);
            }
        }

    }
}
