use super::*;
use crate::{gl46 as native_gl, version::Version};
use std::ffi::CStr;
use std::ptr;
use std::{collections::HashSet, ffi::CString, num::NonZeroU32};

macro_rules! khr_fallback {
    ($gl:ident . $is_loaded:ident, $gl2:ident . $khr:ident, $gl3:ident . $core:ident ($($arg:expr),* $(,)?) $(,)?) => {
        if $gl.$is_loaded() {
            $gl3.$core($($arg),*)
        } else {
            $gl2.$khr($($arg),*)
        }
    };
}

#[derive(Default)]
struct Constants {
    max_label_length: i32,
}

/// Store a boxed callback (i.e., `Box<Box<dyn FnMut(...)>>`) as a raw pointer, so that it can be
/// referenced by the C API and later converted back into a `Box` and dropped.
///
/// We use a raw pointer here because `Box` aliasing rules are not fully defined, so we can'
/// guarantee that it's not undefined behavior to keep a `Box` here while it's used as a raw
/// pointer in the C API.
struct DebugCallbackRawPtr {
    callback: *mut std::os::raw::c_void,
}

unsafe impl Send for DebugCallbackRawPtr {}
unsafe impl Sync for DebugCallbackRawPtr {}

impl Drop for DebugCallbackRawPtr {
    fn drop(&mut self) {
        unsafe {
            // Convert callback back into `Box` and drop it.
            let thin_ptr = Box::from_raw(self.callback as *mut DebugCallback);
            let callback = *thin_ptr;
            drop(callback);
        }
    }
}

pub struct Context {
    raw: native_gl::GlFns,
    extensions: HashSet<String>,
    constants: Constants,
    version: Version,
    debug_callback: Option<DebugCallbackRawPtr>,
}

impl Context {
    pub unsafe fn from_loader_function_cstr<F>(mut loader_function: F) -> Self
    where
        F: FnMut(&CStr) -> *const std::os::raw::c_void,
    {
        let raw: native_gl::GlFns =
            native_gl::GlFns::load_with(|p: *const std::os::raw::c_char| {
                let c_str = std::ffi::CStr::from_ptr(p);
                loader_function(c_str) as *mut std::os::raw::c_void
            });

        // Retrieve and parse `GL_VERSION`
        let raw_string = raw.GetString(VERSION);

        if raw_string.is_null() {
            panic!("Reading GL_VERSION failed. Make sure there is a valid GL context currently active.")
        }

        let raw_version = std::ffi::CStr::from_ptr(raw_string as *const native_gl::GLchar)
            .to_str()
            .unwrap()
            .to_owned();
        let version = Version::parse(&raw_version).unwrap();

        // Setup extensions and constants after the context has been built
        let mut context = Self {
            raw,
            extensions: HashSet::new(),
            constants: Constants::default(),
            version,
            debug_callback: None,
        };

        // Use core-only functions to populate extension list
        if (context.version >= Version::new(3, 0, None, String::from("")))
            || (context.version >= Version::new_embedded(3, 0, String::from("")))
        {
            let num_extensions = context.get_parameter_i32(NUM_EXTENSIONS);
            for i in 0..num_extensions {
                let extension_name = context.get_parameter_indexed_string(EXTENSIONS, i as u32);
                context.extensions.insert(extension_name);
            }
        } else {
            // Fallback
            context.extensions.extend(
                context
                    .get_parameter_string(EXTENSIONS)
                    .split(' ')
                    .map(|s| s.to_string()),
            );
        };

        // After the extensions are known, we can populate constants (including
        // constants that depend on extensions being enabled)
        context.constants.max_label_length = if context.supports_debug() {
            context.get_parameter_i32(MAX_LABEL_LENGTH)
        } else {
            0
        };

        context
    }

    pub unsafe fn from_loader_function<F>(mut loader_function: F) -> Self
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
        Self::from_loader_function_cstr(move |name| loader_function(name.to_str().unwrap()))
    }

    /// Creates a texture from an external GL name.
    ///
    /// This can be useful when a texture is created outside of glow (e.g. OpenXR surface) but glow
    /// still needs access to it for rendering.
    #[deprecated = "Use the NativeTexture constructor instead"]
    pub unsafe fn create_texture_from_gl_name(gl_name: native_gl::GLuint) -> NativeTexture {
        NativeTexture(non_zero_gl_name(gl_name))
    }

    /// Creates a framebuffer from an external GL name.
    ///
    /// This can be useful when a framebuffer is created outside of glow (e.g: via `surfman` or another
    /// crate that supports sharing of buffers between GL contexts), but glow needs to set it as a target.
    #[deprecated = "Use the NativeFramebuffer constructor instead"]
    pub unsafe fn create_framebuffer_from_gl_name(gl_name: native_gl::GLuint) -> NativeFramebuffer {
        NativeFramebuffer(non_zero_gl_name(gl_name))
    }

    unsafe fn get_parameter_gl_name(&self, parameter: u32) -> Option<NonZeroU32> {
        let value = self.get_parameter_i32(parameter) as u32;
        if value == 0 {
            None
        } else {
            Some(non_zero_gl_name(value))
        }
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Native_GL_Context")
    }
}

fn non_zero_gl_name(value: native_gl::GLuint) -> NonZeroU32 {
    NonZeroU32::new(value as u32).expect("expected non-zero GL name")
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeShader(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeProgram(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeBuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeVertexArray(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeTexture(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeSampler(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeFence(pub native_gl::GLsync);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeFramebuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeRenderbuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeQuery(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeUniformLocation(pub native_gl::GLuint);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeTransformFeedback(pub NonZeroU32);

impl crate::__private::Sealed for Context {}

impl HasContext for Context {
    type Shader = NativeShader;
    type Program = NativeProgram;
    type Buffer = NativeBuffer;
    type VertexArray = NativeVertexArray;
    type Texture = NativeTexture;
    type Sampler = NativeSampler;
    type Fence = NativeFence;
    type Framebuffer = NativeFramebuffer;
    type Renderbuffer = NativeRenderbuffer;
    type Query = NativeQuery;
    type UniformLocation = NativeUniformLocation;
    type TransformFeedback = NativeTransformFeedback;

    fn supported_extensions(&self) -> &HashSet<String> {
        &self.extensions
    }

    fn supports_debug(&self) -> bool {
        if self.extensions.contains("GL_KHR_debug") {
            // Supports extension (either GL or GL ES)
            true
        } else if self.version.is_embedded {
            // GL ES >= 3.2
            self.version.major == 3 && self.version.minor >= 2
        } else {
            // GL >= 4.3
            self.version.major == 4 && self.version.minor >= 3
        }
    }

    fn version(&self) -> &Version {
        &self.version
    }

    unsafe fn create_framebuffer(&self) -> Result<Self::Framebuffer, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenFramebuffers(1, &mut name);
        NonZeroU32::new(name)
            .map(NativeFramebuffer)
            .ok_or_else(|| String::from("Unable to create Framebuffer object"))
    }

    unsafe fn create_named_framebuffer(&self) -> Result<Self::Framebuffer, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.CreateFramebuffers(1, &mut name);
        NonZeroU32::new(name)
            .map(NativeFramebuffer)
            .ok_or_else(|| String::from("Unable to create Framebuffer object"))
    }

    unsafe fn is_framebuffer(&self, framebuffer: Self::Framebuffer) -> bool {
        let gl = &self.raw;
        gl.IsFramebuffer(framebuffer.0.get()) != 0
    }

    unsafe fn create_query(&self) -> Result<Self::Query, String> {
        let gl = &self.raw;
        let mut name = 0;
        if gl.GenQueries_is_loaded() {
            gl.GenQueries(1, &mut name);
        } else {
            gl.GenQueriesEXT(1, &mut name);
        }
        NonZeroU32::new(name)
            .map(NativeQuery)
            .ok_or_else(|| String::from("Unable to create Query object"))
    }

    unsafe fn create_renderbuffer(&self) -> Result<Self::Renderbuffer, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenRenderbuffers(1, &mut name);
        NonZeroU32::new(name)
            .map(NativeRenderbuffer)
            .ok_or_else(|| String::from("Unable to create Renderbuffer object"))
    }

    unsafe fn is_renderbuffer(&self, renderbuffer: Self::Renderbuffer) -> bool {
        let gl = &self.raw;
        gl.IsRenderbuffer(renderbuffer.0.get()) != 0
    }

    unsafe fn create_sampler(&self) -> Result<Self::Sampler, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenSamplers(1, &mut name);
        NonZeroU32::new(name)
            .map(NativeSampler)
            .ok_or_else(|| String::from("Unable to create Sampler object"))
    }

    unsafe fn create_shader(&self, shader_type: u32) -> Result<Self::Shader, String> {
        let gl = &self.raw;
        NonZeroU32::new(gl.CreateShader(shader_type as u32))
            .map(NativeShader)
            .ok_or_else(|| String::from("Unable to create Shader object"))
    }

    unsafe fn is_shader(&self, shader: Self::Shader) -> bool {
        let gl = &self.raw;
        gl.IsShader(shader.0.get()) != 0
    }

    unsafe fn create_texture(&self) -> Result<Self::Texture, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenTextures(1, &mut name);
        NonZeroU32::new(name)
            .map(NativeTexture)
            .ok_or_else(|| String::from("Unable to create Texture object"))
    }

    unsafe fn create_named_texture(&self, target: u32) -> Result<Self::Texture, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.CreateTextures(target, 1, &mut name);
        NonZeroU32::new(name)
            .map(NativeTexture)
            .ok_or_else(|| String::from("Unable to create Texture object"))
    }

    unsafe fn is_texture(&self, texture: Self::Texture) -> bool {
        let gl = &self.raw;
        gl.IsTexture(texture.0.get()) != 0
    }

    unsafe fn delete_shader(&self, shader: Self::Shader) {
        let gl = &self.raw;
        gl.DeleteShader(shader.0.get());
    }

    unsafe fn shader_source(&self, shader: Self::Shader, source: &str) {
        let gl = &self.raw;
        gl.ShaderSource(
            shader.0.get(),
            1,
            &(source.as_ptr() as *const native_gl::GLchar),
            &(source.len() as native_gl::GLint),
        );
    }

    unsafe fn compile_shader(&self, shader: Self::Shader) {
        let gl = &self.raw;
        gl.CompileShader(shader.0.get());
    }

    unsafe fn get_shader_completion_status(&self, shader: Self::Shader) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetShaderiv(shader.0.get(), COMPLETION_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_shader_compile_status(&self, shader: Self::Shader) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetShaderiv(shader.0.get(), COMPILE_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_shader_info_log(&self, shader: Self::Shader) -> String {
        let gl = &self.raw;
        let mut length = 0;
        gl.GetShaderiv(shader.0.get(), INFO_LOG_LENGTH, &mut length);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            gl.GetShaderInfoLog(
                shader.0.get(),
                length,
                &mut length,
                (&log[..]).as_ptr() as *mut native_gl::GLchar,
            );
            log.truncate(length as usize);
            log
        } else {
            String::from("")
        }
    }

    unsafe fn get_shader_precision_format(
        &self,
        shader_type: u32,
        precision_type: u32,
    ) -> Option<ShaderPrecisionFormat> {
        let gl = &self.raw;

        if gl.GetShaderPrecisionFormat_is_loaded() {
            let mut range = [0, 0];
            let mut precision = 0;
            gl.GetShaderPrecisionFormat(
                shader_type,
                precision_type,
                range.as_mut_ptr(),
                &mut precision,
            );
            // In some cases GetShaderPrecisionFormat exists but it's just a stub
            // so we return only if variables got populated
            if range[1] != 0 {
                return Some(ShaderPrecisionFormat {
                    range_min: range[0],
                    range_max: range[1],
                    precision,
                });
            }
        }

        None
    }

    unsafe fn get_tex_image(
        &self,
        target: u32,
        level: i32,
        format: u32,
        ty: u32,
        pixels: PixelPackData,
    ) {
        let gl = &self.raw;
        gl.GetTexImage(
            target,
            level,
            format,
            ty,
            match pixels {
                PixelPackData::BufferOffset(offset) => offset as *mut std::ffi::c_void,
                PixelPackData::Slice(Some(data)) => data.as_mut_ptr() as *mut std::ffi::c_void,
                PixelPackData::Slice(None) => ptr::null_mut(),
            },
        );
    }

    unsafe fn create_program(&self) -> Result<Self::Program, String> {
        let gl = &self.raw;
        NonZeroU32::new(gl.CreateProgram())
            .map(NativeProgram)
            .ok_or_else(|| String::from("Unable to create Program object"))
    }

    unsafe fn is_program(&self, program: Self::Program) -> bool {
        let gl = &self.raw;
        gl.IsProgram(program.0.get()) != 0
    }

    unsafe fn delete_program(&self, program: Self::Program) {
        let gl = &self.raw;
        gl.DeleteProgram(program.0.get());
    }

    unsafe fn attach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let gl = &self.raw;
        gl.AttachShader(program.0.get(), shader.0.get());
    }

    unsafe fn detach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let gl = &self.raw;
        gl.DetachShader(program.0.get(), shader.0.get());
    }

    unsafe fn link_program(&self, program: Self::Program) {
        let gl = &self.raw;
        gl.LinkProgram(program.0.get());
    }

    unsafe fn validate_program(&self, program: Self::Program) {
        let gl = &self.raw;
        gl.ValidateProgram(program.0.get());
    }

    unsafe fn get_program_completion_status(&self, program: Self::Program) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetProgramiv(program.0.get(), COMPLETION_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_program_link_status(&self, program: Self::Program) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetProgramiv(program.0.get(), LINK_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_program_validate_status(&self, program: Self::Program) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetProgramiv(program.0.get(), VALIDATE_STATUS, &mut status);
        status == 1
    }

    unsafe fn get_program_parameter_i32(&self, program: Self::Program, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetProgramiv(program.0.get(), parameter, &mut value);
        value
    }

    unsafe fn get_program_info_log(&self, program: Self::Program) -> String {
        let gl = &self.raw;
        let mut length = 0;
        gl.GetProgramiv(program.0.get(), INFO_LOG_LENGTH, &mut length);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            gl.GetProgramInfoLog(
                program.0.get(),
                length,
                &mut length,
                (&log[..]).as_ptr() as *mut native_gl::GLchar,
            );
            log.truncate(length as usize);
            log
        } else {
            String::from("")
        }
    }

    unsafe fn get_program_resource_i32(
        &self,
        program: Self::Program,
        interface: u32,
        index: u32,
        properties: &[u32],
    ) -> Vec<i32> {
        let gl = &self.raw;
        // query the number of output parameters first
        let mut length = 0i32;
        gl.GetProgramResourceiv(
            program.0.get(),
            interface,
            index,
            properties.len() as i32,
            properties.as_ptr(),
            0,
            &mut length,
            ptr::null_mut(),
        );
        // get the parameter values
        let mut params = vec![0i32; length as usize];
        gl.GetProgramResourceiv(
            program.0.get(),
            interface,
            index,
            properties.len() as i32,
            properties.as_ptr(),
            length,
            &mut length,
            params.as_mut_ptr(),
        );
        params
    }

    unsafe fn program_uniform_1_i32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: i32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform1i(program.0.get(), loc.0 as i32, x);
        }
    }

    unsafe fn program_uniform_2_i32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: i32,
        y: i32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform2i(program.0.get(), loc.0 as i32, x, y);
        }
    }

    unsafe fn program_uniform_3_i32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform3i(program.0.get(), loc.0 as i32, x, y, z);
        }
    }

    unsafe fn program_uniform_4_i32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform4i(program.0.get(), loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn program_uniform_1_i32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[i32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform1iv(program.0.get(), loc.0 as i32, v.len() as i32, v.as_ptr());
        }
    }

    unsafe fn program_uniform_2_i32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[i32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform2iv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 2,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_3_i32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[i32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform3iv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 3,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_4_i32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[i32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform4iv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 4,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_1_u32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: u32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform1ui(program.0.get(), loc.0 as i32, x);
        }
    }

    unsafe fn program_uniform_2_u32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: u32,
        y: u32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform2ui(program.0.get(), loc.0 as i32, x, y);
        }
    }

    unsafe fn program_uniform_3_u32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform3ui(program.0.get(), loc.0 as i32, x, y, z);
        }
    }

    unsafe fn program_uniform_4_u32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform4ui(program.0.get(), loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn program_uniform_1_u32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[u32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform1uiv(program.0.get(), loc.0 as i32, v.len() as i32, v.as_ptr());
        }
    }

    unsafe fn program_uniform_2_u32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[u32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform2uiv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 2,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_3_u32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[u32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform3uiv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 3,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_4_u32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[u32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform4uiv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 4,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_1_f32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: f32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform1f(program.0.get(), loc.0 as i32, x);
        }
    }

    unsafe fn program_uniform_2_f32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: f32,
        y: f32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform2f(program.0.get(), loc.0 as i32, x, y);
        }
    }

    unsafe fn program_uniform_3_f32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform3f(program.0.get(), loc.0 as i32, x, y, z);
        }
    }

    unsafe fn program_uniform_4_f32(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform4f(program.0.get(), loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn program_uniform_1_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform1fv(program.0.get(), loc.0 as i32, v.len() as i32, v.as_ptr());
        }
    }

    unsafe fn program_uniform_2_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform2fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 2,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_3_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform3fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 3,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_4_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniform4fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 4,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_2_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix2fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 4,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_2x3_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix2x3fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 6,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_2x4_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix2x4fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 8,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_3x2_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix3x2fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 6,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_3_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix3fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 9,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_3x4_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix3x4fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 12,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_4x2_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix4x2fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 8,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_4x3_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix4x3fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 12,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_uniform_matrix_4_f32_slice(
        &self,
        program: Self::Program,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.ProgramUniformMatrix4fv(
                program.0.get(),
                loc.0 as i32,
                v.len() as i32 / 16,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn program_binary_retrievable_hint(&self, program: Self::Program, value: bool) {
        let gl = &self.raw;
        gl.ProgramParameteri(
            program.0.get(),
            crate::PROGRAM_BINARY_RETRIEVABLE_HINT,
            value as i32,
        )
    }

    unsafe fn get_program_binary(&self, program: Self::Program) -> Option<ProgramBinary> {
        let gl = &self.raw;

        // We don't need to error check here as if the call fails, length will be returned as 0.
        let mut len = 0;
        gl.GetProgramiv(program.0.get(), crate::PROGRAM_BINARY_LENGTH, &mut len);

        let mut format = 0;
        let mut buffer = vec![0u8; len as usize];

        gl.GetProgramBinary(
            program.0.get(),
            len,
            ptr::null_mut(),
            &mut format,
            buffer.as_mut_ptr() as *mut core::ffi::c_void,
        );

        if gl.GetError() == crate::NO_ERROR {
            Some(ProgramBinary { buffer, format })
        } else {
            None
        }
    }

    unsafe fn program_binary(&self, program: Self::Program, binary: &ProgramBinary) {
        let gl = &self.raw;

        gl.ProgramBinary(
            program.0.get(),
            binary.format,
            binary.buffer.as_ptr() as *const core::ffi::c_void,
            binary.buffer.len() as native_gl::types::GLsizei,
        )
    }

    unsafe fn get_active_uniforms(&self, program: Self::Program) -> u32 {
        let gl = &self.raw;
        let mut count = 0;
        gl.GetProgramiv(program.0.get(), ACTIVE_UNIFORMS, &mut count);
        count as u32
    }

    unsafe fn get_active_uniforms_parameter(
        &self,
        program: Self::Program,
        uniforms: &[u32],
        pname: u32,
    ) -> Vec<i32> {
        let gl = &self.raw;
        let mut results = vec![0; uniforms.len()];
        gl.GetActiveUniformsiv(
            program.0.get(),
            uniforms.len() as _,
            uniforms.as_ptr(),
            pname,
            results.as_mut_ptr(),
        );
        results
    }

    unsafe fn get_active_uniform(
        &self,
        program: Self::Program,
        index: u32,
    ) -> Option<ActiveUniform> {
        let gl = &self.raw;
        let mut uniform_max_size = 0;
        gl.GetProgramiv(
            program.0.get(),
            ACTIVE_UNIFORM_MAX_LENGTH,
            &mut uniform_max_size,
        );

        let mut name = String::with_capacity(uniform_max_size as usize);
        name.extend(std::iter::repeat('\0').take(uniform_max_size as usize));
        let mut length = 0;
        let mut size = 0;
        let mut utype = 0;
        gl.GetActiveUniform(
            program.0.get(),
            index,
            uniform_max_size,
            &mut length,
            &mut size,
            &mut utype,
            name.as_ptr() as *mut native_gl::GLchar,
        );
        name.truncate(length as usize);

        Some(ActiveUniform { size, utype, name })
    }

    unsafe fn use_program(&self, program: Option<Self::Program>) {
        let gl = &self.raw;
        gl.UseProgram(program.map(|p| p.0.get()).unwrap_or(0));
    }

    unsafe fn create_buffer(&self) -> Result<Self::Buffer, String> {
        let gl = &self.raw;
        let mut buffer = 0;
        gl.GenBuffers(1, &mut buffer);
        NonZeroU32::new(buffer)
            .map(NativeBuffer)
            .ok_or_else(|| String::from("Unable to create Buffer object"))
    }

    unsafe fn create_named_buffer(&self) -> Result<Self::Buffer, String> {
        let gl = &self.raw;
        let mut buffer = 0;
        gl.CreateBuffers(1, &mut buffer);
        NonZeroU32::new(buffer)
            .map(NativeBuffer)
            .ok_or_else(|| String::from("Unable to create Buffer object"))
    }

    unsafe fn is_buffer(&self, buffer: Self::Buffer) -> bool {
        let gl = &self.raw;
        gl.IsBuffer(buffer.0.get()) != 0
    }

    unsafe fn bind_buffer(&self, target: u32, buffer: Option<Self::Buffer>) {
        let gl = &self.raw;
        gl.BindBuffer(target, buffer.map(|b| b.0.get()).unwrap_or(0));
    }

    unsafe fn bind_buffer_base(&self, target: u32, index: u32, buffer: Option<Self::Buffer>) {
        let gl = &self.raw;
        gl.BindBufferBase(target, index, buffer.map(|b| b.0.get()).unwrap_or(0));
    }

    unsafe fn bind_buffer_range(
        &self,
        target: u32,
        index: u32,
        buffer: Option<Self::Buffer>,
        offset: i32,
        size: i32,
    ) {
        let gl = &self.raw;
        gl.BindBufferRange(
            target,
            index,
            buffer.map(|b| b.0.get()).unwrap_or(0),
            offset as isize,
            size as isize,
        );
    }

    unsafe fn bind_vertex_buffer(
        &self,
        binding_index: u32,
        buffer: Option<Buffer>,
        offset: i32,
        stride: i32,
    ) {
        let gl = &self.raw;
        gl.BindVertexBuffer(
            binding_index,
            buffer.map(|b| b.0.get()).unwrap_or(0),
            offset as isize,
            stride,
        );
    }

    unsafe fn bind_framebuffer(&self, target: u32, framebuffer: Option<Self::Framebuffer>) {
        let gl = &self.raw;
        gl.BindFramebuffer(target, framebuffer.map(|fb| fb.0.get()).unwrap_or(0));
    }

    unsafe fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<Self::Renderbuffer>) {
        let gl = &self.raw;
        gl.BindRenderbuffer(target, renderbuffer.map(|rb| rb.0.get()).unwrap_or(0));
    }

    unsafe fn blit_framebuffer(
        &self,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: u32,
        filter: u32,
    ) {
        let gl = &self.raw;
        gl.BlitFramebuffer(
            src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
        );
    }

    unsafe fn blit_named_framebuffer(
        &self,
        read_buffer: Option<Self::Framebuffer>,
        draw_buffer: Option<Self::Framebuffer>,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: u32,
        filter: u32,
    ) {
        let gl = &self.raw;
        gl.BlitNamedFramebuffer(
            read_buffer.map(|f| f.0.get()).unwrap_or(0),
            draw_buffer.map(|f| f.0.get()).unwrap_or(0),
            src_x0,
            src_y0,
            src_x1,
            src_y1,
            dst_x0,
            dst_y0,
            dst_x1,
            dst_y1,
            mask,
            filter,
        );
    }

    unsafe fn create_vertex_array(&self) -> Result<Self::VertexArray, String> {
        let gl = &self.raw;
        let mut vertex_array = 0;
        if gl.GenVertexArrays_is_loaded() {
            gl.GenVertexArrays(1, &mut vertex_array);
        } else {
            #[cfg(not(target_vendor = "apple"))]
            gl.GenVertexArraysOES(1, &mut vertex_array);
            #[cfg(target_vendor = "apple")]
            gl.GenVertexArraysAPPLE(1, &mut vertex_array);
        }
        NonZeroU32::new(vertex_array)
            .map(NativeVertexArray)
            .ok_or_else(|| String::from("Unable to create VertexArray object"))
    }

    unsafe fn create_named_vertex_array(&self) -> Result<Self::VertexArray, String> {
        let gl = &self.raw;
        let mut vertex_array = 0;
        gl.CreateVertexArrays(1, &mut vertex_array);
        NonZeroU32::new(vertex_array)
            .map(NativeVertexArray)
            .ok_or_else(|| String::from("Unable to create VertexArray object"))
    }

    unsafe fn delete_vertex_array(&self, vertex_array: Self::VertexArray) {
        let gl = &self.raw;
        if gl.DeleteVertexArrays_is_loaded() {
            gl.DeleteVertexArrays(1, &vertex_array.0.get());
        } else {
            #[cfg(not(target_vendor = "apple"))]
            gl.DeleteVertexArraysOES(1, &vertex_array.0.get());
            #[cfg(target_vendor = "apple")]
            gl.DeleteVertexArraysAPPLE(1, &vertex_array.0.get());
        }
    }

    unsafe fn bind_vertex_array(&self, vertex_array: Option<Self::VertexArray>) {
        let gl = &self.raw;
        if gl.BindVertexArray_is_loaded() {
            gl.BindVertexArray(vertex_array.map(|va| va.0.get()).unwrap_or(0));
        } else {
            #[cfg(not(target_vendor = "apple"))]
            gl.BindVertexArrayOES(vertex_array.map(|va| va.0.get()).unwrap_or(0));
            #[cfg(target_vendor = "apple")]
            gl.BindVertexArrayAPPLE(vertex_array.map(|va| va.0.get()).unwrap_or(0));
        }
    }

    unsafe fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        let gl = &self.raw;
        gl.ClearColor(red, green, blue, alpha);
    }

    unsafe fn supports_f64_precision(&self) -> bool {
        !self.version.is_embedded
    }

    unsafe fn clear_depth_f64(&self, depth: f64) {
        let gl = &self.raw;
        gl.ClearDepth(depth);
    }

    unsafe fn clear_depth_f32(&self, depth: f32) {
        let gl = &self.raw;
        gl.ClearDepthf(depth);
    }

    unsafe fn clear_depth(&self, depth: f64) {
        if self.supports_f64_precision() {
            self.clear_depth_f64(depth);
        } else {
            self.clear_depth_f32(depth as f32);
        }
    }

    unsafe fn clear_stencil(&self, stencil: i32) {
        let gl = &self.raw;
        gl.ClearStencil(stencil);
    }

    unsafe fn clear(&self, mask: u32) {
        let gl = &self.raw;
        gl.Clear(mask);
    }

    unsafe fn patch_parameter_i32(&self, parameter: u32, value: i32) {
        let gl = &self.raw;
        gl.PatchParameteri(parameter, value);
    }

    unsafe fn pixel_store_i32(&self, parameter: u32, value: i32) {
        let gl = &self.raw;
        gl.PixelStorei(parameter, value);
    }

    unsafe fn pixel_store_bool(&self, parameter: u32, value: bool) {
        let gl = &self.raw;
        gl.PixelStorei(parameter, value as i32);
    }

    unsafe fn get_frag_data_location(&self, program: Self::Program, name: &str) -> i32 {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        gl.GetFragDataLocation(program.0.get(), name.as_ptr() as *const native_gl::GLchar)
    }

    unsafe fn bind_frag_data_location(
        &self,
        program: Self::Program,
        color_number: u32,
        name: &str,
    ) {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        gl.BindFragDataLocation(
            program.0.get(),
            color_number,
            name.as_ptr() as *const native_gl::GLchar,
        );
    }

    unsafe fn buffer_data_size(&self, target: u32, size: i32, usage: u32) {
        let gl = &self.raw;
        gl.BufferData(target, size as isize, std::ptr::null(), usage);
    }

    unsafe fn named_buffer_data_size(&self, buffer: Self::Buffer, size: i32, usage: u32) {
        let gl = &self.raw;
        gl.NamedBufferData(buffer.0.get(), size as isize, std::ptr::null(), usage);
    }

    unsafe fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32) {
        let gl = &self.raw;
        gl.BufferData(
            target,
            data.len() as isize,
            data.as_ptr() as *const std::ffi::c_void,
            usage,
        );
    }

    unsafe fn named_buffer_data_u8_slice(&self, buffer: Self::Buffer, data: &[u8], usage: u32) {
        let gl = &self.raw;
        gl.NamedBufferData(
            buffer.0.get(),
            data.len() as isize,
            data.as_ptr() as *const std::ffi::c_void,
            usage,
        );
    }

    unsafe fn buffer_sub_data_u8_slice(&self, target: u32, offset: i32, src_data: &[u8]) {
        let gl = &self.raw;
        gl.BufferSubData(
            target,
            offset as isize,
            src_data.len() as isize,
            src_data.as_ptr() as *const std::ffi::c_void,
        );
    }

    unsafe fn named_buffer_sub_data_u8_slice(
        &self,
        buffer: Self::Buffer,
        offset: i32,
        src_data: &[u8],
    ) {
        let gl = &self.raw;
        gl.NamedBufferSubData(
            buffer.0.get(),
            offset as isize,
            src_data.len() as isize,
            src_data.as_ptr() as *const std::ffi::c_void,
        );
    }

    unsafe fn get_buffer_sub_data(&self, target: u32, offset: i32, dst_data: &mut [u8]) {
        let gl = &self.raw;
        gl.GetBufferSubData(
            target,
            offset as isize,
            dst_data.len() as isize,
            dst_data.as_mut_ptr() as *mut std::ffi::c_void,
        );
    }

    unsafe fn tex_buffer(&self, target: u32, internal_format: u32, buffer: Option<Self::Buffer>) {
        let gl = &self.raw;
        gl.TexBuffer(
            target,
            internal_format,
            buffer.map(|b| b.0.get()).unwrap_or(0),
        );
    }

    unsafe fn buffer_storage(&self, target: u32, size: i32, data: Option<&[u8]>, flags: u32) {
        let gl = &self.raw;
        let size = size as isize;
        let data = data.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void;
        if gl.BufferStorage_is_loaded() {
            gl.BufferStorage(target, size, data, flags);
        } else {
            gl.BufferStorageEXT(target, size, data, flags);
        }
    }
    unsafe fn named_buffer_storage(
        &self,
        buffer: Self::Buffer,
        size: i32,
        data: Option<&[u8]>,
        flags: u32,
    ) {
        let gl = &self.raw;
        let data = data.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void;
        gl.NamedBufferStorage(buffer.0.get(), size as isize, data, flags);
    }

    unsafe fn check_framebuffer_status(&self, target: u32) -> u32 {
        let gl = &self.raw;
        gl.CheckFramebufferStatus(target)
    }

    unsafe fn check_named_framebuffer_status(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        target: u32,
    ) -> u32 {
        let gl = &self.raw;
        gl.CheckNamedFramebufferStatus(framebuffer.map(|f| f.0.get()).unwrap_or(0), target)
    }

    unsafe fn clear_buffer_i32_slice(&self, target: u32, draw_buffer: u32, values: &[i32]) {
        let gl = &self.raw;
        gl.ClearBufferiv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_u32_slice(&self, target: u32, draw_buffer: u32, values: &[u32]) {
        let gl = &self.raw;
        gl.ClearBufferuiv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_f32_slice(&self, target: u32, draw_buffer: u32, values: &[f32]) {
        let gl = &self.raw;
        gl.ClearBufferfv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_depth_stencil(
        &self,
        target: u32,
        draw_buffer: u32,
        depth: f32,
        stencil: i32,
    ) {
        let gl = &self.raw;
        gl.ClearBufferfi(target, draw_buffer as i32, depth, stencil);
    }

    unsafe fn clear_named_framebuffer_i32_slice(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        target: u32,
        draw_buffer: u32,
        values: &[i32],
    ) {
        let gl = &self.raw;
        gl.ClearNamedFramebufferiv(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            target,
            draw_buffer as i32,
            values.as_ptr(),
        );
    }

    unsafe fn clear_named_framebuffer_u32_slice(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        target: u32,
        draw_buffer: u32,
        values: &[u32],
    ) {
        let gl = &self.raw;
        gl.ClearNamedFramebufferuiv(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            target,
            draw_buffer as i32,
            values.as_ptr(),
        );
    }

    unsafe fn clear_named_framebuffer_f32_slice(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        target: u32,
        draw_buffer: u32,
        values: &[f32],
    ) {
        let gl = &self.raw;
        gl.ClearNamedFramebufferfv(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            target,
            draw_buffer as i32,
            values.as_ptr(),
        );
    }

    unsafe fn clear_named_framebuffer_depth_stencil(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        target: u32,
        draw_buffer: u32,
        depth: f32,
        stencil: i32,
    ) {
        let gl = &self.raw;
        gl.ClearNamedFramebufferfi(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            target,
            draw_buffer as i32,
            depth,
            stencil,
        );
    }

    unsafe fn client_wait_sync(&self, fence: Self::Fence, flags: u32, timeout: i32) -> u32 {
        let gl = &self.raw;
        gl.ClientWaitSync(fence.0, flags, timeout as u64)
    }

    unsafe fn get_sync_parameter_i32(&self, fence: Self::Fence, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut v = 0;
        gl.GetSynciv(fence.0, parameter, 1, ptr::null_mut(), &mut v);
        v
    }

    unsafe fn wait_sync(&self, fence: Self::Fence, flags: u32, timeout: u64) {
        let gl = &self.raw;
        gl.WaitSync(fence.0, flags, timeout)
    }

    unsafe fn copy_buffer_sub_data(
        &self,
        src_target: u32,
        dst_target: u32,
        src_offset: i32,
        dst_offset: i32,
        size: i32,
    ) {
        let gl = &self.raw;
        gl.CopyBufferSubData(
            src_target,
            dst_target,
            src_offset as isize,
            dst_offset as isize,
            size as isize,
        );
    }

    unsafe fn copy_image_sub_data(
        &self,
        src_name: Self::Texture,
        src_target: u32,
        src_level: i32,
        src_x: i32,
        src_y: i32,
        src_z: i32,
        dst_name: Self::Texture,
        dst_target: u32,
        dst_level: i32,
        dst_x: i32,
        dst_y: i32,
        dst_z: i32,
        src_width: i32,
        src_height: i32,
        src_depth: i32,
    ) {
        let gl = &self.raw;
        gl.CopyImageSubData(
            src_name.0.get(),
            src_target,
            src_level,
            src_x,
            src_y,
            src_z,
            dst_name.0.get(),
            dst_target,
            dst_level,
            dst_x,
            dst_y,
            dst_z,
            src_width,
            src_height,
            src_depth,
        );
    }

    unsafe fn copy_tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        border: i32,
    ) {
        let gl = &self.raw;
        gl.CopyTexImage2D(target, level, internal_format, x, y, width, height, border);
    }

    unsafe fn copy_tex_sub_image_2d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.CopyTexSubImage2D(target, level, x_offset, y_offset, x, y, width, height);
    }

    unsafe fn copy_tex_sub_image_3d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.CopyTexSubImage3D(
            target, level, x_offset, y_offset, z_offset, x, y, width, height,
        );
    }

    unsafe fn delete_buffer(&self, buffer: Self::Buffer) {
        let gl = &self.raw;
        gl.DeleteBuffers(1, &buffer.0.get());
    }

    unsafe fn delete_framebuffer(&self, framebuffer: Self::Framebuffer) {
        let gl = &self.raw;
        gl.DeleteFramebuffers(1, &framebuffer.0.get());
    }

    unsafe fn delete_query(&self, query: Self::Query) {
        let gl = &self.raw;
        if gl.DeleteQueries_is_loaded() {
            gl.DeleteQueries(1, &query.0.get());
        } else {
            gl.DeleteQueriesEXT(1, &query.0.get());
        }
    }

    unsafe fn delete_renderbuffer(&self, renderbuffer: Self::Renderbuffer) {
        let gl = &self.raw;
        gl.DeleteRenderbuffers(1, &renderbuffer.0.get());
    }

    unsafe fn delete_sampler(&self, sampler: Self::Sampler) {
        let gl = &self.raw;
        gl.DeleteSamplers(1, &sampler.0.get());
    }

    unsafe fn delete_sync(&self, fence: Self::Fence) {
        let gl = &self.raw;
        gl.DeleteSync(fence.0);
    }

    unsafe fn delete_texture(&self, texture: Self::Texture) {
        let gl = &self.raw;
        gl.DeleteTextures(1, &texture.0.get());
    }

    unsafe fn disable(&self, parameter: u32) {
        let gl = &self.raw;
        gl.Disable(parameter);
    }

    unsafe fn disable_draw_buffer(&self, parameter: u32, draw_buffer: u32) {
        let gl = &self.raw;
        gl.Disablei(parameter, draw_buffer);
    }

    unsafe fn disable_vertex_attrib_array(&self, index: u32) {
        let gl = &self.raw;
        gl.DisableVertexAttribArray(index);
    }

    unsafe fn dispatch_compute(&self, groups_x: u32, groups_y: u32, groups_z: u32) {
        let gl = &self.raw;
        gl.DispatchCompute(groups_x, groups_y, groups_z);
    }

    unsafe fn dispatch_compute_indirect(&self, offset: i32) {
        let gl = &self.raw;
        gl.DispatchComputeIndirect(offset as isize);
    }

    unsafe fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        let gl = &self.raw;
        gl.DrawArrays(mode as u32, first, count);
    }

    unsafe fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32) {
        let gl = &self.raw;
        gl.DrawArraysInstanced(mode as u32, first, count, instance_count);
    }

    unsafe fn draw_arrays_instanced_base_instance(
        &self,
        mode: u32,
        first: i32,
        count: i32,
        instance_count: i32,
        base_instance: u32,
    ) {
        let gl = &self.raw;
        gl.DrawArraysInstancedBaseInstance(
            mode as u32,
            first,
            count,
            instance_count,
            base_instance,
        );
    }

    unsafe fn draw_arrays_indirect_offset(&self, mode: u32, offset: i32) {
        let gl = &self.raw;
        gl.DrawArraysIndirect(mode, offset as *const std::ffi::c_void);
    }

    unsafe fn draw_buffer(&self, draw_buffer: u32) {
        let gl = &self.raw;
        gl.DrawBuffer(draw_buffer);
    }

    unsafe fn named_framebuffer_draw_buffer(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        draw_buffer: u32,
    ) {
        let gl = &self.raw;
        gl.NamedFramebufferDrawBuffer(framebuffer.map(|f| f.0.get()).unwrap_or(0), draw_buffer);
    }

    unsafe fn draw_buffers(&self, buffers: &[u32]) {
        let gl = &self.raw;
        gl.DrawBuffers(buffers.len() as i32, buffers.as_ptr());
    }

    unsafe fn named_framebuffer_draw_buffers(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        buffers: &[u32],
    ) {
        let gl = &self.raw;
        gl.NamedFramebufferDrawBuffers(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            buffers.len() as i32,
            buffers.as_ptr(),
        );
    }

    unsafe fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        let gl = &self.raw;
        gl.DrawElements(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn draw_elements_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        base_vertex: i32,
    ) {
        let gl = &self.raw;
        gl.DrawElementsBaseVertex(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
            base_vertex,
        );
    }

    unsafe fn draw_elements_instanced(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
    ) {
        let gl = &self.raw;
        gl.DrawElementsInstanced(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count,
        );
    }

    unsafe fn draw_elements_instanced_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
        base_vertex: i32,
    ) {
        let gl = &self.raw;
        gl.DrawElementsInstancedBaseVertex(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count,
            base_vertex,
        );
    }

    unsafe fn draw_elements_instanced_base_vertex_base_instance(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
        base_vertex: i32,
        base_instance: u32,
    ) {
        let gl = &self.raw;
        gl.DrawElementsInstancedBaseVertexBaseInstance(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count,
            base_vertex,
            base_instance,
        );
    }

    unsafe fn draw_elements_indirect_offset(&self, mode: u32, element_type: u32, offset: i32) {
        let gl = &self.raw;
        gl.DrawElementsIndirect(mode, element_type, offset as *const std::ffi::c_void);
    }

    unsafe fn enable(&self, parameter: u32) {
        let gl = &self.raw;
        gl.Enable(parameter);
    }

    unsafe fn is_enabled(&self, parameter: u32) -> bool {
        let gl = &self.raw;
        gl.IsEnabled(parameter) != 0
    }

    unsafe fn enable_draw_buffer(&self, parameter: u32, draw_buffer: u32) {
        let gl = &self.raw;
        gl.Enablei(parameter, draw_buffer);
    }

    unsafe fn enable_vertex_array_attrib(&self, vao: Self::VertexArray, index: u32) {
        let gl = &self.raw;
        gl.EnableVertexArrayAttrib(vao.0.get(), index);
    }

    unsafe fn enable_vertex_attrib_array(&self, index: u32) {
        let gl = &self.raw;
        gl.EnableVertexAttribArray(index);
    }

    unsafe fn flush(&self) {
        let gl = &self.raw;
        gl.Flush();
    }

    unsafe fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<Self::Renderbuffer>,
    ) {
        let gl = &self.raw;
        gl.FramebufferRenderbuffer(
            target,
            attachment,
            renderbuffer_target,
            renderbuffer.map(|rb| rb.0.get()).unwrap_or(0),
        );
    }

    unsafe fn framebuffer_texture(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTexture(
            target,
            attachment,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
        );
    }

    unsafe fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTexture2D(
            target,
            attachment,
            texture_target,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
        );
    }

    unsafe fn framebuffer_texture_2d_multisample(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
        samples: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTexture2DMultisampleEXT(
            target,
            attachment,
            texture_target,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
            samples,
        );
    }

    unsafe fn framebuffer_texture_3d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTexture3D(
            target,
            attachment,
            texture_target,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
            layer,
        );
    }

    unsafe fn framebuffer_texture_layer(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTextureLayer(
            target,
            attachment,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
            layer,
        );
    }

    unsafe fn named_framebuffer_renderbuffer(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<Self::Renderbuffer>,
    ) {
        let gl = &self.raw;
        gl.NamedFramebufferRenderbuffer(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            attachment,
            renderbuffer_target,
            renderbuffer.map(|rb| rb.0.get()).unwrap_or(0),
        );
    }

    unsafe fn named_framebuffer_texture(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
    ) {
        let gl = &self.raw;
        gl.NamedFramebufferTexture(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            attachment,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
        );
    }

    unsafe fn named_framebuffer_texture_layer(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        let gl = &self.raw;
        gl.NamedFramebufferTextureLayer(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            attachment,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
            layer,
        );
    }

    unsafe fn front_face(&self, value: u32) {
        let gl = &self.raw;
        gl.FrontFace(value as u32);
    }

    unsafe fn get_error(&self) -> u32 {
        let gl = &self.raw;
        gl.GetError()
    }

    unsafe fn get_tex_parameter_i32(&self, target: u32, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetTexParameteriv(target, parameter, &mut value);
        value
    }

    unsafe fn get_tex_parameter_f32(&self, target: u32, parameter: u32) -> f32 {
        let gl = &self.raw;
        let mut value = 0.;
        gl.GetTexParameterfv(target, parameter, &mut value);
        value
    }

    unsafe fn get_texture_level_parameter_i32(
        &self,
        texture: Self::Texture,
        level: i32,
        parameter: u32,
    ) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetTextureLevelParameteriv(texture.0.get(), level, parameter, &mut value);
        value
    }

    unsafe fn get_texture_level_parameter_f32(
        &self,
        texture: Self::Texture,
        level: i32,
        parameter: u32,
    ) -> f32 {
        let gl = &self.raw;
        let mut value = 0.0;
        gl.GetTextureLevelParameterfv(texture.0.get(), level, parameter, &mut value);
        value
    }

    unsafe fn get_tex_level_parameter_i32(&self, target: u32, level: i32, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetTexLevelParameteriv(target, level, parameter, &mut value);
        value
    }

    unsafe fn get_tex_level_parameter_f32(&self, target: u32, level: i32, parameter: u32) -> f32 {
        let gl = &self.raw;
        let mut value = 0.0;
        gl.GetTexLevelParameterfv(target, level, parameter, &mut value);
        value
    }

    unsafe fn get_buffer_parameter_i32(&self, target: u32, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetBufferParameteriv(target, parameter, &mut value);
        value
    }

    unsafe fn get_parameter_bool(&self, parameter: u32) -> bool {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetBooleanv(parameter, &mut value);
        value != FALSE
    }

    unsafe fn get_parameter_bool_array<const N: usize>(&self, parameter: u32) -> [bool; N] {
        let gl = &self.raw;
        let mut value = [0; N];
        gl.GetBooleanv(parameter, &mut value[0]);
        value.map(|v| v != FALSE)
    }

    unsafe fn get_parameter_i32(&self, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetIntegerv(parameter, &mut value);
        value
    }

    unsafe fn get_parameter_i32_slice(&self, parameter: u32, out: &mut [i32]) {
        let gl = &self.raw;
        gl.GetIntegerv(parameter, &mut out[0]);
    }

    unsafe fn get_parameter_i64(&self, parameter: u32) -> i64 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetInteger64v(parameter, &mut value);
        value
    }

    unsafe fn get_parameter_i64_slice(&self, parameter: u32, out: &mut [i64]) {
        let gl = &self.raw;
        gl.GetInteger64v(parameter, &mut out[0]);
    }

    unsafe fn get_parameter_indexed_i64(&self, parameter: u32, index: u32) -> i64 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetInteger64i_v(parameter, index, &mut value);
        value
    }

    unsafe fn get_parameter_f32(&self, parameter: u32) -> f32 {
        let gl = &self.raw;
        let mut value: f32 = 0.0;
        gl.GetFloatv(parameter, &mut value);
        value
    }

    unsafe fn get_parameter_f32_slice(&self, parameter: u32, out: &mut [f32]) {
        let gl = &self.raw;
        gl.GetFloatv(parameter, &mut out[0]);
    }

    unsafe fn get_parameter_indexed_i32(&self, parameter: u32, index: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetIntegeri_v(parameter, index, &mut value);
        value
    }

    unsafe fn get_parameter_indexed_string(&self, parameter: u32, index: u32) -> String {
        let gl = &self.raw;
        let raw_ptr = gl.GetStringi(parameter, index);
        std::ffi::CStr::from_ptr(raw_ptr as *const native_gl::GLchar)
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn get_parameter_string(&self, parameter: u32) -> String {
        let gl = &self.raw;
        let raw_ptr = gl.GetString(parameter);
        if raw_ptr.is_null() {
            panic!(
                "Get parameter string 0x{:X} failed. Maybe your GL context version is too outdated.",
                parameter
            )
        }
        std::ffi::CStr::from_ptr(raw_ptr as *const native_gl::GLchar)
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn get_parameter_buffer(&self, parameter: u32) -> Option<Self::Buffer> {
        self.get_parameter_gl_name(parameter).map(NativeBuffer)
    }

    unsafe fn get_parameter_framebuffer(&self, parameter: u32) -> Option<Self::Framebuffer> {
        self.get_parameter_gl_name(parameter).map(NativeFramebuffer)
    }

    unsafe fn get_parameter_program(&self, parameter: u32) -> Option<Self::Program> {
        self.get_parameter_gl_name(parameter).map(NativeProgram)
    }

    unsafe fn get_parameter_renderbuffer(&self, parameter: u32) -> Option<Self::Renderbuffer> {
        self.get_parameter_gl_name(parameter)
            .map(NativeRenderbuffer)
    }

    unsafe fn get_parameter_sampler(&self, parameter: u32) -> Option<Self::Sampler> {
        self.get_parameter_gl_name(parameter).map(NativeSampler)
    }

    unsafe fn get_parameter_texture(&self, parameter: u32) -> Option<Self::Texture> {
        self.get_parameter_gl_name(parameter).map(NativeTexture)
    }

    unsafe fn get_parameter_transform_feedback(
        &self,
        parameter: u32,
    ) -> Option<Self::TransformFeedback> {
        self.get_parameter_gl_name(parameter)
            .map(NativeTransformFeedback)
    }

    unsafe fn get_parameter_vertex_array(&self, parameter: u32) -> Option<Self::VertexArray> {
        self.get_parameter_gl_name(parameter).map(NativeVertexArray)
    }

    unsafe fn get_renderbuffer_parameter_i32(&self, target: u32, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetRenderbufferParameteriv(target, parameter, &mut value);
        value
    }

    unsafe fn get_framebuffer_parameter_i32(&self, target: u32, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetFramebufferParameteriv(target, parameter, &mut value);
        value
    }

    unsafe fn get_named_framebuffer_parameter_i32(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        parameter: u32,
    ) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetNamedFramebufferParameteriv(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            parameter,
            &mut value,
        );
        value
    }

    unsafe fn get_framebuffer_attachment_parameter_i32(
        &self,
        target: u32,
        attachment: u32,
        parameter: u32,
    ) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetFramebufferAttachmentParameteriv(target, attachment, parameter, &mut value);
        value
    }

    unsafe fn get_named_framebuffer_attachment_parameter_i32(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        attachment: u32,
        parameter: u32,
    ) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetNamedFramebufferAttachmentParameteriv(
            framebuffer.map(|f| f.0.get()).unwrap_or(0),
            attachment,
            parameter,
            &mut value,
        );
        value
    }

    unsafe fn get_uniform_location(
        &self,
        program: Self::Program,
        name: &str,
    ) -> Option<Self::UniformLocation> {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        let uniform_location =
            gl.GetUniformLocation(program.0.get(), name.as_ptr() as *const native_gl::GLchar);
        if uniform_location < 0 {
            None
        } else {
            Some(NativeUniformLocation(uniform_location as u32))
        }
    }

    unsafe fn get_attrib_location(&self, program: Self::Program, name: &str) -> Option<u32> {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        let attrib_location =
            gl.GetAttribLocation(program.0.get(), name.as_ptr() as *const native_gl::GLchar);
        if attrib_location < 0 {
            None
        } else {
            Some(attrib_location as u32)
        }
    }

    unsafe fn bind_attrib_location(&self, program: Self::Program, index: u32, name: &str) {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        gl.BindAttribLocation(
            program.0.get(),
            index,
            name.as_ptr() as *const native_gl::GLchar,
        );
    }

    unsafe fn get_active_attributes(&self, program: Self::Program) -> u32 {
        let gl = &self.raw;
        let mut count = 0;
        gl.GetProgramiv(program.0.get(), ACTIVE_ATTRIBUTES, &mut count);
        count as u32
    }

    unsafe fn get_active_attribute(
        &self,
        program: Self::Program,
        index: u32,
    ) -> Option<ActiveAttribute> {
        let gl = &self.raw;
        let mut attribute_max_size = 0;
        gl.GetProgramiv(
            program.0.get(),
            ACTIVE_ATTRIBUTE_MAX_LENGTH,
            &mut attribute_max_size,
        );
        let mut name = String::with_capacity(attribute_max_size as usize);
        name.extend(std::iter::repeat('\0').take(attribute_max_size as usize));
        let mut length = 0;
        let mut size = 0;
        let mut atype = 0;
        gl.GetActiveAttrib(
            program.0.get(),
            index,
            attribute_max_size,
            &mut length,
            &mut size,
            &mut atype,
            name.as_ptr() as *mut native_gl::GLchar,
        );

        name.truncate(length as usize);

        Some(ActiveAttribute { name, size, atype })
    }

    unsafe fn get_sync_status(&self, fence: Self::Fence) -> u32 {
        let gl = &self.raw;
        let mut len = 0;
        let mut values = [UNSIGNALED as i32];
        gl.GetSynciv(
            fence.0,
            SYNC_STATUS,
            values.len() as i32,
            &mut len,
            values.as_mut_ptr(),
        );
        values[0] as u32
    }

    unsafe fn is_sync(&self, fence: Self::Fence) -> bool {
        let gl = &self.raw;
        1 == gl.IsSync(fence.0)
    }

    unsafe fn renderbuffer_storage(
        &self,
        target: u32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.RenderbufferStorage(target, internal_format, width, height);
    }

    unsafe fn renderbuffer_storage_multisample(
        &self,
        target: u32,
        samples: i32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.RenderbufferStorageMultisample(target, samples, internal_format, width, height);
    }

    unsafe fn sampler_parameter_f32(&self, sampler: Self::Sampler, name: u32, value: f32) {
        let gl = &self.raw;
        gl.SamplerParameterf(sampler.0.get(), name, value);
    }

    unsafe fn sampler_parameter_f32_slice(&self, sampler: Self::Sampler, name: u32, value: &[f32]) {
        let gl = &self.raw;
        gl.SamplerParameterfv(sampler.0.get(), name, value.as_ptr());
    }

    unsafe fn sampler_parameter_i32(&self, sampler: Self::Sampler, name: u32, value: i32) {
        let gl = &self.raw;
        gl.SamplerParameteri(sampler.0.get(), name, value);
    }

    unsafe fn get_sampler_parameter_i32(&self, sampler: Self::Sampler, name: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetSamplerParameteriv(sampler.0.get(), name, &mut value);
        value
    }

    unsafe fn get_sampler_parameter_f32(&self, sampler: Self::Sampler, name: u32) -> f32 {
        let gl = &self.raw;
        let mut value = 0.;
        gl.GetSamplerParameterfv(sampler.0.get(), name, &mut value);
        value
    }

    unsafe fn get_sampler_parameter_f32_slice(
        &self,
        sampler: Self::Sampler,
        name: u32,
        out: &mut [f32],
    ) {
        let gl = &self.raw;
        gl.GetSamplerParameterfv(sampler.0.get(), name, out.as_mut_ptr());
    }

    unsafe fn generate_mipmap(&self, target: u32) {
        let gl = &self.raw;
        gl.GenerateMipmap(target);
    }

    unsafe fn generate_texture_mipmap(&self, texture: Self::Texture) {
        let gl = &self.raw;
        gl.GenerateTextureMipmap(texture.0.get());
    }

    unsafe fn tex_image_1d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        let gl = &self.raw;
        gl.TexImage1D(
            target,
            level,
            internal_format,
            width,
            border,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(Some(data)) => data.as_ptr() as *const std::ffi::c_void,
                PixelUnpackData::Slice(None) => ptr::null(),
            },
        );
    }

    unsafe fn compressed_tex_image_1d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        border: i32,
        image_size: i32,
        pixels: &[u8],
    ) {
        let gl = &self.raw;
        gl.CompressedTexImage1D(
            target,
            level,
            internal_format as u32,
            width,
            border,
            image_size,
            pixels.as_ptr() as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        let gl = &self.raw;
        gl.TexImage2D(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(Some(data)) => data.as_ptr() as *const std::ffi::c_void,
                PixelUnpackData::Slice(None) => ptr::null(),
            },
        );
    }

    unsafe fn tex_image_2d_multisample(
        &self,
        target: u32,
        samples: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        fixed_sample_locations: bool,
    ) {
        let gl = &self.raw;
        gl.TexImage2DMultisample(
            target,
            samples,
            internal_format as u32,
            width,
            height,
            if fixed_sample_locations { 1 } else { 0 },
        );
    }

    unsafe fn compressed_tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        image_size: i32,
        pixels: &[u8],
    ) {
        let gl = &self.raw;
        gl.CompressedTexImage2D(
            target,
            level,
            internal_format as u32,
            width,
            height,
            border,
            image_size,
            pixels.as_ptr() as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_image_3d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        let gl = &self.raw;
        gl.TexImage3D(
            target,
            level,
            internal_format,
            width,
            height,
            depth,
            border,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(Some(data)) => data.as_ptr() as *const std::ffi::c_void,
                PixelUnpackData::Slice(None) => ptr::null(),
            },
        );
    }

    unsafe fn compressed_tex_image_3d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        image_size: i32,
        pixels: &[u8],
    ) {
        let gl = &self.raw;
        gl.CompressedTexImage3D(
            target,
            level,
            internal_format as u32,
            width,
            height,
            depth,
            border,
            image_size,
            pixels.as_ptr() as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_storage_1d(&self, target: u32, levels: i32, internal_format: u32, width: i32) {
        let gl = &self.raw;
        gl.TexStorage1D(target, levels, internal_format, width);
    }

    unsafe fn tex_storage_2d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.TexStorage2D(target, levels, internal_format, width, height);
    }

    unsafe fn texture_storage_2d(
        &self,
        texture: Self::Texture,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.TextureStorage2D(texture.0.get(), levels, internal_format, width, height);
    }

    unsafe fn tex_storage_2d_multisample(
        &self,
        target: u32,
        samples: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        fixed_sample_locations: bool,
    ) {
        let gl = &self.raw;
        gl.TexStorage2DMultisample(
            target,
            samples,
            internal_format,
            width,
            height,
            if fixed_sample_locations { 1 } else { 0 },
        );
    }

    unsafe fn tex_storage_3d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        depth: i32,
    ) {
        let gl = &self.raw;
        gl.TexStorage3D(target, levels, internal_format, width, height, depth);
    }

    unsafe fn texture_storage_3d(
        &self,
        texture: Self::Texture,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        depth: i32,
    ) {
        let gl = &self.raw;
        gl.TextureStorage3D(
            texture.0.get(),
            levels,
            internal_format,
            width,
            height,
            depth,
        );
    }

    unsafe fn get_uniform_i32(
        &self,
        program: Self::Program,
        location: &Self::UniformLocation,
        v: &mut [i32],
    ) {
        let gl = &self.raw;
        gl.GetUniformiv(program.0.get(), location.0 as i32, v.as_mut_ptr())
    }

    unsafe fn get_uniform_u32(
        &self,
        program: Self::Program,
        location: &Self::UniformLocation,
        v: &mut [u32],
    ) {
        let gl = &self.raw;
        gl.GetUniformuiv(program.0.get(), location.0 as i32, v.as_mut_ptr())
    }

    unsafe fn get_uniform_f32(
        &self,
        program: Self::Program,
        location: &Self::UniformLocation,
        v: &mut [f32],
    ) {
        let gl = &self.raw;
        gl.GetUniformfv(program.0.get(), location.0 as i32, v.as_mut_ptr())
    }

    unsafe fn uniform_1_i32(&self, location: Option<&Self::UniformLocation>, x: i32) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform1i(loc.0 as i32, x);
        }
    }

    unsafe fn uniform_2_i32(&self, location: Option<&Self::UniformLocation>, x: i32, y: i32) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform2i(loc.0 as i32, x, y);
        }
    }

    unsafe fn uniform_3_i32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform3i(loc.0 as i32, x, y, z);
        }
    }

    unsafe fn uniform_4_i32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform4i(loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn uniform_1_i32_slice(&self, location: Option<&Self::UniformLocation>, v: &[i32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform1iv(loc.0 as i32, v.len() as i32, v.as_ptr());
        }
    }

    unsafe fn uniform_2_i32_slice(&self, location: Option<&Self::UniformLocation>, v: &[i32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform2iv(loc.0 as i32, v.len() as i32 / 2, v.as_ptr());
        }
    }

    unsafe fn uniform_3_i32_slice(&self, location: Option<&Self::UniformLocation>, v: &[i32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform3iv(loc.0 as i32, v.len() as i32 / 3, v.as_ptr());
        }
    }

    unsafe fn uniform_4_i32_slice(&self, location: Option<&Self::UniformLocation>, v: &[i32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform4iv(loc.0 as i32, v.len() as i32 / 4, v.as_ptr());
        }
    }

    unsafe fn uniform_1_u32(&self, location: Option<&Self::UniformLocation>, x: u32) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform1ui(loc.0 as i32, x);
        }
    }

    unsafe fn uniform_2_u32(&self, location: Option<&Self::UniformLocation>, x: u32, y: u32) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform2ui(loc.0 as i32, x, y);
        }
    }

    unsafe fn uniform_3_u32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform3ui(loc.0 as i32, x, y, z);
        }
    }

    unsafe fn uniform_4_u32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform4ui(loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn uniform_1_u32_slice(&self, location: Option<&Self::UniformLocation>, v: &[u32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform1uiv(loc.0 as i32, v.len() as i32, v.as_ptr());
        }
    }

    unsafe fn uniform_2_u32_slice(&self, location: Option<&Self::UniformLocation>, v: &[u32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform2uiv(loc.0 as i32, v.len() as i32 / 2, v.as_ptr());
        }
    }

    unsafe fn uniform_3_u32_slice(&self, location: Option<&Self::UniformLocation>, v: &[u32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform3uiv(loc.0 as i32, v.len() as i32 / 3, v.as_ptr());
        }
    }

    unsafe fn uniform_4_u32_slice(&self, location: Option<&Self::UniformLocation>, v: &[u32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform4uiv(loc.0 as i32, v.len() as i32 / 4, v.as_ptr());
        }
    }

    unsafe fn uniform_1_f32(&self, location: Option<&Self::UniformLocation>, x: f32) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform1f(loc.0 as i32, x);
        }
    }

    unsafe fn uniform_2_f32(&self, location: Option<&Self::UniformLocation>, x: f32, y: f32) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform2f(loc.0 as i32, x, y);
        }
    }

    unsafe fn uniform_3_f32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform3f(loc.0 as i32, x, y, z);
        }
    }

    unsafe fn uniform_4_f32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform4f(loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn uniform_1_f32_slice(&self, location: Option<&Self::UniformLocation>, v: &[f32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform1fv(loc.0 as i32, v.len() as i32, v.as_ptr());
        }
    }

    unsafe fn uniform_2_f32_slice(&self, location: Option<&Self::UniformLocation>, v: &[f32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform2fv(loc.0 as i32, v.len() as i32 / 2, v.as_ptr());
        }
    }

    unsafe fn uniform_3_f32_slice(&self, location: Option<&Self::UniformLocation>, v: &[f32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform3fv(loc.0 as i32, v.len() as i32 / 3, v.as_ptr());
        }
    }

    unsafe fn uniform_4_f32_slice(&self, location: Option<&Self::UniformLocation>, v: &[f32]) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.Uniform4fv(loc.0 as i32, v.len() as i32 / 4, v.as_ptr());
        }
    }

    unsafe fn uniform_matrix_2_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix2fv(
                loc.0 as i32,
                v.len() as i32 / 4,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_2x3_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix2x3fv(
                loc.0 as i32,
                v.len() as i32 / 6,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_2x4_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix2x4fv(
                loc.0 as i32,
                v.len() as i32 / 8,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_3x2_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix3x2fv(
                loc.0 as i32,
                v.len() as i32 / 6,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_3_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix3fv(
                loc.0 as i32,
                v.len() as i32 / 9,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_3x4_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix3x4fv(
                loc.0 as i32,
                v.len() as i32 / 12,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_4x2_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix4x2fv(
                loc.0 as i32,
                v.len() as i32 / 8,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_4x3_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix4x3fv(
                loc.0 as i32,
                v.len() as i32 / 12,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_4_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        let gl = &self.raw;
        if let Some(loc) = location {
            gl.UniformMatrix4fv(
                loc.0 as i32,
                v.len() as i32 / 16,
                transpose as u8,
                v.as_ptr(),
            );
        }
    }

    unsafe fn unmap_buffer(&self, target: u32) {
        let gl = &self.raw;
        gl.UnmapBuffer(target);
    }

    unsafe fn cull_face(&self, value: u32) {
        let gl = &self.raw;
        gl.CullFace(value as u32);
    }

    unsafe fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        let gl = &self.raw;
        gl.ColorMask(red as u8, green as u8, blue as u8, alpha as u8);
    }

    unsafe fn color_mask_draw_buffer(
        &self,
        draw_buffer: u32,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    ) {
        let gl = &self.raw;
        gl.ColorMaski(draw_buffer, red as u8, green as u8, blue as u8, alpha as u8);
    }

    unsafe fn depth_mask(&self, value: bool) {
        let gl = &self.raw;
        gl.DepthMask(value as u8);
    }

    unsafe fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        let gl = &self.raw;
        gl.BlendColor(red, green, blue, alpha);
    }

    unsafe fn line_width(&self, width: f32) {
        let gl = &self.raw;
        gl.LineWidth(width);
    }

    unsafe fn map_buffer_range(
        &self,
        target: u32,
        offset: i32,
        length: i32,
        access: u32,
    ) -> *mut u8 {
        let gl = &self.raw;
        gl.MapBufferRange(target, offset as isize, length as isize, access) as *mut u8
    }

    unsafe fn flush_mapped_buffer_range(&self, target: u32, offset: i32, length: i32) {
        let gl = &self.raw;
        gl.FlushMappedBufferRange(target, offset as isize, length as isize)
    }

    unsafe fn invalidate_buffer_sub_data(&self, target: u32, offset: i32, length: i32) {
        let gl = &self.raw;
        gl.InvalidateBufferSubData(target, offset as isize, length as isize)
    }

    unsafe fn invalidate_framebuffer(&self, target: u32, attachments: &[u32]) {
        let gl = &self.raw;
        gl.InvalidateFramebuffer(target, attachments.len() as i32, attachments.as_ptr());
    }

    unsafe fn invalidate_sub_framebuffer(
        &self,
        target: u32,
        attachments: &[u32],
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.InvalidateSubFramebuffer(
            target,
            attachments.len() as i32,
            attachments.as_ptr(),
            x,
            y,
            width,
            height,
        );
    }

    unsafe fn polygon_offset(&self, factor: f32, units: f32) {
        let gl = &self.raw;
        gl.PolygonOffset(factor, units);
    }

    unsafe fn polygon_mode(&self, face: u32, mode: u32) {
        let gl = &self.raw;
        gl.PolygonMode(face as u32, mode as u32);
    }

    unsafe fn finish(&self) {
        let gl = &self.raw;
        gl.Finish();
    }

    unsafe fn bind_texture(&self, target: u32, texture: Option<Self::Texture>) {
        let gl = &self.raw;
        gl.BindTexture(target, texture.map(|t| t.0.get()).unwrap_or(0));
    }

    unsafe fn bind_texture_unit(&self, unit: u32, texture: Option<Self::Texture>) {
        let gl = &self.raw;
        gl.BindTextureUnit(unit, texture.map(|t| t.0.get()).unwrap_or(0));
    }

    unsafe fn bind_sampler(&self, unit: u32, sampler: Option<Self::Sampler>) {
        let gl = &self.raw;
        gl.BindSampler(unit, sampler.map(|s| s.0.get()).unwrap_or(0));
    }

    unsafe fn active_texture(&self, unit: u32) {
        let gl = &self.raw;
        gl.ActiveTexture(unit);
    }

    unsafe fn fence_sync(&self, condition: u32, flags: u32) -> Result<Self::Fence, String> {
        let gl = &self.raw;
        Ok(NativeFence(gl.FenceSync(condition as u32, flags)))
    }

    unsafe fn tex_parameter_f32(&self, target: u32, parameter: u32, value: f32) {
        let gl = &self.raw;
        gl.TexParameterf(target, parameter, value);
    }

    unsafe fn tex_parameter_i32(&self, target: u32, parameter: u32, value: i32) {
        let gl = &self.raw;
        gl.TexParameteri(target, parameter, value);
    }

    unsafe fn texture_parameter_i32(&self, texture: Self::Texture, parameter: u32, value: i32) {
        let gl = &self.raw;
        gl.TextureParameteri(texture.0.get(), parameter, value);
    }

    unsafe fn tex_parameter_f32_slice(&self, target: u32, parameter: u32, values: &[f32]) {
        let gl = &self.raw;
        gl.TexParameterfv(target, parameter, values.as_ptr());
    }

    unsafe fn tex_parameter_i32_slice(&self, target: u32, parameter: u32, values: &[i32]) {
        let gl = &self.raw;
        gl.TexParameteriv(target, parameter, values.as_ptr());
    }

    unsafe fn tex_sub_image_2d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        let gl = &self.raw;
        gl.TexSubImage2D(
            target,
            level,
            x_offset,
            y_offset,
            width,
            height,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(Some(data)) => data.as_ptr() as *const std::ffi::c_void,
                PixelUnpackData::Slice(None) => ptr::null(),
            },
        );
    }

    unsafe fn texture_sub_image_2d(
        &self,
        texture: Self::Texture,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        let gl = &self.raw;
        gl.TextureSubImage2D(
            texture.0.get(),
            level,
            x_offset,
            y_offset,
            width,
            height,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(Some(data)) => data.as_ptr() as *const std::ffi::c_void,
                PixelUnpackData::Slice(None) => ptr::null(),
            },
        );
    }

    unsafe fn compressed_tex_sub_image_2d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        pixels: CompressedPixelUnpackData,
    ) {
        let gl = &self.raw;
        let (data, image_size) = match pixels {
            CompressedPixelUnpackData::BufferRange(ref range) => (
                range.start as *const std::ffi::c_void,
                (range.end - range.start) as i32,
            ),
            CompressedPixelUnpackData::Slice(data) => {
                (data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
            }
        };

        gl.CompressedTexSubImage2D(
            target, level, x_offset, y_offset, width, height, format, image_size, data,
        );
    }

    unsafe fn tex_sub_image_3d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        let gl = &self.raw;
        gl.TexSubImage3D(
            target,
            level,
            x_offset,
            y_offset,
            z_offset,
            width,
            height,
            depth,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(Some(data)) => data.as_ptr() as *const std::ffi::c_void,
                PixelUnpackData::Slice(None) => ptr::null(),
            },
        );
    }

    unsafe fn texture_sub_image_3d(
        &self,
        texture: Self::Texture,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        let gl = &self.raw;
        gl.TextureSubImage3D(
            texture.0.get(),
            level,
            x_offset,
            y_offset,
            z_offset,
            width,
            height,
            depth,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(Some(data)) => data.as_ptr() as *const std::ffi::c_void,
                PixelUnpackData::Slice(None) => ptr::null(),
            },
        );
    }

    unsafe fn compressed_tex_sub_image_3d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        pixels: CompressedPixelUnpackData,
    ) {
        let gl = &self.raw;
        let (data, image_size) = match pixels {
            CompressedPixelUnpackData::BufferRange(ref range) => (
                range.start as *const std::ffi::c_void,
                (range.end - range.start) as i32,
            ),
            CompressedPixelUnpackData::Slice(data) => {
                (data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
            }
        };

        gl.CompressedTexSubImage3D(
            target, level, x_offset, y_offset, z_offset, width, height, depth, format, image_size,
            data,
        );
    }

    unsafe fn depth_func(&self, func: u32) {
        let gl = &self.raw;
        gl.DepthFunc(func as u32);
    }

    unsafe fn depth_range_f32(&self, near: f32, far: f32) {
        let gl = &self.raw;
        gl.DepthRangef(near, far);
    }

    unsafe fn depth_range_f64(&self, near: f64, far: f64) {
        let gl = &self.raw;
        gl.DepthRange(near, far);
    }

    unsafe fn depth_range(&self, near: f64, far: f64) {
        if self.supports_f64_precision() {
            self.depth_range_f64(near, far);
        } else {
            self.depth_range_f32(near as f32, far as f32);
        }
    }

    unsafe fn depth_range_f64_slice(&self, first: u32, count: i32, values: &[[f64; 2]]) {
        let gl = &self.raw;
        gl.DepthRangeArrayv(first, count, values.as_ptr() as *const f64);
    }

    unsafe fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        let gl = &self.raw;
        gl.Scissor(x, y, width, height);
    }

    unsafe fn scissor_slice(&self, first: u32, count: i32, scissors: &[[i32; 4]]) {
        let gl = &self.raw;
        gl.ScissorArrayv(first, count, scissors.as_ptr() as *const i32);
    }

    unsafe fn vertex_array_attrib_binding_f32(
        &self,
        vao: Self::VertexArray,
        index: u32,
        binding_index: u32,
    ) {
        let gl = &self.raw;
        gl.VertexArrayAttribBinding(vao.0.get(), index, binding_index);
    }

    unsafe fn vertex_array_attrib_format_f32(
        &self,
        vao: Self::VertexArray,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        relative_offset: u32,
    ) {
        let gl = &self.raw;
        gl.VertexArrayAttribFormat(
            vao.0.get(),
            index,
            size,
            data_type,
            normalized as u8,
            relative_offset,
        );
    }

    unsafe fn vertex_array_attrib_format_i32(
        &self,
        vao: Self::VertexArray,
        index: u32,
        size: i32,
        data_type: u32,
        relative_offset: u32,
    ) {
        let gl = &self.raw;
        gl.VertexArrayAttribIFormat(vao.0.get(), index, size, data_type, relative_offset);
    }

    unsafe fn vertex_array_attrib_format_f64(
        &self,
        vao: Self::VertexArray,
        index: u32,
        size: i32,
        data_type: u32,
        relative_offset: u32,
    ) {
        let gl = &self.raw;
        gl.VertexArrayAttribLFormat(vao.0.get(), index, size, data_type, relative_offset);
    }

    unsafe fn vertex_array_element_buffer(
        &self,
        vao: Self::VertexArray,
        buffer: Option<Self::Buffer>,
    ) {
        let gl = &self.raw;
        gl.VertexArrayElementBuffer(vao.0.get(), buffer.map(|b| b.0.get()).unwrap_or(0));
    }

    unsafe fn vertex_array_vertex_buffer(
        &self,
        vao: Self::VertexArray,
        binding_index: u32,
        buffer: Option<Self::Buffer>,
        offset: i32,
        stride: i32,
    ) {
        let gl = &self.raw;
        gl.VertexArrayVertexBuffer(
            vao.0.get(),
            binding_index,
            buffer.map(|b| b.0.get()).unwrap_or(0),
            offset as isize,
            stride,
        );
    }

    unsafe fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        let gl = &self.raw;
        gl.VertexAttribDivisor(index, divisor);
    }

    unsafe fn get_vertex_attrib_parameter_f32_slice(
        &self,
        index: u32,
        pname: u32,
        result: &mut [f32],
    ) {
        let gl = &self.raw;
        gl.GetVertexAttribfv(index, pname, result.as_mut_ptr());
    }

    unsafe fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribPointer(
            index,
            size,
            data_type,
            normalized as u8,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn vertex_attrib_pointer_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribIPointer(
            index,
            size,
            data_type,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn vertex_attrib_pointer_f64(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribLPointer(
            index,
            size,
            data_type,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn vertex_attrib_format_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        relative_offset: u32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribFormat(index, size, data_type, normalized as u8, relative_offset);
    }

    unsafe fn vertex_attrib_format_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        relative_offset: u32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribIFormat(index, size, data_type, relative_offset);
    }

    unsafe fn vertex_attrib_format_f64(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        relative_offset: u32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribLFormat(index, size, data_type, relative_offset);
    }

    unsafe fn vertex_attrib_1_f32(&self, index: u32, x: f32) {
        let gl = &self.raw;
        gl.VertexAttrib1f(index, x);
    }

    unsafe fn vertex_attrib_2_f32(&self, index: u32, x: f32, y: f32) {
        let gl = &self.raw;
        gl.VertexAttrib2f(index, x, y);
    }

    unsafe fn vertex_attrib_3_f32(&self, index: u32, x: f32, y: f32, z: f32) {
        let gl = &self.raw;
        gl.VertexAttrib3f(index, x, y, z);
    }

    unsafe fn vertex_attrib_4_f32(&self, index: u32, x: f32, y: f32, z: f32, w: f32) {
        let gl = &self.raw;
        gl.VertexAttrib4f(index, x, y, z, w);
    }

    unsafe fn vertex_attrib_4_i32(&self, index: u32, x: i32, y: i32, z: i32, w: i32) {
        let gl = &self.raw;
        gl.VertexAttribI4i(index, x, y, z, w);
    }

    unsafe fn vertex_attrib_4_u32(&self, index: u32, x: u32, y: u32, z: u32, w: u32) {
        let gl = &self.raw;
        gl.VertexAttribI4ui(index, x, y, z, w);
    }

    unsafe fn vertex_attrib_1_f32_slice(&self, index: u32, v: &[f32]) {
        let gl = &self.raw;
        gl.VertexAttrib1fv(index, v.as_ptr());
    }

    unsafe fn vertex_attrib_2_f32_slice(&self, index: u32, v: &[f32]) {
        let gl = &self.raw;
        gl.VertexAttrib2fv(index, v.as_ptr());
    }

    unsafe fn vertex_attrib_3_f32_slice(&self, index: u32, v: &[f32]) {
        let gl = &self.raw;
        gl.VertexAttrib3fv(index, v.as_ptr());
    }

    unsafe fn vertex_attrib_4_f32_slice(&self, index: u32, v: &[f32]) {
        let gl = &self.raw;
        gl.VertexAttrib4fv(index, v.as_ptr());
    }

    unsafe fn vertex_attrib_binding(&self, attrib_index: u32, binding_index: u32) {
        let gl = &self.raw;
        gl.VertexAttribBinding(attrib_index, binding_index);
    }

    unsafe fn vertex_binding_divisor(&self, binding_index: u32, divisor: u32) {
        let gl = &self.raw;
        gl.VertexBindingDivisor(binding_index, divisor);
    }

    unsafe fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        let gl = &self.raw;
        gl.Viewport(x, y, width, height);
    }

    unsafe fn viewport_f32_slice(&self, first: u32, count: i32, values: &[[f32; 4]]) {
        let gl = &self.raw;
        gl.ViewportArrayv(first, count, values.as_ptr() as *const f32);
    }

    unsafe fn blend_equation(&self, mode: u32) {
        let gl = &self.raw;
        gl.BlendEquation(mode as u32);
    }

    unsafe fn blend_equation_draw_buffer(&self, draw_buffer: u32, mode: u32) {
        let gl = &self.raw;
        gl.BlendEquationi(draw_buffer, mode as u32);
    }

    unsafe fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) {
        let gl = &self.raw;
        gl.BlendEquationSeparate(mode_rgb as u32, mode_alpha as u32);
    }

    unsafe fn blend_equation_separate_draw_buffer(
        &self,
        draw_buffer: u32,
        mode_rgb: u32,
        mode_alpha: u32,
    ) {
        let gl = &self.raw;
        gl.BlendEquationSeparatei(draw_buffer, mode_rgb as u32, mode_alpha as u32);
    }

    unsafe fn blend_func(&self, src: u32, dst: u32) {
        let gl = &self.raw;
        gl.BlendFunc(src as u32, dst as u32);
    }

    unsafe fn blend_func_draw_buffer(&self, draw_buffer: u32, src: u32, dst: u32) {
        let gl = &self.raw;
        gl.BlendFunci(draw_buffer, src as u32, dst as u32);
    }

    unsafe fn blend_func_separate(
        &self,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    ) {
        let gl = &self.raw;
        gl.BlendFuncSeparate(
            src_rgb as u32,
            dst_rgb as u32,
            src_alpha as u32,
            dst_alpha as u32,
        );
    }

    unsafe fn blend_func_separate_draw_buffer(
        &self,
        draw_buffer: u32,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    ) {
        let gl = &self.raw;
        gl.BlendFuncSeparatei(
            draw_buffer,
            src_rgb as u32,
            dst_rgb as u32,
            src_alpha as u32,
            dst_alpha as u32,
        );
    }

    unsafe fn stencil_func(&self, func: u32, reference: i32, mask: u32) {
        let gl = &self.raw;
        gl.StencilFunc(func as u32, reference, mask);
    }

    unsafe fn stencil_func_separate(&self, face: u32, func: u32, reference: i32, mask: u32) {
        let gl = &self.raw;
        gl.StencilFuncSeparate(face as u32, func as u32, reference, mask);
    }

    unsafe fn stencil_mask(&self, mask: u32) {
        let gl = &self.raw;
        gl.StencilMask(mask);
    }

    unsafe fn stencil_mask_separate(&self, face: u32, mask: u32) {
        let gl = &self.raw;
        gl.StencilMaskSeparate(face as u32, mask);
    }

    unsafe fn stencil_op(&self, stencil_fail: u32, depth_fail: u32, pass: u32) {
        let gl = &self.raw;
        gl.StencilOp(stencil_fail as u32, depth_fail as u32, pass as u32);
    }

    unsafe fn stencil_op_separate(&self, face: u32, stencil_fail: u32, depth_fail: u32, pass: u32) {
        let gl = &self.raw;
        gl.StencilOpSeparate(
            face as u32,
            stencil_fail as u32,
            depth_fail as u32,
            pass as u32,
        );
    }

    unsafe fn debug_message_control(
        &self,
        source: u32,
        msg_type: u32,
        severity: u32,
        ids: &[u32],
        enabled: bool,
    ) {
        let gl = &self.raw;

        let ids_ptr = if ids.is_empty() {
            std::ptr::null()
        } else {
            ids.as_ptr()
        };

        khr_fallback!(
            gl.DebugMessageControl_is_loaded,
            gl.DebugMessageControlKHR,
            gl.DebugMessageControl(
                source,
                msg_type,
                severity,
                ids.len() as i32,
                ids_ptr,
                enabled as u8,
            ),
        );
    }

    unsafe fn debug_message_insert<S>(
        &self,
        source: u32,
        msg_type: u32,
        id: u32,
        severity: u32,
        msg: S,
    ) where
        S: AsRef<str>,
    {
        let gl = &self.raw;
        let message = msg.as_ref().as_bytes();
        let length = message.len() as i32;
        khr_fallback!(
            gl.DebugMessageInsert_is_loaded,
            gl.DebugMessageInsertKHR,
            gl.DebugMessageInsert(
                source,
                msg_type,
                id,
                severity,
                length,
                message.as_ptr() as *const native_gl::GLchar,
            ),
        );
    }

    unsafe fn debug_message_callback<F>(&mut self, callback: F)
    where
        F: Fn(u32, u32, u32, u32, &str) + Send + Sync + 'static,
    {
        match self.debug_callback {
            Some(_) => {
                panic!("Debug callback already set");
            }
            None => {
                let trait_object: DebugCallback = Box::new(callback);
                let thin_ptr = Box::new(trait_object);
                let raw_ptr = Box::into_raw(thin_ptr) as *mut _ as *mut std::ffi::c_void;

                let gl = &self.raw;

                khr_fallback!(
                    gl.DebugMessageCallback_is_loaded,
                    gl.DebugMessageCallbackKHR,
                    gl.DebugMessageCallback(Some(raw_debug_message_callback), raw_ptr),
                );

                self.debug_callback = Some(DebugCallbackRawPtr { callback: raw_ptr });
            }
        }
    }

    unsafe fn get_debug_message_log(&self, count: u32) -> Vec<DebugMessageLogEntry> {
        let ct = count as usize;
        let mut sources = Vec::with_capacity(ct);
        let mut types = Vec::with_capacity(ct);
        let mut ids = Vec::with_capacity(ct);
        let mut severities = Vec::with_capacity(ct);
        let mut lengths = Vec::with_capacity(ct);
        let buf_size = (count * MAX_DEBUG_MESSAGE_LENGTH) as i32;
        let mut message_log = Vec::with_capacity(buf_size as usize);

        let gl = &self.raw;
        let received = khr_fallback!(
            gl.GetDebugMessageLog_is_loaded,
            gl.GetDebugMessageLogKHR,
            gl.GetDebugMessageLog(
                count,
                buf_size,
                sources.as_mut_ptr(),
                types.as_mut_ptr(),
                ids.as_mut_ptr(),
                severities.as_mut_ptr(),
                lengths.as_mut_ptr(),
                message_log.as_mut_ptr(),
            ),
        ) as usize;

        sources.set_len(received);
        types.set_len(received);
        ids.set_len(received);
        severities.set_len(received);
        lengths.set_len(received);
        message_log.set_len(buf_size as usize);

        let mut entries = Vec::new();
        let mut offset = 0;
        for i in 0..received {
            let message =
                std::ffi::CStr::from_ptr(message_log[offset..].as_ptr()).to_string_lossy();
            offset += lengths[i] as usize;
            entries.push(DebugMessageLogEntry {
                source: sources[i],
                msg_type: types[i],
                id: ids[i],
                severity: severities[i],
                message: message.to_string(),
            });
        }

        entries
    }

    unsafe fn push_debug_group<S>(&self, source: u32, id: u32, message: S)
    where
        S: AsRef<str>,
    {
        let gl = &self.raw;
        let msg = message.as_ref().as_bytes();
        let length = msg.len() as i32;
        khr_fallback!(
            gl.PushDebugGroup_is_loaded,
            gl.PushDebugGroupKHR,
            gl.PushDebugGroup(source, id, length, msg.as_ptr() as *const native_gl::GLchar),
        );
    }

    unsafe fn pop_debug_group(&self) {
        let gl = &self.raw;
        khr_fallback!(
            gl.PopDebugGroup_is_loaded,
            gl.PopDebugGroupKHR,
            gl.PopDebugGroup(),
        );
    }

    unsafe fn object_label<S>(&self, identifier: u32, name: u32, label: Option<S>)
    where
        S: AsRef<str>,
    {
        let gl = &self.raw;

        match label {
            Some(l) => {
                let lbl = l.as_ref().as_bytes();
                let length = lbl.len() as i32;
                khr_fallback!(
                    gl.ObjectLabel_is_loaded,
                    gl.ObjectLabelKHR,
                    gl.ObjectLabel(
                        identifier,
                        name,
                        length,
                        lbl.as_ptr() as *const native_gl::GLchar,
                    ),
                );
            }
            None => {
                khr_fallback!(
                    gl.ObjectLabel_is_loaded,
                    gl.ObjectLabelKHR,
                    gl.ObjectLabel(identifier, name, 0, std::ptr::null()),
                );
            }
        }
    }

    unsafe fn get_object_label(&self, identifier: u32, name: u32) -> String {
        let gl = &self.raw;
        let mut len = 0;
        let mut label_buf = Vec::with_capacity(self.constants.max_label_length as usize);
        khr_fallback!(
            gl.GetObjectLabel_is_loaded,
            gl.GetObjectLabelKHR,
            gl.GetObjectLabel(
                identifier,
                name,
                self.constants.max_label_length,
                &mut len,
                label_buf.as_mut_ptr(),
            ),
        );
        label_buf.set_len(len as usize);
        std::ffi::CStr::from_ptr(label_buf.as_ptr())
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn object_ptr_label<S>(&self, sync: Self::Fence, label: Option<S>)
    where
        S: AsRef<str>,
    {
        let gl = &self.raw;

        match label {
            Some(l) => {
                let lbl = l.as_ref().as_bytes();
                let length = lbl.len() as i32;
                khr_fallback!(
                    gl.ObjectPtrLabel_is_loaded,
                    gl.ObjectPtrLabelKHR,
                    gl.ObjectPtrLabel(
                        sync.0 as *mut std::ffi::c_void,
                        length,
                        lbl.as_ptr() as *const native_gl::GLchar,
                    ),
                );
            }
            None => {
                khr_fallback!(
                    gl.ObjectPtrLabel_is_loaded,
                    gl.ObjectPtrLabelKHR,
                    gl.ObjectPtrLabel(sync.0 as *mut std::ffi::c_void, 0, std::ptr::null()),
                );
            }
        }
    }

    unsafe fn get_object_ptr_label(&self, sync: Self::Fence) -> String {
        let gl = &self.raw;
        let mut len = 0;
        let mut label_buf = Vec::with_capacity(self.constants.max_label_length as usize);
        khr_fallback!(
            gl.GetObjectPtrLabel_is_loaded,
            gl.GetObjectPtrLabelKHR,
            gl.GetObjectPtrLabel(
                sync.0 as *mut std::ffi::c_void,
                self.constants.max_label_length,
                &mut len,
                label_buf.as_mut_ptr(),
            ),
        );
        label_buf.set_len(len as usize);
        std::ffi::CStr::from_ptr(label_buf.as_ptr())
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn get_uniform_block_index(&self, program: Self::Program, name: &str) -> Option<u32> {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        let index = gl.GetUniformBlockIndex(program.0.get(), name.as_ptr());
        if index == INVALID_INDEX {
            None
        } else {
            Some(index)
        }
    }

    unsafe fn get_uniform_indices(
        &self,
        program: Self::Program,
        names: &[&str],
    ) -> Vec<Option<u32>> {
        let gl = &self.raw;
        let c_names = names
            .iter()
            .map(|&name| CString::new(name).unwrap())
            .collect::<Vec<_>>();
        let c_name_ptrs = c_names.iter().map(|name| name.as_ptr()).collect::<Vec<_>>();

        let count = names.len();
        let mut indices = vec![0; count];
        gl.GetUniformIndices(
            program.0.get(),
            count as _,
            c_name_ptrs.as_ptr(),
            indices.as_mut_ptr(),
        );
        indices
            .iter()
            .map(|&index| {
                if index == INVALID_INDEX {
                    None
                } else {
                    Some(index)
                }
            })
            .collect()
    }

    unsafe fn uniform_block_binding(&self, program: Self::Program, index: u32, binding: u32) {
        let gl = &self.raw;
        gl.UniformBlockBinding(program.0.get(), index, binding);
    }

    unsafe fn get_shader_storage_block_index(
        &self,
        program: Self::Program,
        name: &str,
    ) -> Option<u32> {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        let index =
            gl.GetProgramResourceIndex(program.0.get(), SHADER_STORAGE_BLOCK, name.as_ptr());
        if index == INVALID_INDEX {
            None
        } else {
            Some(index)
        }
    }

    unsafe fn shader_storage_block_binding(
        &self,
        program: Self::Program,
        index: u32,
        binding: u32,
    ) {
        let gl = &self.raw;
        gl.ShaderStorageBlockBinding(program.0.get(), index, binding);
    }

    unsafe fn read_buffer(&self, src: u32) {
        let gl = &self.raw;
        gl.ReadBuffer(src);
    }

    unsafe fn named_framebuffer_read_buffer(
        &self,
        framebuffer: Option<Self::Framebuffer>,
        src: u32,
    ) {
        let gl = &self.raw;
        gl.NamedFramebufferReadBuffer(framebuffer.map(|f| f.0.get()).unwrap_or(0), src);
    }

    unsafe fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        gltype: u32,
        pixels: PixelPackData,
    ) {
        let gl = &self.raw;
        gl.ReadPixels(
            x,
            y,
            width,
            height,
            format,
            gltype,
            match pixels {
                PixelPackData::BufferOffset(offset) => offset as *mut std::ffi::c_void,
                PixelPackData::Slice(Some(data)) => data.as_mut_ptr() as *mut std::ffi::c_void,
                PixelPackData::Slice(None) => ptr::null_mut(),
            },
        );
    }

    unsafe fn begin_query(&self, target: u32, query: Self::Query) {
        let gl = &self.raw;
        if gl.BeginQuery_is_loaded() {
            gl.BeginQuery(target, query.0.get());
        } else {
            gl.BeginQueryEXT(target, query.0.get());
        }
    }

    unsafe fn end_query(&self, target: u32) {
        let gl = &self.raw;
        if gl.EndQuery_is_loaded() {
            gl.EndQuery(target);
        } else {
            gl.EndQueryEXT(target);
        }
    }

    unsafe fn query_counter(&self, query: Self::Query, target: u32) {
        let gl = &self.raw;
        if gl.QueryCounter_is_loaded() {
            gl.QueryCounter(query.0.get(), target);
        } else {
            gl.QueryCounterEXT(query.0.get(), target);
        }
    }

    unsafe fn get_query_parameter_u32(&self, query: Self::Query, parameter: u32) -> u32 {
        let gl = &self.raw;
        let mut value = 0;
        if gl.GetQueryBufferObjectiv_is_loaded() {
            gl.GetQueryObjectuiv(query.0.get(), parameter, &mut value);
        } else {
            gl.GetQueryObjectuivEXT(query.0.get(), parameter, &mut value);
        }
        value
    }

    unsafe fn get_query_parameter_u64(&self, query: Self::Query, parameter: u32) -> u64 {
        let gl = &self.raw;
        let mut value = 0;
        if gl.GetQueryBufferObjectiv_is_loaded() {
            gl.GetQueryObjectui64v(query.0.get(), parameter, &mut value);
        } else {
            gl.GetQueryObjectui64vEXT(query.0.get(), parameter, &mut value);
        }
        value
    }

    unsafe fn get_query_parameter_u64_with_offset(
        &self,
        query: Self::Query,
        parameter: u32,
        offset: usize,
    ) {
        let gl = &self.raw;
        if gl.GetQueryObjectui64v_is_loaded() {
            gl.GetQueryObjectui64v(query.0.get(), parameter, offset as *mut _);
        } else {
            gl.GetQueryObjectui64vEXT(query.0.get(), parameter, offset as *mut _);
        }
    }

    unsafe fn create_transform_feedback(&self) -> Result<Self::TransformFeedback, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenTransformFeedbacks(1, &mut name);
        NonZeroU32::new(name)
            .map(NativeTransformFeedback)
            .ok_or_else(|| String::from("Unable to create TransformFeedback object"))
    }

    unsafe fn is_transform_feedback(&self, transform_feedback: Self::TransformFeedback) -> bool {
        let gl = &self.raw;
        gl.IsTransformFeedback(transform_feedback.0.get()) != 0
    }

    unsafe fn delete_transform_feedback(&self, transform_feedback: Self::TransformFeedback) {
        let gl = &self.raw;
        gl.DeleteTransformFeedbacks(1, &transform_feedback.0.get());
    }

    unsafe fn bind_transform_feedback(
        &self,
        target: u32,
        transform_feedback: Option<Self::TransformFeedback>,
    ) {
        let gl = &self.raw;
        gl.BindTransformFeedback(target, transform_feedback.map(|tf| tf.0.get()).unwrap_or(0));
    }

    unsafe fn begin_transform_feedback(&self, primitive_mode: u32) {
        let gl = &self.raw;
        gl.BeginTransformFeedback(primitive_mode);
    }

    unsafe fn end_transform_feedback(&self) {
        let gl = &self.raw;
        gl.EndTransformFeedback();
    }

    unsafe fn pause_transform_feedback(&self) {
        let gl = &self.raw;
        gl.PauseTransformFeedback();
    }

    unsafe fn resume_transform_feedback(&self) {
        let gl = &self.raw;
        gl.ResumeTransformFeedback();
    }

    unsafe fn transform_feedback_varyings(
        &self,
        program: Self::Program,
        varyings: &[&str],
        buffer_mode: u32,
    ) {
        let gl = &self.raw;

        let strings: Vec<CString> = varyings
            .iter()
            .copied()
            .map(CString::new)
            .collect::<Result<_, _>>()
            .unwrap();
        let varyings: Vec<_> = strings.iter().map(|c_str| c_str.as_ptr()).collect();

        gl.TransformFeedbackVaryings(
            program.0.get(),
            varyings.len() as i32,
            varyings.as_ptr(),
            buffer_mode,
        );
    }

    unsafe fn get_transform_feedback_varying(
        &self,
        program: Self::Program,
        index: u32,
    ) -> Option<ActiveTransformFeedback> {
        let gl = &self.raw;

        const max_name_size: usize = 256;
        let mut name_bytes = [0; max_name_size];

        let mut size = 0;
        let mut tftype = 0;

        gl.GetTransformFeedbackVarying(
            program.0.get(),
            index,
            name_bytes.len() as i32,
            std::ptr::null_mut(),
            &mut size,
            &mut tftype,
            name_bytes.as_mut_ptr(),
        );

        let name = CStr::from_ptr(name_bytes.as_mut_ptr())
            .to_string_lossy()
            .into_owned();

        Some(ActiveTransformFeedback { size, tftype, name })
    }

    unsafe fn memory_barrier(&self, barriers: u32) {
        let gl = &self.raw;
        gl.MemoryBarrier(barriers);
    }

    unsafe fn memory_barrier_by_region(&self, barriers: u32) {
        let gl = &self.raw;
        gl.MemoryBarrierByRegion(barriers);
    }

    unsafe fn bind_image_texture(
        &self,
        unit: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layered: bool,
        layer: i32,
        access: u32,
        format: u32,
    ) {
        let gl = &self.raw;
        gl.BindImageTexture(
            unit,
            texture.map(|tex| tex.0.get()).unwrap_or(0),
            level,
            layered as u8,
            layer,
            access,
            format,
        );
    }
    unsafe fn get_active_uniform_block_parameter_i32(
        &self,
        program: Self::Program,
        uniform_block_index: u32,
        parameter: u32,
    ) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetActiveUniformBlockiv(program.0.get(), uniform_block_index, parameter, &mut value);
        value
    }

    unsafe fn get_active_uniform_block_parameter_i32_slice(
        &self,
        program: Self::Program,
        uniform_block_index: u32,
        parameter: u32,
        out: &mut [i32],
    ) {
        let gl = &self.raw;
        gl.GetActiveUniformBlockiv(
            program.0.get(),
            uniform_block_index,
            parameter,
            out.as_mut_ptr(),
        );
    }
    unsafe fn get_active_uniform_block_name(
        &self,
        program: Self::Program,
        uniform_block_index: u32,
    ) -> String {
        let gl = &self.raw;

        // Probe for the length of the name of the uniform block, and, failing
        // that, fall back to allocating a buffer that is 256 bytes long. This
        // should be good enough for pretty much all contexts, including faulty
        // or partially faulty ones.
        let len = self.get_active_uniform_block_parameter_i32(
            program,
            uniform_block_index,
            crate::UNIFORM_BLOCK_NAME_LENGTH,
        );
        let len = if gl.GetError() == crate::NO_ERROR && len > 0 {
            len as usize
        } else {
            256
        };

        let mut buffer = vec![0; len];
        let mut length = 0;
        gl.GetActiveUniformBlockName(
            program.0.get(),
            uniform_block_index,
            buffer.len() as _,
            &mut length,
            buffer.as_mut_ptr(),
        );

        if length > 0 {
            assert_eq!(
                std::mem::size_of::<u8>(),
                std::mem::size_of::<native_gl::GLchar>(),
                "This operation is only safe in systems in which the length of \
                a GLchar is the same as that of an u8"
            );
            assert_eq!(
                std::mem::align_of::<u8>(),
                std::mem::align_of::<native_gl::GLchar>(),
                "This operation is only safe in systems in which the alignment \
                of a GLchar is the same as that of an u8"
            );
            let buffer = std::slice::from_raw_parts(
                buffer.as_ptr() as *const u8,
                (length as usize + 1).min(buffer.len()),
            );

            let name = CStr::from_bytes_with_nul(&buffer[..])
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();

            name
        } else {
            String::from("")
        }
    }

    unsafe fn max_shader_compiler_threads(&self, count: u32) {
        let gl = &self.raw;
        if gl.MaxShaderCompilerThreadsKHR_is_loaded() {
            gl.MaxShaderCompilerThreadsKHR(count);
        } else {
            gl.MaxShaderCompilerThreadsARB(count);
        }
    }

    unsafe fn hint(&self, target: u32, mode: u32) {
        let gl = &self.raw;
        gl.Hint(target, mode);
    }

    unsafe fn sample_coverage(&self, value: f32, invert: bool) {
        let gl = &self.raw;
        gl.SampleCoverage(value, invert as u8);
    }

    unsafe fn get_internal_format_i32_slice(
        &self,
        target: u32,
        internal_format: u32,
        pname: u32,
        result: &mut [i32],
    ) {
        let gl = &self.raw;
        gl.GetInternalformativ(
            target,
            internal_format,
            pname,
            result.len() as _,
            result.as_mut_ptr(),
        )
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        match self.debug_callback.take() {
            Some(_) => {
                // Unset the debug callback before destroying the context.
                unsafe {
                    let gl = &self.raw;
                    if gl.DebugMessageCallback_is_loaded() {
                        gl.DebugMessageCallback(None, std::ptr::null());
                    } else {
                        gl.DebugMessageCallbackKHR(None, std::ptr::null());
                    }
                }
            }
            None => {}
        }
    }
}

extern "system" fn raw_debug_message_callback(
    source: u32,
    gltype: u32,
    id: u32,
    severity: u32,
    length: i32,
    message: *const native_gl::GLchar,
    user_param: *mut std::ffi::c_void,
) {
    let _result = std::panic::catch_unwind(move || unsafe {
        let callback: &DebugCallback = &*(user_param as *const DebugCallback);
        let slice = std::slice::from_raw_parts(message as *const u8, length as usize);
        let msg = String::from_utf8_lossy(slice);
        (callback)(source, gltype, id, severity, &msg);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Context>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Context>();
    }
}
