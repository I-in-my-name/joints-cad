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
    fn register_object(){}

    //With the cameras Extrinsics matrix, we can use the inverse to effectively translate to a new
    //coordinate system around the camera, allowing for easier and clearer logic.
    fn get_visible_objects(&self, camera: Camera) -> Vec<coordinate_object>{
        camera.return_visible_objects(self.all_independents.clone())
    }
}

