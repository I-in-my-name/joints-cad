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
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::*;
use winit::event_loop::{EventLoop,ActiveEventLoop,ControlFlow};
use winit::keyboard::KeyCode;
use winit::event::DeviceEvent::*;
use winit::event::WindowEvent::*;
use winit::window::{WindowId,Window};
use winit::application::ApplicationHandler;



fn main() -> Result<(), Error> {
    let mut pixels = PixelsApplication::new()?;
    
    pixels.run_app();
    thread::sleep(Duration::new(3,0));
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
    fn setup(&mut self) {
        let mut camera = Camera::new();
        camera.update_extrinsics_centre(Point::new(0.0,0.0,0.0,1.0));
        //camera.rotate(na::Matrix3::new(0.707107, 0.0, 0.707107,
        //    0.0, 1.0, 0.0,
        //    -0.707107, 0.0, 0.707107));
    
        //camera.rotate(na::Matrix3::new(0.0, 0.0, 1.0,
        //  0.0, 1.0, 0.0,
        //  1.0, 0.0, 0.0)); 
        //camera.rotate_degrees_y(90.0);
        self.register_object(coordinate_object::Camera_object(camera));
        //worldspace.register_object(coordinate_object::Point_object(Point::new(-20.0,0.0,0.5,1.0)));
        self.register_object(coordinate_object::Point_object(Point::new(-100.0,40.0,200.0,1.0)));
        //self.register_object(coordinate_object::Point_object(Point::new(-10.0,0.0,0.0,1.0)));
        self.register_object(coordinate_object::Point_object(Point::new(0.0,-0.7071072,0.707107,1.0)));
        let mut visible_objects: Vec<&coordinate_object>;
        self.update_cameras();
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
            camera.update_camera();

        }
    }
    fn get_new_pixels(&self, pixels: &mut Pixels,  size: PhysicalSize<u32>){
        //print!("\n\n\n\n\n\n\n\n\n\ncams: {:?}\n\n\n\n\n\n\n\n\n\n\n",self.reference_to_cameras().pop().unwrap().orientation);
        for mut camera in self.reference_to_cameras(){
            let mut colour;
            colour = self.get_screen_values(camera);
            for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate(){
                pixel.copy_from_slice(&colour[i]);
            }
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
    fn update_size(&mut self, size:PhysicalSize<u32>){
        self.cameras[0].update_screen_size(size.width as i32, size.height as i32);
    }
}
struct PixelsApplication{
    event_loop: EventLoop<()>,
    subhandler: Option<Subhandler>,
}
impl PixelsApplication{
    pub fn new() -> Result<Self, Error>{
        env_logger::init();
        let mut event_loop = EventLoop::new().unwrap();
        let default_size = LogicalSize::new(128.0,128.0);
        let mut window_grabbed = event_loop.create_window(Window::default_attributes().with_title("Grand CAD Environment").with_decorations(true).with_visible(true)); 
        let mut window = match window_grabbed {
            Ok(window) => window,
            _ => panic!(),
        };
        let mut new_pixels = {
            let window_size = window.inner_size();
            print!("\n_size: {:?}\n",window_size);
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height , surface_texture)?
        };
        event_loop.set_control_flow(ControlFlow::Wait);
        
        let mut pixapp = PixelsApplication{
            event_loop: event_loop,
            subhandler: None,
        };
        pixapp.set_handler(Some(Subhandler::new(window,new_pixels,)));
        Ok(pixapp)
    }
    pub fn draw_to_window(&mut self){
        self.subhandler.as_mut().unwrap().redraw();
    }
    pub fn set_handler(&mut self, new_handler: Option<Subhandler>){
        self.subhandler = new_handler;
    }
    pub fn run_app(self){
        self.event_loop.run_app(&mut self.subhandler.unwrap()); 
    }
}
struct Subhandler{
    pixels: Pixels,
    window: Window,
    worldspace: WorldSpace,

    right_mouse_button: bool,
}
impl Subhandler{
    pub fn new(window: Window, pixels: Pixels) -> Self{ 
        Subhandler {
            pixels: pixels,
            window: window,
            worldspace: WorldSpace::new(),
        }
    }
    pub fn redraw(&self){
        self.window.request_redraw();
    }


}
impl ApplicationHandler for Subhandler{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        print!("resumed/started app");
        //self.window = event_loop.create_window(Window::default_attributes()).unwrap();
        self.worldspace.setup();
        self.worldspace.update_size(self.window.inner_size());
        self.worldspace.get_new_pixels(&mut self.pixels,self.window.inner_size());
        self.pixels.render();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                print!("finished");
            },
           WindowEvent::RedrawRequested => {
                print!("REQ");
                self.worldspace.get_new_pixels(&mut self.pixels,self.window.inner_size());
                self.pixels.render();
            },
            WindowEvent::CursorMoved{
                device_id,
                position,
            } => {
                self.worldspace.cameras[0].rotate_degrees_y(1.0);
                self.worldspace.cameras[0].update_camera();


                thread::sleep(Duration::new(0,10));

                self.redraw();
            },
            WindowEvent::MouseInput{
                device_id: DeviceId,
                state: ElementState,
                button: MouseButton,} => {
                    match button {
                        Right => match state{ 
                            Pressed => self.right_mouse_button = true, 
                            Released => self.right_mouse_button = false,
                            },
                        _ => {},
                    };
                },
            WindowEvent::MouseWheel {
                device_id: id,
                delta: delta,
                phase: phase,
            } => {
                print!("{:?}",delta);
            },
            _ =>{},
        }
    
    } 
    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ){
        match event{
            DeviceEvent::Key(rawEvent) => {
                match rawEvent{
                    RawKeyEvent{physical_key, state} => {
                        print!("key:   {:?}",physical_key);
                    },
                    _ => {print!("YYY");}, 
                };
            },
            DeviceEvent::MouseMotion {
                delta: (a, b),
            } => {
                //print!("MOUSE");
            },

            DeviceEvent::MouseWheel {
                delta: delta,
            } => {
                print!("SCROLL");
            }
            _ => {},
        };
    }
}

