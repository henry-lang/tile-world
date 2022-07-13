mod camera;
mod input;
mod shaders;
mod tile;
mod world;

use glium::{
    glutin::{
        self,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex, uniform,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior},
    Display, DrawParameters, Surface,
};

use glam::{Mat4, Vec2};

use simple_logger::SimpleLogger;
use std::rc::Rc;
use std::time::Instant;
use std::{path::PathBuf, time::Duration};

use camera::Camera;
use input::Input;
use shaders::Shaders;
use tile::TileTypes;

struct Resources {
    tile_types: TileTypes,
}

fn create_display(event_loop: &EventLoop<()>) -> Display {
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new();

    Display::new(wb, cb, event_loop).expect("create display")
}

fn main() {
    SimpleLogger::new()
        .with_module_level("winit", log::LevelFilter::Info)
        .init()
        .unwrap();

    let event_loop = glutin::event_loop::EventLoop::new();

    let display = Rc::new(create_display(&event_loop));

    let shaders = Shaders::create_all(&display);

    let tile_types = TileTypes::load(["assets", "tile_types.ron"].iter().collect::<PathBuf>());
    log::info!("Found {} tile types from tile_types.ron", tile_types.len());

    let mut tile_textures = tile_types.build_texture(&display);

    let mut camera = Camera::new(display.clone(), Vec2::splat(0.5), 0., 100.0..1000.);
    let mut input = Input::new();

    #[derive(Copy, Clone)]
    struct Vertex {
        tile_id: u16,
    }

    implement_vertex!(Vertex, tile_id);

    fn create_tile(x: f32, y: f32, vec: &mut Vec<Vertex>) {
        if x == 0. && y == 0. {
            vec.push(Vertex { tile_id: 0 });
        } else {
            vec.push(Vertex { tile_id: 1 });
        }
    }

    let mut shape = vec![];

    for x in 0..10 {
        for y in 0..10 {
            create_tile(x as f32, y as f32, &mut shape);
        }
    }

    let vertex_buffer = glium::VertexBuffer::new(display.as_ref(), &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let display_clone = display.clone();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        input.update(&event);
        camera.update(&input);

        if input.is_key_pressed(glutin::event::VirtualKeyCode::R) {
            tile_textures = tile_types.build_texture(&display);
        }

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let behavior = SamplerBehavior {
                    minify_filter: MinifySamplerFilter::Nearest,
                    magnify_filter: MagnifySamplerFilter::Nearest,
                    ..Default::default()
                };

                let uniforms = uniform! {
                    projection: camera.projection_matrix().to_cols_array_2d(),
                    tiles: Sampler(&tile_textures, behavior)
                };

                let mut target = display_clone.draw();
                target.clear_color(1., 1., 1., 1.);

                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &shaders.tiles(),
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();

                target.finish().unwrap();
            }
            _ => (),
        }
    });
}
