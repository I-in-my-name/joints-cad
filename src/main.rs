use nalgebra;
use libs::core::*;
use libs::pixels_lib::*;

use libs::core::coordinate_object::{Camera_object,Line_object};
mod libs{
    pub mod core;
    pub mod pixels_lib;
}

fn main() {
    libs::pixels_lib::game();
    let mut worldspace = WorldSpace::new();
    let mut camera = Camera::new();

    worldspace.register_object(coordinate_object::Camera_object(camera));
    for camera in worldspace.cameras.iter(){
        let visible_objects: Vec<coordinate_object> = worldspace.get_visible_objects(camera);
    }
}

struct WorldSpace {
    all_independents: Vec<coordinate_object>,
    cameras: Vec<Camera>,
}
impl WorldSpace{
    fn new() -> Self{
        WorldSpace{
            all_independents: vec![
                Line_object(Line::new(Point::new(100.0,0.0,0.0,1.0),Point::new(-100.0,0.0,0.0,1.0))),

                Line_object(Line::new(Point::new(0.0,100.0,0.0,1.0),Point::new(0.0,-100.0,0.0,1.0))),
                
                Line_object(Line::new(Point::new(0.0,0.0,100.0,1.0),Point::new(0.0,0.0,-100.0,1.0)))
            ],
            cameras: vec![],
        }
    }
    fn register_object(&mut self, object: coordinate_object){
        match object{
            Camera_object(camera) => self.cameras.push(camera),
            _ => self.all_independents.push(object),
        };
    }

    //With the cameras Extrinsics matrix, we can use the inverse to effectively translate to a new
    //coordinate system around the camera, allowing for easier and clearer logic.
    fn get_visible_objects(&self, camera: &Camera) -> Vec<coordinate_object>{
        camera.return_visible_objects(self.all_independents.clone())
    }
}

