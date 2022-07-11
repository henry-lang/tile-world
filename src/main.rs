mod camera;
mod input;
mod shaders;
mod tile;
mod world;

use glium::{
    glutin::{
        self,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    index::PrimitiveType,
    uniform, Display, Surface,
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

    tile_types.build_texture(&display);

    let mut camera = Camera::new(display.clone(), Vec2::splat(0.5), 0., 100.0..1000.);
    let mut input = Input::new();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(display.as_ref(), &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let display_clone = display.clone();

    event_loop.run(move |event, _, control_flow| {
        //*control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(1000 / 60));

        input.update(&event);
        camera.update(&input);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        }
        let uniforms = uniform! {
            projection: camera.projection_matrix().to_cols_array_2d()
        };

        let mut target = display_clone.draw();
        target.clear_color(1., 1., 1., 1.);

        target
            .draw(
                &vertex_buffer,
                &indices,
                &shaders.default(),
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        target.finish().unwrap();
    });
}
