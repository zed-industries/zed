use alloc::{borrow::ToOwned, ffi::CString, format};
use core::ffi::CStr;
use ttf_parser::{GlyphId, Tag};
use wasmi::{self, AsContextMut, Caller, Config, Engine, Linker, Module, Store};

use super::{
    buffer::{hb_buffer_t, GlyphPosition},
    face::hb_glyph_extents_t,
    hb_font_t, hb_glyph_info_t,
    ot_shape::{hb_ot_shape_context_t, shape_internal},
    ot_shape_plan::hb_ot_shape_plan_t,
};

struct ShapingData<'a> {
    font: &'a hb_font_t<'a>,
    plan: &'a hb_ot_shape_plan_t,
    buffer: &'a mut hb_buffer_t,
}

pub(crate) fn shape_with_wasm(
    font: &hb_font_t,
    plan: &hb_ot_shape_plan_t,
    buffer: &mut hb_buffer_t,
) -> Option<()> {
    // If font has no Wasm blob just return None to carry on as usual.
    let wasm_blob = font
        .raw_face()
        .table(ttf_parser::Tag::from_bytes(b"Wasm"))?;

    let mut config = Config::default();
    config.compilation_mode(wasmi::CompilationMode::Lazy);
    let engine = Engine::new(&config);

    let module = Module::new(&engine, wasm_blob).ok()?;

    let mut linker = Linker::new(&engine);

    // Not every function defined by HarfBuzz is defined here.
    // Only the ones used by the harfbuzz_wasm crate
    //
    // For more info see
    // "Spec": https://github.com/harfbuzz/harfbuzz/blob/main/docs/wasm-shaper.md
    // crate: https://github.com/harfbuzz/harfbuzz-wasm-examples/blob/main/harfbuzz-wasm/src/lib.rs
    linker
        .func_wrap("env", "face_get_upem", face_get_upem)
        .ok()?
        .func_wrap("env", "font_get_face", font_get_face)
        .ok()?
        .func_wrap("env", "font_get_glyph", font_get_glyph)
        .ok()?
        .func_wrap("env", "font_get_scale", font_get_scale)
        .ok()?
        .func_wrap("env", "font_get_glyph_extents", font_get_glyph_extents)
        .ok()?
        .func_wrap("env", "font_glyph_to_string", font_glyph_to_string)
        .ok()?
        .func_wrap("env", "font_get_glyph_h_advance", font_get_glyph_h_advance)
        .ok()?
        .func_wrap("env", "font_get_glyph_v_advance", font_get_glyph_v_advance)
        .ok()?
        .func_wrap("env", "font_copy_glyph_outline", font_copy_glyph_outline)
        .ok()?
        .func_wrap("env", "face_copy_table", face_copy_table)
        .ok()?
        .func_wrap("env", "buffer_copy_contents", buffer_copy_contents)
        .ok()?
        .func_wrap("env", "buffer_set_contents", buffer_set_contents)
        .ok()?
        .func_wrap("env", "debugprint", debugprint)
        .ok()?
        .func_wrap("env", "shape_with", shape_with)
        .ok()?;

    let data = ShapingData { font, plan, buffer };
    let mut store = Store::new(&engine, data);

    let instance = linker
        .instantiate(&mut store, &module)
        .ok()?
        .start(&mut store)
        .ok()?;

    // return early if no "memory" or "shape" exports.
    instance.get_memory(&mut store, "memory")?;
    let shape = instance
        .get_typed_func::<(u32, u32, u32, u32, u32), i32>(&mut store, "shape")
        .ok()?;

    match shape.call(&mut store, (0, 0, 0, 0, 0)) {
        Ok(0) => {
            log::info!("Wasm Shaper return with failure.");
            return None;
        }
        Err(e) => {
            log::error!("Wasm Module Error: {e}");
            return None;
        }
        _ => (),
    }

    Some(())
}

// Definition in comments in the definition in harfbuzz_wasm crate.

// fn face_get_upem(face: u32) -> u32;
// Returns the units-per-em of the font face.
fn face_get_upem(caller: Caller<'_, ShapingData>, _face: u32) -> u32 {
    caller.data().font.units_per_em as u32
}

// fn font_get_face(font: u32) -> u32;
// Creates a new face token from the given font token.
fn font_get_face(_caller: Caller<'_, ShapingData>, _font: u32) -> u32 {
    // Nothing to do here. We do not use face tokens.
    0
}

// fn font_get_glyph(font: u32, unicode: u32, uvs: u32) -> u32;
// Returns the nominal glyph ID for the given codepoint, using the cmap table of the font to map Unicode codepoint (and variation selector) to glyph ID.
fn font_get_glyph(caller: Caller<'_, ShapingData>, _font: u32, codepoint: u32, uvs: u32) -> u32 {
    let Some(codepoint) = char::from_u32(codepoint) else {
        return 0;
    };

    match (uvs, char::from_u32(uvs)) {
        (0, _) | (_, None) => caller.data().font.glyph_index(codepoint),
        (_, Some(uvs)) => caller.data().font.glyph_variation_index(codepoint, uvs),
    }
    .unwrap_or_default()
    .0 as u32
}

// fn font_get_scale(font: u32, x_scale: *mut i32, y_scale: *mut i32);
// Returns the scale of the current font.
fn font_get_scale(mut caller: Caller<'_, ShapingData>, _font: u32, x_scale: u32, y_scale: u32) {
    // Return upem as rustybuzz has no scale.
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let upem = caller.data().font.units_per_em();

    _ = memory.write(
        &mut caller.as_context_mut(),
        x_scale as usize,
        &upem.to_le_bytes(),
    );
    _ = memory.write(
        &mut caller.as_context_mut(),
        y_scale as usize,
        &upem.to_le_bytes(),
    );
}

// fn font_get_glyph_extents(font: u32, glyph: u32, extents: *mut CGlyphExtents) -> bool;
// Returns the glyph's extents for the given glyph ID at current scale and variation settings.
fn font_get_glyph_extents(
    mut caller: Caller<'_, ShapingData>,
    _font: u32,
    glyph: u32,
    extents: u32,
) -> u32 {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let mut glyph_extents = hb_glyph_extents_t::default();
    let ret = caller
        .data()
        .font
        .glyph_extents(GlyphId(glyph as u16), &mut glyph_extents);
    if ret {
        _ = memory.write(
            caller.as_context_mut(),
            extents as usize,
            bytemuck::bytes_of(&glyph_extents),
        );
    }

    ret as u32
}

// fn font_glyph_to_string(font: u32, glyph: u32, str: *const u8, len: u32);
// Copies the name of the given glyph, or, if no name is available, a string of the form gXXXX into the given string.
fn font_glyph_to_string(
    mut caller: Caller<'_, ShapingData>,
    _font: u32,
    glyph: u32,
    str: u32,
    len: u32,
) {
    // len is the assigned heap memory. We should not allocate more than that.
    // Should not assume we are not writing over anything.
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let mut name = caller
        .data()
        .font
        .glyph_name(GlyphId(glyph as u16))
        .map(ToOwned::to_owned)
        .unwrap_or(format!("g{:0>4}", glyph));
    name.truncate(len as usize - 1);
    let name = CString::new(name).unwrap();
    let name = name.as_bytes_with_nul();

    _ = memory.write(caller.as_context_mut(), str as usize, name);
}

// fn font_get_glyph_h_advance(font: u32, glyph: u32) -> i32;
// Returns the default horizontal advance for the given glyph ID the current scale and variations settings.
fn font_get_glyph_h_advance(caller: Caller<'_, ShapingData>, _font: u32, glyph: u32) -> i32 {
    caller.data().font.glyph_h_advance(GlyphId(glyph as u16))
}

// fn font_get_glyph_v_advance(font: u32, glyph: u32) -> i32;
// Returns the default vertical advance for the given glyph ID the current scale and variations settings.
fn font_get_glyph_v_advance(caller: Caller<'_, ShapingData>, _font: u32, glyph: u32) -> i32 {
    caller.data().font.glyph_v_advance(GlyphId(glyph as u16))
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
enum PointType {
    MoveTo,
    LineTo,
    QuadraticTo,
    CubicTo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct OutlinePoint {
    x: f32,
    y: f32,
    kind: PointType,
}
impl OutlinePoint {
    fn new(x: f32, y: f32, kind: PointType) -> Self {
        OutlinePoint { x, y, kind }
    }
}

unsafe impl bytemuck::Zeroable for OutlinePoint {}
unsafe impl bytemuck::Pod for OutlinePoint {}

#[derive(Default)]
struct GlyphOutline {
    points: alloc::vec::Vec<OutlinePoint>,
    contours: alloc::vec::Vec<u32>,
}

impl ttf_parser::OutlineBuilder for GlyphOutline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.points.push(OutlinePoint::new(x, y, PointType::MoveTo))
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.points.push(OutlinePoint::new(x, y, PointType::LineTo))
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.points.extend_from_slice(
            [(x1, y1), (x, y)]
                .map(|(x, y)| OutlinePoint::new(x, y, PointType::QuadraticTo))
                .as_slice(),
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.points.extend_from_slice(
            [(x1, y1), (x2, y2), (x, y)]
                .map(|(x, y)| OutlinePoint::new(x, y, PointType::CubicTo))
                .as_slice(),
        );
    }

    fn close(&mut self) {
        // harfbuzz_wasm crate expects these points to be
        // the index of the point after the end of countour.
        // so it is definitely len.
        self.contours.push(self.points.len() as u32)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CGlyphOutline {
    n_points: u32,
    points: u32, // pointer
    n_contours: u32,
    contours: u32, // pointer
}

unsafe impl bytemuck::Zeroable for CGlyphOutline {}
unsafe impl bytemuck::Pod for CGlyphOutline {}

// fn font_copy_glyph_outline(font: u32, glyph: u32, outline: *mut CGlyphOutline) -> bool;
// Copies the outline of the given glyph ID, at current scale and variation settings, into the outline structure provided.
fn font_copy_glyph_outline(
    mut caller: Caller<'_, ShapingData>,
    _font: u32,
    glyph: u32,
    outline: u32,
) -> u32 {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let mut builder = GlyphOutline::default();
    if caller
        .data()
        .font
        .outline_glyph(GlyphId(glyph as u16), &mut builder)
        .is_none()
    {
        return 0;
    };

    let points_size = builder.points.len() * core::mem::size_of::<OutlinePoint>();
    let contours_size = builder.contours.len() * core::mem::size_of::<u32>();
    let needed_size = points_size + contours_size;
    // 1 page is 65536 or 0x10000 bytes.
    let page_growth_needed = needed_size / 0x10000 + 1;

    let eom = memory.data(&caller).len();
    if memory
        .grow(&mut caller.as_context_mut(), page_growth_needed as u32)
        .is_err()
    {
        return 0;
    };

    let mem_data = memory.data_mut(&mut caller);

    mem_data[eom..eom + points_size].copy_from_slice(bytemuck::cast_slice(&builder.points));
    mem_data[eom + points_size..eom + needed_size]
        .copy_from_slice(bytemuck::cast_slice(&builder.contours));

    let builder = CGlyphOutline {
        n_points: builder.points.len() as u32,
        points: eom as u32,
        n_contours: builder.contours.len() as u32,
        contours: (eom + points_size) as u32,
    };

    if memory
        .write(
            caller.as_context_mut(),
            outline as usize,
            bytemuck::bytes_of(&builder),
        )
        .is_err()
    {
        return 0;
    };

    1
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Blob {
    // Length of the blob in bytes
    length: u32,
    data: u32, // pointer
}

unsafe impl bytemuck::Zeroable for Blob {}
unsafe impl bytemuck::Pod for Blob {}

// fn face_copy_table(font: u32, tag: u32, blob: *mut Blob) -> bool;
// Copies the binary data in the OpenType table referenced by tag into the supplied blob structure.
fn face_copy_table(mut caller: Caller<'_, ShapingData>, _font: u32, tag: u32, blob: u32) -> u32 {
    // So here to copy stuff INTO the module, we need to copy it into its heap
    // We should not assume that there is an area that's not written to,
    // so the most straightforward way to get "clean" memory is to grow it by one page,
    // and allocate there. This is not idiomatic to either Rust or Wasm but it is the
    // best we can do with the given API.

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let tag = tag.to_be_bytes();
    let Some(table) = caller.data().font.raw_face().table(Tag::from_bytes(&tag)) else {
        return 0;
    };

    // 1 page is 65536 or 0x10000 bytes.
    let page_growth_needed = table.len() / 0x10000 + 1;

    let eom = memory.data_size(&caller);
    if memory
        .grow(&mut caller.as_context_mut(), page_growth_needed as u32)
        .is_err()
    {
        return 0;
    };

    let ret = Blob {
        length: table.len() as u32,
        data: eom as u32,
    };

    if memory
        .write(
            caller.as_context_mut(),
            blob as usize,
            bytemuck::bytes_of(&ret),
        )
        .is_err()
    {
        return 0;
    };

    if memory.write(caller.as_context_mut(), eom, table).is_err() {
        return 0;
    };

    1
}

// fn buffer_copy_contents(buffer: u32, cbuffer: *mut CBufferContents) -> bool;
// Retrieves the contents of the host shaping engine's buffer into the buffer_contents structure. This should typically be called at the beginning of shaping.
fn buffer_copy_contents(mut caller: Caller<'_, ShapingData>, _buffer: u32, cbuffer: u32) -> u32 {
    // see face_copy_table for why we're growing memory.
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let length = caller.data().buffer.len;

    // 1 page is 65536 or 0x10000 bytes.
    let char_size = core::mem::size_of::<GlyphPosition>() + core::mem::size_of::<hb_glyph_info_t>();
    let page_growth_needed = length * char_size / 0x10000 + 1;

    let eom = memory.data(&caller).len();
    if memory
        .grow(&mut caller.as_context_mut(), page_growth_needed as u32)
        .is_err()
    {
        return 0;
    };

    let (mem_data, store_data) = memory.data_and_store_mut(&mut caller);

    let rb_buffer = &store_data.buffer;

    let pos_loc = eom + length * core::mem::size_of::<hb_glyph_info_t>();
    let end_loc = pos_loc + length * core::mem::size_of::<GlyphPosition>();

    mem_data[eom..pos_loc].copy_from_slice(bytemuck::cast_slice(&rb_buffer.info));
    mem_data[pos_loc..end_loc].copy_from_slice(bytemuck::cast_slice(&rb_buffer.pos));

    let buffer_contents = CBufferContents {
        length: length as u32,
        info: eom as u32,
        position: pos_loc as u32,
    };
    if memory
        .write(
            &mut caller.as_context_mut(),
            cbuffer as usize,
            bytemuck::bytes_of(&buffer_contents),
        )
        .is_err()
    {
        return 0;
    };

    1
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CBufferContents {
    length: u32,
    info: u32,     // pointer
    position: u32, // pointer
}

unsafe impl bytemuck::Zeroable for CBufferContents {}
unsafe impl bytemuck::Pod for CBufferContents {}

// fn buffer_set_contents(buffer: u32, cbuffer: &CBufferContents) -> bool;
// Copy the buffer_contents structure back into the host shaping engine's buffer. This should typically be called at the end of shaping.
fn buffer_set_contents(mut caller: Caller<'_, ShapingData>, _buffer: u32, cbuffer: u32) -> u32 {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let mut buffer = [0; core::mem::size_of::<CBufferContents>()];
    if memory
        .read(caller.as_context_mut(), cbuffer as usize, &mut buffer)
        .is_err()
    {
        return 0;
    };
    let Ok(buffer) = bytemuck::try_from_bytes::<CBufferContents>(&buffer) else {
        return 0;
    };

    let (mem_data, store_data) = memory.data_and_store_mut(&mut caller);

    let array_length = buffer.length as usize * core::mem::size_of::<hb_glyph_info_t>();

    store_data.buffer.len = buffer.length as usize;

    store_data.buffer.info.clear();
    store_data
        .buffer
        .info
        .extend_from_slice(bytemuck::cast_slice(
            &mem_data[buffer.info as usize..buffer.info as usize + array_length],
        ));

    store_data.buffer.pos.clear();
    store_data
        .buffer
        .pos
        .extend_from_slice(bytemuck::cast_slice(
            &mem_data[buffer.position as usize..buffer.position as usize + array_length],
        ));

    1
}

// fn debugprint(s: *const u8);
// Produces a debugging message in the host shaper's log output;
fn debugprint(caller: Caller<'_, ShapingData>, s: u32) {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    if memory.data(&caller).get(s as usize).is_none() {
        return;
    }
    let bytes = &memory.data(&caller)[s as usize..];
    let msg = CStr::from_bytes_until_nul(bytes)
        .unwrap_or_default()
        .to_string_lossy();

    log::debug!("Wasm Module: {msg}");
}

// fn shape_with(font: u32, buffer: u32, features: u32, num_features: u32, shaper: *const u8) -> i32;
// Run another shaping engine's shaping process on the given font and buffer. The only shaping engine guaranteed to be available is ot, the OpenType shaper, but others may also be available. This allows the WASM author to process a buffer "normally", before further manipulating it.
fn shape_with(
    mut caller: Caller<'_, ShapingData>,
    // We don't use font token and buffer token
    _font: u32,
    _buffer: u32,
    // harfbuzz-wasm doesn't use the features pointer
    _features: u32,
    _num_features: u32,
    // we dont have custom shapers (yet?).
    shaper: u32,
) -> i32 {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    if memory.data(&caller).get(shaper as usize).is_none() {
        return 0;
    }
    let bytes = &memory.data(&caller)[shaper as usize..];
    let shaper = CStr::from_bytes_until_nul(bytes)
        .unwrap_or_default()
        .to_string_lossy();

    if !(shaper.eq_ignore_ascii_case("ot") || shaper.eq_ignore_ascii_case("rustybuzz")) {
        log::warn!("Only ot shaper is available in rustybuzz.");
        return 0;
    }

    let face = caller.data().font;
    let plan = caller.data().plan;
    let target_direction = caller.data().buffer.direction;

    shape_internal(&mut hb_ot_shape_context_t {
        plan,
        face,
        buffer: caller.data_mut().buffer,
        target_direction,
    });

    1
}
