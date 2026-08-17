#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::incompatible_msrv)]

use core::{cell::OnceCell, ptr::NonNull};

use objc2::{
    declare_class, msg_send_id, mutability::MainThreadOnly, rc::Retained, runtime::ProtocolObject,
    ClassType, DeclaredClass,
};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{
    ns_string, MainThreadMarker, NSDate, NSNotification, NSObject, NSObjectProtocol, NSPoint,
    NSRect, NSSize,
};
use objc2_metal::{
    MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice,
    MTLLibrary, MTLPackedFloat3, MTLPrimitiveType, MTLRenderCommandEncoder,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState,
};
use objc2_metal_kit::{MTKView, MTKViewDelegate};

#[derive(Copy, Clone)]
#[repr(C)]
struct SceneProperties {
    time: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct VertexInput {
    position: MTLPackedFloat3,
    color: MTLPackedFloat3,
}

macro_rules! idcell {
    ($name:ident => $this:expr) => {
        $this.ivars().$name.set($name).expect(&format!(
            "ivar should not already be initialized: `{}`",
            stringify!($name)
        ));
    };
    ($name:ident <= $this:expr) => {
        #[rustfmt::skip]
        let Some($name) = $this.ivars().$name.get() else {
            unreachable!(
                "ivar should be initialized: `{}`",
                stringify!($name)
            )
        };
    };
}

// declare the desired instance variables
struct Ivars {
    start_date: Retained<NSDate>,
    command_queue: OnceCell<Retained<ProtocolObject<dyn MTLCommandQueue>>>,
    pipeline_state: OnceCell<Retained<ProtocolObject<dyn MTLRenderPipelineState>>>,
    window: OnceCell<Retained<NSWindow>>,
}

// declare the Objective-C class machinery
declare_class!(
    struct Delegate;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is an application delegate.
    // - `Delegate` does not implement `Drop`.
    unsafe impl ClassType for Delegate {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
        const NAME: &'static str = "Delegate";
    }

    impl DeclaredClass for Delegate {
        type Ivars = Ivars;
    }

    unsafe impl NSObjectProtocol for Delegate {}

    // define the delegate methods for the `NSApplicationDelegate` protocol
    unsafe impl NSApplicationDelegate for Delegate {
        #[method(applicationDidFinishLaunching:)]
        #[allow(non_snake_case)]
        unsafe fn applicationDidFinishLaunching(&self, _notification: &NSNotification) {
            let mtm = MainThreadMarker::from(self);
            // create the app window
            let window = {
                let content_rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(768., 768.));
                let style = NSWindowStyleMask::Closable
                    | NSWindowStyleMask::Resizable
                    | NSWindowStyleMask::Titled;
                let backing_store_type = NSBackingStoreType::NSBackingStoreBuffered;
                let flag = false;
                unsafe {
                    NSWindow::initWithContentRect_styleMask_backing_defer(
                        mtm.alloc(),
                        content_rect,
                        style,
                        backing_store_type,
                        flag,
                    )
                }
            };

            // get the default device
            let device = {
                let ptr = unsafe { MTLCreateSystemDefaultDevice() };
                unsafe { Retained::retain(ptr) }.expect("Failed to get default system device.")
            };

            // create the command queue
            let command_queue = device
                .newCommandQueue()
                .expect("Failed to create a command queue.");

            // create the metal view
            let mtk_view = {
                let frame_rect = window.frame();
                unsafe { MTKView::initWithFrame_device(mtm.alloc(), frame_rect, Some(&device)) }
            };

            // create the pipeline descriptor
            let pipeline_descriptor = MTLRenderPipelineDescriptor::new();

            unsafe {
                pipeline_descriptor
                    .colorAttachments()
                    .objectAtIndexedSubscript(0)
                    .setPixelFormat(mtk_view.colorPixelFormat());
            }

            // compile the shaders
            let library = device
                .newLibraryWithSource_options_error(
                    ns_string!(include_str!("triangle.metal")),
                    None,
                )
                .expect("Failed to create a library.");

            // configure the vertex shader
            let vertex_function = library.newFunctionWithName(ns_string!("vertex_main"));
            pipeline_descriptor.setVertexFunction(vertex_function.as_deref());

            // configure the fragment shader
            let fragment_function = library.newFunctionWithName(ns_string!("fragment_main"));
            pipeline_descriptor.setFragmentFunction(fragment_function.as_deref());

            // create the pipeline state
            let pipeline_state = device
                .newRenderPipelineStateWithDescriptor_error(&pipeline_descriptor)
                .expect("Failed to create a pipeline state.");

            // configure the metal view delegate
            unsafe {
                let object = ProtocolObject::from_ref(self);
                mtk_view.setDelegate(Some(object));
            }

            // configure the window
            window.setContentView(Some(&mtk_view));
            window.center();
            window.setTitle(ns_string!("metal example"));
            window.makeKeyAndOrderFront(None);

            // initialize the delegate state
            idcell!(command_queue => self);
            idcell!(pipeline_state => self);
            idcell!(window => self);
        }
    }

    // define the delegate methods for the `MTKViewDelegate` protocol
    unsafe impl MTKViewDelegate for Delegate {
        #[method(drawInMTKView:)]
        #[allow(non_snake_case)]
        unsafe fn drawInMTKView(&self, mtk_view: &MTKView) {
            idcell!(command_queue <= self);
            idcell!(pipeline_state <= self);

            // prepare for drawing
            let Some(current_drawable) = (unsafe { mtk_view.currentDrawable() }) else {
                return;
            };
            let Some(command_buffer) = command_queue.commandBuffer() else {
                return;
            };
            let Some(pass_descriptor) = (unsafe { mtk_view.currentRenderPassDescriptor() }) else {
                return;
            };
            let Some(encoder) = command_buffer.renderCommandEncoderWithDescriptor(&pass_descriptor)
            else {
                return;
            };

            // compute the scene properties
            let scene_properties_data = &SceneProperties {
                time: unsafe { self.ivars().start_date.timeIntervalSinceNow() } as f32,
            };
            // write the scene properties to the vertex shader argument buffer at index 0
            let scene_properties_bytes = NonNull::from(scene_properties_data);
            unsafe {
                encoder.setVertexBytes_length_atIndex(
                    scene_properties_bytes.cast::<core::ffi::c_void>(),
                    core::mem::size_of_val(scene_properties_data),
                    0,
                )
            };

            // compute the triangle geometry
            let vertex_input_data: &[VertexInput] = &[
                VertexInput {
                    position: MTLPackedFloat3 {
                        x: -f32::sqrt(3.0) / 4.0,
                        y: -0.25,
                        z: 0.,
                    },
                    color: MTLPackedFloat3 {
                        x: 1.,
                        y: 0.,
                        z: 0.,
                    },
                },
                VertexInput {
                    position: MTLPackedFloat3 {
                        x: f32::sqrt(3.0) / 4.0,
                        y: -0.25,
                        z: 0.,
                    },
                    color: MTLPackedFloat3 {
                        x: 0.,
                        y: 1.,
                        z: 0.,
                    },
                },
                VertexInput {
                    position: MTLPackedFloat3 {
                        x: 0.,
                        y: 0.5,
                        z: 0.,
                    },
                    color: MTLPackedFloat3 {
                        x: 0.,
                        y: 0.,
                        z: 1.,
                    },
                },
            ];
            // write the triangle geometry to the vertex shader argument buffer at index 1
            let vertex_input_bytes = NonNull::from(vertex_input_data);
            unsafe {
                encoder.setVertexBytes_length_atIndex(
                    vertex_input_bytes.cast::<core::ffi::c_void>(),
                    core::mem::size_of_val(vertex_input_data),
                    1,
                )
            };

            // configure the encoder with the pipeline and draw the triangle
            encoder.setRenderPipelineState(pipeline_state);
            unsafe {
                encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 3)
            };
            encoder.endEncoding();

            // schedule the command buffer for display and commit
            command_buffer.presentDrawable(ProtocolObject::from_ref(&*current_drawable));
            command_buffer.commit();
        }

        #[method(mtkView:drawableSizeWillChange:)]
        #[allow(non_snake_case)]
        unsafe fn mtkView_drawableSizeWillChange(&self, _view: &MTKView, _size: NSSize) {
            // println!("mtkView_drawableSizeWillChange");
        }
    }
);

impl Delegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars {
            start_date: unsafe { NSDate::now() },
            command_queue: OnceCell::default(),
            pipeline_state: OnceCell::default(),
            window: OnceCell::default(),
        });
        unsafe { msg_send_id![super(this), init] }
    }
}

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    // configure the app
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

    // configure the application delegate
    let delegate = Delegate::new(mtm);
    let object = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(object));

    // run the app
    unsafe { app.run() };
}
