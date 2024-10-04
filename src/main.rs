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
use std::{thread, env};
use std::time::{Duration};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop,ActiveEventLoop};
use winit::keyboard::KeyCode;
use winit::window::{WindowId,Window};
use winit::application::ApplicationHandler;



fn main() -> Result<(), Error> {
    let mut worldspace: WorldSpace = WorldSpace::new();
    let mut camera = Camera::new();
    camera.update_extrinsics_centre(Point::new(0.0,0.0,0.0,1.0));
    //camera.rotate(na::Matrix3::new(0.707107, 0.0, 0.707107,
    //    0.0, 1.0, 0.0,
    //    -0.707107, 0.0, 0.707107));

    //camera.rotate(na::Matrix3::new(0.0, 0.0, 1.0,
    //  0.0, 1.0, 0.0,
    //  1.0, 0.0, 0.0)); 
    camera.rotate_degrees_y(90.0);
    worldspace.register_object(coordinate_object::Camera_object(camera));
    //worldspace.register_object(coordinate_object::Point_object(Point::new(-20.0,0.0,0.5,1.0)));
    worldspace.register_object(coordinate_object::Point_object(Point::new(-100.0,40.0,200.0,1.0)));
    worldspace.register_object(coordinate_object::Point_object(Point::new(0.0,-0.7071072,0.707107,1.0)));
    let mut visible_objects: Vec<&coordinate_object>;
    worldspace.update_cameras();
    for mut camera in worldspace.reference_to_cameras(){
        visible_objects = worldspace.get_visible_objects(camera);
        print!("{:?}",visible_objects);
        let string: Vec<char> = vec![];
        for (i,pixel) in worldspace.get_screen_values(camera).into_iter().enumerate(){
            if i % 128 == 0{
                print!("\n");
            }
            if pixel == [0x5e, 0x48, 0xe8, 0xff]{
                print!("#");
            }else{
                print!(".");
            }
        }
    }
    //print!("Above /\\");
    let pixels = PixelsApplication::new()?;

    thread::sleep(Duration::new(15,0));

    
    Ok(())
}

struct WorldSpace {
    all_independents: Vec<coordinate_object>,
    cameras: Vec<Camera>,
}
impl WorldSpace{
    fn new() -> Self{
        WorldSpace{
            all_independents: vec![],
            cameras: vec![],
        }
    }
    fn register_object(&mut self, object: coordinate_object){
        match object{
            Camera_object(camera) => self.cameras.push(camera),
            _ => self.all_independents.push(object),
        };
    }
    fn reference_to_cameras(&self) -> Vec<&Camera>{
        let mut vec: Vec<&Camera> = vec![];
        for camera in self.cameras.iter(){
            vec.push(camera);
        }
        vec
    }
    fn update_cameras(&mut self){
        for camera in self.cameras.iter_mut(){
            camera.update_basis_change_matrix();
        }
    }

    //With the cameras Extrinsics matrix, we can use the inverse to effectively translate to a new
    //coordinate system around the camera, allowing for easier and clearer logic.
    fn get_visible_objects<'a>(& 'a self, camera: & 'a Camera) -> Vec<&coordinate_object>{
        camera.return_visible_objects(&self.all_independents)
    }
    fn get_screen_values(&self, camera: & Camera) -> Vec<[u8;4]>{
    //We want to order these as local points by their depth (greatest to smallest and apply all in
    //that order).
    camera.get_screen_values(&self.all_independents)
    }
}
struct PixelsApplication{
    pixels: Pixels,
    event_loop: EventLoop<()>,
    window: Window,
}
impl PixelsApplication {
    pub fn new() -> Result<Self, Error>{
        env_logger::init();
        let mut event_loop = EventLoop::new().unwrap();
        let default_size = LogicalSize::new(128.0,128.0);
        let mut window_grabbed = event_loop.create_window(Window::default_attributes()); 
        let mut window = match window_grabbed {
            Ok(window) => window,
            _ => panic!(),
        };

        //window.set_maximized(true);
        match window.current_monitor() {
            Some(monitor) => window.set_min_inner_size(Some(monitor.size())),
             _ => (),
        };
        let mut new_pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height , surface_texture)?
        };

        Ok(PixelsApplication{
            pixels: new_pixels,
            event_loop: event_loop,
            window: window,
            })
        }
}
impl ApplicationHandler for PixelsApplication{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = event_loop.create_window(Window::default_attributes()).unwrap();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.request_redraw();
            }
            _ => (),
        }
    
    } 
}
