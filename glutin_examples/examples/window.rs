#[macro_use]
extern crate glium;

mod support;

use ::glutin::dpi::LogicalSize;
#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::{index::PrimitiveType, Program};
use glium_glyph::{
    glyph_brush::{ab_glyph::FontArc, Layout, Section, Text, VerticalAlign},
    GlyphBrush,
};

use std::{
    borrow::BorrowMut,
    time::{Duration, Instant},
};
// The `TextSystem` contains the shaders and elements used for text display.

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(LogicalSize::new(800.0, 600.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let dejavu = FontArc::try_from_slice(include_bytes!("../../f.ttf")).unwrap();
    let fonts = vec![dejavu];

    let mut glyph_brush = GlyphBrush::new(&display, fonts);

    println!("hello.");
    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 3],
    }

    let data = [
        Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
        Vertex { position: [0.0, 0.5], color: [0.0, 0.0, 1.0] },
        Vertex { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
    ];

    implement_vertex!(Vertex, position, color);
    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = { glium::VertexBuffer::new(&display, &data).unwrap() };
    // building the index buffer
    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();
    let program =
        Program::from_source(&display, include_str!("vert.vs"), include_str!("frag.fs"), None)
            .unwrap();

    let mut draw = move || {
        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target
            .draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default())
            .unwrap();
        {
            let s = data.iter().map(|v| (format!("{:?}\n", v.position), v)).collect::<Vec<_>>();
            println!("{:?}", s);
            for (s, v) in s.iter() {
                let section = Section::default()
                    .add_text(Text::new(s))
                    .with_screen_position((v.position[0], v.position[1]));
                glyph_brush.queue(section);
            }
            glyph_brush.draw_queued(&display, &mut target);
        }
        target.finish().unwrap();
    };
    draw();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    draw();
                    glutin::event_loop::ControlFlow::Poll
                }
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}
