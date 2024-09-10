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

struct worldSpace {
    
}
