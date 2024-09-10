#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::{thread, time};

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::{WindowBuilder};

pub fn game() -> Result<(), Error> {
     env_logger::init();
    let event_loop = EventLoop::new();
    let default_size = LogicalSize::new(128.0,128.0);
    let window = {
        WindowBuilder::new()
            .with_title("The Grand CAD Environment")
            .with_inner_size(default_size)
            .with_min_inner_size(default_size)
            .build(&event_loop)
            .unwrap()
    };
    window.set_decorations(true);
    //window.set_maximized(true);
    match window.current_monitor() {
        Some(monitor) => window.set_inner_size(monitor.size()),
         _ => (),
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height , surface_texture)?
    };
    
    event_loop.run(move |event, _, control_flow| {
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


