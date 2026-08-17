use glium::{
    backend::glutin::Display,
    glutin::{
        self, dpi,
        event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::ControlFlow,
    },
    texture::{ClientFormat, RawImage2d},
    BlitTarget, Rect, Surface,
};
use std::{borrow::Cow, env, fs::File, io, path};

/// Load the image using `png`
fn load_image(path: &path::PathBuf) -> io::Result<RawImage2d<'static, u8>> {
    use png::ColorType::*;
    let mut decoder = png::Decoder::new(File::open(path)?);
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info()?;
    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut img_data)?;

    let (data, format) = match info.color_type {
        Rgb => (img_data, ClientFormat::U8U8U8),
        Rgba => (img_data, ClientFormat::U8U8U8U8),
        Grayscale => (
            {
                let mut vec = Vec::with_capacity(img_data.len() * 3);
                for g in img_data {
                    vec.extend([g, g, g].iter().cloned())
                }
                vec
            },
            ClientFormat::U8U8U8,
        ),
        GrayscaleAlpha => (
            {
                let mut vec = Vec::with_capacity(img_data.len() * 3);
                for ga in img_data.chunks(2) {
                    let g = ga[0];
                    let a = ga[1];
                    vec.extend([g, g, g, a].iter().cloned())
                }
                vec
            },
            ClientFormat::U8U8U8U8,
        ),
        _ => unreachable!("uncovered color type"),
    };

    Ok(RawImage2d {
        data: Cow::Owned(data),
        width: info.width,
        height: info.height,
        format,
    })
}

fn main_loop(files: Vec<path::PathBuf>) -> io::Result<()> {
    let mut files = files.into_iter();
    let image = load_image(&files.next().unwrap())?;

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("Show Example");
    let context_builder = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    resize_window(&display, &image);
    let mut texture = glium::Texture2d::new(&display, image).unwrap();
    draw(&display, &texture);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => exit(control_flow),
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: code,
                            ..
                        },
                    ..
                },
            ..
        } => match code {
            Some(VirtualKeyCode::Escape) => exit(control_flow),
            Some(VirtualKeyCode::Right) => match &files.next() {
                Some(path) => {
                    match load_image(path) {
                        Ok(image) => {
                            resize_window(&display, &image);
                            texture = glium::Texture2d::new(&display, image).unwrap();
                            draw(&display, &texture);
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                            exit(control_flow);
                        }
                    };
                }
                None => exit(control_flow),
            },
            _ => {}
        },
        Event::RedrawRequested(_) => draw(&display, &texture),
        _ => {}
    });
}

fn draw(display: &glium::Display, texture: &glium::Texture2d) {
    let frame = display.draw();
    fill_v_flipped(
        &texture.as_surface(),
        &frame,
        glium::uniforms::MagnifySamplerFilter::Linear,
    );
    frame.finish().unwrap();
}

fn exit(control_flow: &mut ControlFlow) {
    *control_flow = ControlFlow::Exit;
}

fn fill_v_flipped<S1, S2>(src: &S1, target: &S2, filter: glium::uniforms::MagnifySamplerFilter)
where
    S1: Surface,
    S2: Surface,
{
    let src_dim = src.get_dimensions();
    let src_rect = Rect {
        left: 0,
        bottom: 0,
        width: src_dim.0,
        height: src_dim.1,
    };
    let target_dim = target.get_dimensions();
    let target_rect = BlitTarget {
        left: 0,
        bottom: target_dim.1,
        width: target_dim.0 as i32,
        height: -(target_dim.1 as i32),
    };
    src.blit_color(&src_rect, target, &target_rect, filter);
}

fn resize_window(display: &Display, image: &RawImage2d<'static, u8>) {
    let mut width = image.width;
    let mut height = image.height;
    if width < 50 && height < 50 {
        width *= 10;
        height *= 10;
    }
    display
        .gl_window()
        .window()
        .set_inner_size(dpi::LogicalSize::new(f64::from(width), f64::from(height)));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: show files [...]");
    } else {
        let mut files = vec![];
        for file in args.iter().skip(1) {
            match if file.contains('*') {
                (|| -> io::Result<_> {
                    for entry in glob::glob(file)
                        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.msg))?
                    {
                        files.push(
                            entry
                                .map_err(|_| io::Error::new(io::ErrorKind::Other, "glob error"))?,
                        )
                    }
                    Ok(())
                })()
            } else {
                files.push(path::PathBuf::from(file));
                Ok(())
            } {
                Ok(_) => (),
                Err(err) => {
                    println!("{}: {}", file, err);
                    break;
                }
            }
        }
        // "tests/pngsuite/pngsuite.png"
        match main_loop(files) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
    }
}
