use nalgebra as na;
use libs::core::*;
use libs::pixels_lib::*;
use libs::display_utils::*;

use libs::core::coordinate_object::{Camera_object,Line_object};
mod libs{
    pub mod core;
    pub mod pixels_lib;
    pub mod display_utils;
}

//Crates for pixels and the display
#[deny(clippy::all)]
#[forbid(unsafe_code)]
use std::{thread, time};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::{WindowBuilder};

fn main() -> Result<(), Error> {
    let mut worldspace = WorldSpace::new();
    let mut camera = Camera::new();
    camera.update_extrinsics_centre(Point::new(1.0,0.0,-1.0,1.0));
    camera.rotate(na::Matrix3::new(0.707107, 0.0, 0.707107,
          0.0, 1.0, 0.0,
          -0.707107, 0.0, 0.707107));
    worldspace.register_object(coordinate_object::Camera_object(camera));
    worldspace.register_object(coordinate_object::Point_object(Point::new(-0.5,0.0,0.5,1.0)));
    worldspace.register_object(coordinate_object::Point_object(Point::new(0.0,-0.7071072,0.707107,1.0)));
    let mut visible_objects: Vec<coordinate_object>;
    for camera in worldspace.cameras.iter(){
        visible_objects = worldspace.get_visible_objects(camera);
   //   print!("below \\/");
        print!("{:?}",visible_objects);
    }

    //print!("Above /\\");
    let pixels = PixelsStruct::new()?;
    //pixels.game()
    Ok(())
}

struct WorldSpace {
    all_independents: Vec<coordinate_object>,
    cameras: Vec<Camera>,
}
impl WorldSpace{
    fn new() -> Self{
        WorldSpace{
            all_independents: vec![
      //          Line_object(Line::new(Point::new(100.0,0.0,0.0,1.0),Point::new(-100.0,0.0,0.0,1.0))),

        //        Line_object(Line::new(Point::new(0.0,100.0,0.0,1.0),Point::new(0.0,-100.0,0.0,1.0))),
                
          //      Line_object(Line::new(Point::new(0.0,0.0,100.0,1.0),Point::new(0.0,0.0,-100.0,1.0)))
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
        camera.return_visible_objects(&self.all_independents.clone())
    }
    fn get_screen_values(&self, camera: &Camera) -> Vec<u8>{
    //We want to order these as local points by their depth (greatest to smallest and apply all in
    //that order).
    camera.get_screen_values(&self.all_independents)
    }
}
struct PixelsStruct{
    pixels: Pixels,
    event_loop: EventLoop<()>,
}
impl PixelsStruct {
    pub fn new() -> Result<Self, Error>{
    env_logger::init();
        let new_event_loop = EventLoop::new();
        let default_size = LogicalSize::new(128.0,128.0);
        let window = {
            WindowBuilder::new()
                .with_title("The Grand CAD Environment")
                .with_inner_size(default_size)
                .with_min_inner_size(default_size)
                .build(&new_event_loop)
                .unwrap()
        };
        window.set_decorations(true);
        //window.set_maximized(true);
        match window.current_monitor() {
            Some(monitor) => window.set_inner_size(monitor.size()),
             _ => (),
        };
        let mut new_pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height , surface_texture)?
        };
        Ok(PixelsStruct{
            pixels: new_pixels,
            event_loop: new_event_loop,
            })
        }
    pub fn game(self) -> Result<(), Error> {
            
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
        
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}

