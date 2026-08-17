extern crate khronos_egl as egl;
use gl::types::{GLboolean, GLchar, GLenum, GLint, GLuint, GLvoid};
use std::ffi::CStr;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use wayland_client::{
	protocol::{wl_compositor::WlCompositor, wl_surface::WlSurface},
	DispatchData, Display, EventQueue, Main,
};

use wayland_protocols::xdg_shell::client::{
	xdg_surface::{self, XdgSurface},
	xdg_wm_base::{self, XdgWmBase},
};

fn process_xdg_event(xdg: Main<XdgWmBase>, event: xdg_wm_base::Event, _dd: DispatchData) {
	use xdg_wm_base::Event::*;

	match event {
		Ping { serial } => xdg.pong(serial),
		_ => (),
	}
}

struct DisplayConnection {
	display: Display,
	event_queue: EventQueue,
	compositor: Main<WlCompositor>,
	xdg: Main<XdgWmBase>,
}

fn setup_wayland() -> DisplayConnection {
	let display =
		wayland_client::Display::connect_to_env().expect("unable to connect to the wayland server");
	let mut event_queue = display.create_event_queue();
	let attached_display = display.clone().attach(event_queue.token());

	let globals = wayland_client::GlobalManager::new(&attached_display);

	// Roundtrip to retrieve the globals list
	event_queue
		.sync_roundtrip(&mut (), |_, _, _| unreachable!())
		.unwrap();

	// Get the compositor.
	let compositor: Main<WlCompositor> = globals.instantiate_exact(1).unwrap();

	// Xdg protocol.
	let xdg: Main<XdgWmBase> = globals.instantiate_exact(1).unwrap();
	xdg.quick_assign(process_xdg_event);

	DisplayConnection {
		display,
		event_queue,
		compositor,
		xdg,
	}
}

fn setup_egl(egl: &egl::DynamicInstance, display: &Display) -> egl::Display {
	let egl_display = unsafe {
		egl.get_display(display.get_display_ptr() as *mut std::ffi::c_void)
			.unwrap()
	};

	egl.initialize(egl_display).unwrap();
	egl_display
}

fn create_context(
	egl: &egl::DynamicInstance,
	display: egl::Display,
) -> (egl::Context, egl::Config) {
	let attributes = [
		egl::RED_SIZE,
		8,
		egl::GREEN_SIZE,
		8,
		egl::BLUE_SIZE,
		8,
		egl::NONE,
	];

	let config = egl
		.choose_first_config(display, &attributes)
		.expect("unable to choose an EGL configuration")
		.expect("no EGL configuration found");

	let context_attributes = [
		egl::CONTEXT_MAJOR_VERSION,
		4,
		egl::CONTEXT_MINOR_VERSION,
		0,
		egl::CONTEXT_OPENGL_PROFILE_MASK,
		egl::CONTEXT_OPENGL_CORE_PROFILE_BIT,
		egl::NONE,
	];

	let context = egl
		.create_context(display, config, None, &context_attributes)
		.expect("unable to create an EGL context");

	(context, config)
}

struct Surface {
	handle: Main<WlSurface>,
	initialized: AtomicBool,
}

fn create_surface(
	egl: &Arc<egl::DynamicInstance>,
	ctx: &DisplayConnection,
	egl_display: egl::Display,
	egl_context: egl::Context,
	egl_config: egl::Config,
	width: i32,
	height: i32,
) -> Arc<Surface> {
	let wl_surface = ctx.compositor.create_surface();
	let xdg_surface = ctx.xdg.get_xdg_surface(&wl_surface);

	let xdg_toplevel = xdg_surface.get_toplevel();
	xdg_toplevel.set_app_id("khronos-egl-test".to_string());
	xdg_toplevel.set_title("Test".to_string());

	wl_surface.commit();
	ctx.display.flush().unwrap();

	let surface = Arc::new(Surface {
		handle: wl_surface,
		initialized: AtomicBool::new(false),
	});

	let weak_surface = Arc::downgrade(&surface);

	let egl = egl.clone();
	xdg_surface.quick_assign(
		move |xdg_surface: Main<XdgSurface>, event: xdg_surface::Event, _dd: DispatchData| {
			use xdg_surface::Event::*;

			match event {
				Configure { serial } => {
					if let Some(surface) = weak_surface.upgrade() {
						if !surface.initialized.swap(true, Ordering::Relaxed) {
							let wl_egl_surface =
								wayland_egl::WlEglSurface::new(&surface.handle, width, height);

							let egl_surface = unsafe {
								egl.create_window_surface(
									egl_display,
									egl_config,
									wl_egl_surface.ptr() as egl::NativeWindowType,
									None,
								)
								.expect("unable to create an EGL surface")
							};

							egl.make_current(
								egl_display,
								Some(egl_surface),
								Some(egl_surface),
								Some(egl_context),
							)
							.expect("unable to bind the context");

							render();

							egl.swap_buffers(egl_display, egl_surface)
								.expect("unable to post the surface content");

							xdg_surface.ack_configure(serial);
						}
					}
				}
				_ => (),
			}
		},
	);

	surface
}

fn main() {
	let egl = unsafe {
		Arc::new(
			egl::DynamicInstance::<egl::EGL1_5>::load_required()
				.expect("unable to load libEGL.so.1"),
		)
	};

	// Setup Open GL.
	egl.bind_api(egl::OPENGL_API)
		.expect("unable to select OpenGL API");
	gl::load_with(|name| egl.get_proc_address(name).unwrap() as *const std::ffi::c_void);

	// Setup the Wayland client.
	let mut ctx = setup_wayland();

	// Setup EGL.
	let egl_display = setup_egl(&egl, &ctx.display);
	let (egl_context, egl_config) = create_context(&egl, egl_display);

	// Create a surface.
	// Note that it must be kept alive to the end of execution.
	let _surface = create_surface(&egl, &ctx, egl_display, egl_context, egl_config, 800, 600);

	loop {
		ctx.event_queue
			.dispatch(&mut (), |_, _, _| { /* we ignore unfiltered messages */ })
			.unwrap();
	}
}

const VERTEX: &'static [GLint; 8] = &[-1, -1, 1, -1, 1, 1, -1, 1];

const INDEXES: &'static [GLuint; 4] = &[0, 1, 2, 3];

const VERTEX_SHADER: &[u8] = b"#version 400
in vec2 position;

void main() {
	gl_Position = vec4(position, 0.0f, 1.0f);
}
\0";

const FRAGMENT_SHADER: &[u8] = b"#version 400
out vec4 color;

void main() {
	color = vec4(1.0f, 0.0f, 0.0f, 1.0f);
}
\0";

fn render() {
	unsafe {
		let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
		check_gl_errors();
		let src = CStr::from_bytes_with_nul_unchecked(VERTEX_SHADER).as_ptr();
		gl::ShaderSource(vertex_shader, 1, (&[src]).as_ptr(), ptr::null());
		check_gl_errors();
		gl::CompileShader(vertex_shader);
		check_shader_status(vertex_shader);

		let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
		check_gl_errors();
		let src = CStr::from_bytes_with_nul_unchecked(FRAGMENT_SHADER).as_ptr();
		gl::ShaderSource(fragment_shader, 1, (&[src]).as_ptr(), ptr::null());
		check_gl_errors();
		gl::CompileShader(fragment_shader);
		check_shader_status(fragment_shader);

		let program = gl::CreateProgram();
		check_gl_errors();
		gl::AttachShader(program, vertex_shader);
		check_gl_errors();
		gl::AttachShader(program, fragment_shader);
		check_gl_errors();
		gl::LinkProgram(program);
		check_gl_errors();
		gl::UseProgram(program);
		check_gl_errors();

		let mut buffer = 0;
		gl::GenBuffers(1, &mut buffer);
		check_gl_errors();
		gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
		check_gl_errors();
		gl::BufferData(
			gl::ARRAY_BUFFER,
			8 * 4,
			VERTEX.as_ptr() as *const std::ffi::c_void,
			gl::STATIC_DRAW,
		);
		check_gl_errors();

		let mut vertex_input = 0;
		gl::GenVertexArrays(1, &mut vertex_input);
		check_gl_errors();
		gl::BindVertexArray(vertex_input);
		check_gl_errors();
		gl::EnableVertexAttribArray(0);
		check_gl_errors();
		gl::VertexAttribPointer(0, 2, gl::INT, gl::FALSE as GLboolean, 0, 0 as *const GLvoid);
		check_gl_errors();

		let mut indexes = 0;
		gl::GenBuffers(1, &mut indexes);
		check_gl_errors();
		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indexes);
		check_gl_errors();
		gl::BufferData(
			gl::ELEMENT_ARRAY_BUFFER,
			4 * 4,
			INDEXES.as_ptr() as *const std::ffi::c_void,
			gl::STATIC_DRAW,
		);
		check_gl_errors();

		gl::DrawElements(gl::TRIANGLE_FAN, 4, gl::UNSIGNED_INT, std::ptr::null());
		check_gl_errors();
	}
}

fn format_error(e: GLenum) -> &'static str {
	match e {
		gl::NO_ERROR => "No error",
		gl::INVALID_ENUM => "Invalid enum",
		gl::INVALID_VALUE => "Invalid value",
		gl::INVALID_OPERATION => "Invalid operation",
		gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid framebuffer operation",
		gl::OUT_OF_MEMORY => "Out of memory",
		gl::STACK_UNDERFLOW => "Stack underflow",
		gl::STACK_OVERFLOW => "Stack overflow",
		_ => "Unknown error",
	}
}

pub fn check_gl_errors() {
	unsafe {
		match gl::GetError() {
			gl::NO_ERROR => (),
			e => {
				panic!("OpenGL error: {}", format_error(e))
			}
		}
	}
}

unsafe fn check_shader_status(shader: GLuint) {
	let mut status = gl::FALSE as GLint;
	gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
	if status != (gl::TRUE as GLint) {
		let mut len = 0;
		gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut len);
		if len > 0 {
			let mut buf = Vec::with_capacity(len as usize);
			buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
			gl::GetProgramInfoLog(
				shader,
				len,
				ptr::null_mut(),
				buf.as_mut_ptr() as *mut GLchar,
			);

			let log = String::from_utf8(buf).unwrap();
			eprintln!("shader compilation log:\n{}", log);
		}

		panic!("shader compilation failed");
	}
}
