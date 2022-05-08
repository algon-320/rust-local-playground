//# crossfont = "0.3.2"
//# glium = "0.31.0"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use crossfont::ft::FreeTypeRasterizer;
use crossfont::{BitmapBuffer, FontDesc, GlyphKey, Rasterize, Size, Slant, Style, Weight};
use glium::*;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use std::borrow::Cow;

fn main() -> std::io::Result<()> {
    let font_desc = FontDesc::new(
        "Monospace",
        Style::Description {
            slant: Slant::Normal,
            weight: Weight::Normal,
        },
    );

    let mut freetype = FreeTypeRasterizer::new(1.0, false).expect("new freetype");
    let size = Size::new(32.0);
    let font = freetype.load_font(&font_desc, size).expect("load font");
    let _font_metrics = freetype.metrics(font, size).expect("metrics");

    let window = WindowBuilder::new()
        .with_inner_size(glutin::dpi::PhysicalSize::new(1024, 1024))
        .with_title("font rendering test");
    let context = ContextBuilder::new().with_vsync(true);
    let event_loop = EventLoop::new();
    let display = Display::new(window, context, &event_loop).expect("display new");

    let width = 512 * 2;
    let height = 512;
    let base_image = texture::RawImage2d {
        data: Cow::Owned(vec![0_u8; 3 * width * height]),
        width: width as u32,
        height: height as u32,
        format: texture::ClientFormat::U8U8U8,
    };
    let texture = texture::Texture2d::with_format(
        &display,
        base_image,
        texture::UncompressedFloatFormat::U8U8U8,
        texture::MipmapsOption::NoMipmap,
    )
    .expect("Failed to create texture");

    let vshader = r#"
#version 140
in  vec2 position;
in  vec2 tex_coords;
out vec2 v_tex_coords;

void main() {
    gl_Position = vec4(position, 0, 1);
    v_tex_coords = tex_coords;
}
"#;
    let fshader = r#"
#version 140
uniform sampler2D tex;
in  vec2 v_tex_coords;

void main() {
    vec3 rgb = vec3(1, 1, 1) - texture(tex, v_tex_coords).rgb;
    gl_FragColor = vec4(rgb, 1);
}
"#;

    let program = Program::from_source(&display, vshader, fshader, None).unwrap();

    let message = r#"Hello, World!"#;

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        },

        Event::RedrawRequested(_) => {
            let (window_width, window_height): (u32, u32) =
                display.gl_window().window().inner_size().into();

            let mut cwid = 0;
            let mut chei = 0;
            for ch in message.chars() {
                if ch == '\n' {
                    continue;
                }

                let glyph = freetype
                    .get_glyph(GlyphKey {
                        character: ch,
                        font_key: font,
                        size,
                    })
                    .expect("rasterize");

                if cwid < glyph.width {
                    cwid = glyph.width + glyph.left;
                }
                if chei < glyph.height {
                    chei = glyph.height;
                }
            }

            let mut vertices: Vec<Vertex> = Vec::new();

            let mut x: u32 = 0;
            let mut baseline: u32 = chei as u32;

            for ch in message.chars() {
                if ch == '\n' {
                    baseline += chei as u32;
                    x = 0;
                    continue;
                }

                let glyph = freetype
                    .get_glyph(GlyphKey {
                        character: ch,
                        font_key: font,
                        size,
                    })
                    .expect("rasterize");

                match glyph.buffer {
                    BitmapBuffer::Rgb(bytes) => {
                        let uv_rect = glium::Rect {
                            left: x + glyph.left as u32,
                            bottom: baseline - glyph.top as u32,
                            width: glyph.width as u32,
                            height: glyph.height as u32,
                        };

                        texture.main_level().write(
                            uv_rect,
                            texture::RawImage2d {
                                data: Cow::Borrowed(&bytes),
                                width: uv_rect.width,
                                height: uv_rect.height,
                                format: texture::ClientFormat::U8U8U8,
                            },
                        );

                        let gl_x = (uv_rect.left as f32 / window_width as f32) * 2.0 - 1.0;
                        let gl_y = -(uv_rect.bottom as f32 / window_height as f32) * 2.0 + 1.0;
                        let gl_w = (uv_rect.width as f32 / window_width as f32) * 2.0;
                        let gl_h = (uv_rect.height as f32 / window_height as f32) * 2.0;
                        let tx_x = uv_rect.left as f32 / width as f32;
                        let tx_y = uv_rect.bottom as f32 / height as f32;
                        let tx_w = uv_rect.width as f32 / width as f32;
                        let tx_h = uv_rect.height as f32 / height as f32;

                        vertices.extend_from_slice(&[
                            Vertex {
                                position: [gl_x, gl_y],
                                tex_coords: [tx_x, tx_y],
                            },
                            Vertex {
                                position: [gl_x, gl_y - gl_h],
                                tex_coords: [tx_x, tx_y + tx_h],
                            },
                            Vertex {
                                position: [gl_x + gl_w, gl_y - gl_h],
                                tex_coords: [tx_x + tx_w, tx_y + tx_h],
                            },
                            Vertex {
                                position: [gl_x + gl_w, gl_y - gl_h],
                                tex_coords: [tx_x + tx_w, tx_y + tx_h],
                            },
                            Vertex {
                                position: [gl_x + gl_w, gl_y],
                                tex_coords: [tx_x + tx_w, tx_y],
                            },
                            Vertex {
                                position: [gl_x, gl_y],
                                tex_coords: [tx_x, tx_y],
                            },
                        ]);
                    }
                    BitmapBuffer::Rgba(bytes) => {
                        println!("Rgba Buffer: {} bytes", bytes.len());
                        todo!();
                    }
                }

                x += cwid as u32;
            }

            use index::{NoIndices, PrimitiveType};
            use uniforms::MagnifySamplerFilter;

            let uniforms = uniform! {
                tex: texture.sampled().magnify_filter(MagnifySamplerFilter::Linear),
            };

            let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();

            let mut surface = display.draw();
            surface.clear_color(1.0, 1.0, 1.0, 1.0);
            surface
                .draw(
                    &vertex_buffer,
                    NoIndices(PrimitiveType::TrianglesList),
                    &program,
                    &uniforms,
                    &DrawParameters::default(),
                )
                .expect("draw");

            surface.finish().expect("finish");

            display.gl_window().window().request_redraw();
        }

        _ => {}
    });
}
