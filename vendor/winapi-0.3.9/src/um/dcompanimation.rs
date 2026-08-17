// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dcompanimation.h
use ctypes::{c_double, c_float};
use shared::ntdef::{HRESULT, LARGE_INTEGER};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
RIDL!{#[uuid(0xcbfd91d9, 0x51b2, 0x45e4, 0xb3, 0xde, 0xd1, 0x9c, 0xcf, 0xb8, 0x63, 0xc5)]
interface IDCompositionAnimation(IDCompositionAnimationVtbl): IUnknown(IUnknownVtbl) {
    fn Reset() -> HRESULT,
    fn SetAbsoluteBeginTime(
        beginTime: LARGE_INTEGER,
    ) -> HRESULT,
    fn AddCubic(
        beginOffset: c_double,
        constantCoefficient: c_float,
        linearCoefficient: c_float,
        quadraticCoefficient: c_float,
        cubicCoefficient: c_float,
    )-> HRESULT,
    fn AddSinusoidal(
        beginOffset: c_double,
        bias: c_float,
        amplitude: c_float,
        frequency: c_float,
        phase: c_float,
    )-> HRESULT,
    fn AddRepeat(
        beginOffset: c_double,
        durationToRepeat: c_double,
    )-> HRESULT,
    fn End(
        endOffset: c_double,
        endValue: c_float,
    ) -> HRESULT,
}}
