use nalgebra;
mod libs{
    pub mod core;
    pub mod pixels_lib;
}

fn main() {
    println!("Hello, world!");
    let v: libs::core::Point;
    libs::pixels_lib::game();
}

struct WorldSpace {
    all_independents: Vec<libs::core::coordinate_objects>
}
impl WorldSpace{
    fn new() -> Self{
        WorldSpace{
            all_independents: vec![libs::core::coordinate_objects::Point_object(libs::core::Point::new(1.0,2.0,3.0,1.0))],
        }
    }
}
