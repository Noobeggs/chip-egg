use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Error, Pixels, SurfaceTexture};

use crate::Chip8;
use crate::Options;
use crate::CPU_CLOCK;

use std::time::{Duration, Instant};

pub struct Chip8Window {
    pixels: Pixels,
    chip8: Chip8,
}

impl Chip8Window {
    pub fn new(pixels: Pixels, chip8: Chip8) -> Chip8Window {
        Chip8Window {
            pixels,
            chip8,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.pixels.resize_surface(new_size.width, new_size.height);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        // todo!();
    }

    pub fn render(&mut self) {
        let frame = self.pixels.get_frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let display = self.chip8.display();
            let x = (i % display.width() as usize);
            let y = (i / display.width() as usize);

            let rgba = if display.display()[x][y] == 0 {
                [0x00, 0x00, 0x00, 0xff]
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

pub async fn run(rom: Vec<u8>) -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    
    let options = Options::new();
    let chip8 = Chip8::new(options);
    let display = chip8.display();

    let width = display.width();
    let height = display.height();
    
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        let scaled_size = LogicalSize::new(width as f64 * 3.0, height as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Chip Egg")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width as u32, height as u32, surface_texture)?
    };

    let mut chip8_window = Chip8Window::new(pixels, chip8);
    chip8_window.chip8.load_rom(rom);
    let mut last_cpu_tick = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !chip8_window.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        chip8_window.resize(*physical_size);
                        chip8_window.pixels.render().expect("Error rendering window");
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                chip8_window.update();
                chip8_window.render();
            }
            Event::MainEventsCleared => {
                if last_cpu_tick.elapsed() >= Duration::from_micros(CPU_CLOCK) {
                    chip8_window.chip8.run_cpu_cycle();
                    if chip8_window.chip8.display().redraw() {
                        chip8_window.pixels.render().expect("Error rendering window");
                    }
                }
                last_cpu_tick = Instant::now();
                window.request_redraw();
            }
            _ => {}
        }
    });
}