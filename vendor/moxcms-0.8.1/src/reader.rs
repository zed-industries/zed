/*
 * // Copyright (c) Radzivon Bartoshyk 3/2025. All rights reserved.
 * //
 * // Redistribution and use in source and binary forms, with or without modification,
 * // are permitted provided that the following conditions are met:
 * //
 * // 1.  Redistributions of source code must retain the above copyright notice, this
 * // list of conditions and the following disclaimer.
 * //
 * // 2.  Redistributions in binary form must reproduce the above copyright notice,
 * // this list of conditions and the following disclaimer in the documentation
 * // and/or other materials provided with the distribution.
 * //
 * // 3.  Neither the name of the copyright holder nor the names of its
 * // contributors may be used to endorse or promote products derived from
 * // this software without specific prior written permission.
 * //
 * // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * // AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * // IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * // FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * // DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * // CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * // OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use crate::err::try_vec;
use crate::helpers::{read_matrix_3d, read_vector_3d};
use crate::profile::LutDataType;
use crate::safe_math::{SafeAdd, SafeMul, SafePowi};
use crate::tag::{TAG_SIZE, TagTypeDefinition};
use crate::{
    CicpColorPrimaries, CicpProfile, CmsError, ColorDateTime, ColorProfile, DescriptionString,
    LocalizableString, LutMultidimensionalType, LutStore, LutType, LutWarehouse, Matrix3d,
    Matrix3f, MatrixCoefficients, Measurement, MeasurementGeometry, ParsingOptions, ProfileText,
    StandardIlluminant, StandardObserver, TechnologySignatures, ToneReprCurve,
    TransferCharacteristics, Vector3d, ViewingConditions, Xyz, Xyzd,
};

/// Produces the nearest float to `a` with a maximum error of 1/1024 which
/// happens for large values like 0x40000040.
#[inline]
pub(crate) const fn s15_fixed16_number_to_float(a: i32) -> f32 {
    a as f32 / 65536.
}

#[inline]
pub(crate) const fn s15_fixed16_number_to_double(a: i32) -> f64 {
    a as f64 / 65536.
}

#[inline]
pub(crate) const fn uint16_number_to_float(a: u32) -> f32 {
    a as f32 / 65535.
}

#[inline]
pub(crate) const fn uint16_number_to_float_fast(a: u32) -> f32 {
    a as f32 * (1. / 65535.)
}

// #[inline]
// pub(crate) fn uint8_number_to_float(a: u8) -> f32 {
//     a as f32 / 255.0
// }

#[inline]
pub(crate) fn uint8_number_to_float_fast(a: u8) -> f32 {
    a as f32 * (1. / 255.0)
}

fn utf16be_to_utf16(slice: &[u8]) -> Result<Vec<u16>, CmsError> {
    let mut vec = try_vec![0u16; slice.len() / 2];
    for (dst, chunk) in vec.iter_mut().zip(slice.chunks_exact(2)) {
        *dst = u16::from_be_bytes([chunk[0], chunk[1]]);
    }
    Ok(vec)
}

/// Parse the Unicode section of a desc tag at `unicode_offset` (the byte
/// offset right after the ASCII data). Returns `(language_code, string)`,
/// or `Ok(None)` if truncated or the Unicode length is zero.
fn read_desc_unicode(tag: &[u8], unicode_offset: usize) -> Result<Option<(u32, String)>, CmsError> {
    if tag.len() < unicode_offset.safe_add(8)? {
        return Ok(None);
    }
    let header = &tag[unicode_offset..unicode_offset.safe_add(8)?];
    let language_code = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
    let unicode_length = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as usize;
    if unicode_length == 0 {
        return Ok(Some((language_code, String::new())));
    }
    let byte_length = unicode_length.safe_mul(2)?;
    let str_start = unicode_offset.safe_add(8)?;
    if tag.len() < str_start.safe_add(byte_length)? {
        return Ok(None);
    }
    let uc_data = &tag[str_start..str_start.safe_add(byte_length)?];
    let wc = utf16be_to_utf16(uc_data)?;
    let s = String::from_utf16_lossy(&wc)
        .trim_end_matches('\0')
        .to_string();
    Ok(Some((language_code, s)))
}

impl ColorProfile {
    pub(crate) fn read_lut_type(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<LutType, CmsError> {
        let tag_size = if tag_size == 0 { TAG_SIZE } else { tag_size };
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..last_tag_offset];
        if tag.len() < 48 {
            return Err(CmsError::InvalidProfile);
        }
        let tag_type = u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]);
        LutType::try_from(tag_type)
    }

    pub(crate) fn read_viewing_conditions(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<Option<ViewingConditions>, CmsError> {
        if tag_size < 36 {
            return Ok(None);
        }
        if slice.len() < entry.safe_add(36)? {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..entry.safe_add(36)?];
        let tag_type =
            TagTypeDefinition::from(u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]));
        // Ignore unknown
        if tag_type != TagTypeDefinition::DefViewingConditions {
            return Ok(None);
        }
        let illuminant_x = i32::from_be_bytes([tag[8], tag[9], tag[10], tag[11]]);
        let illuminant_y = i32::from_be_bytes([tag[12], tag[13], tag[14], tag[15]]);
        let illuminant_z = i32::from_be_bytes([tag[16], tag[17], tag[18], tag[19]]);

        let surround_x = i32::from_be_bytes([tag[20], tag[21], tag[22], tag[23]]);
        let surround_y = i32::from_be_bytes([tag[24], tag[25], tag[26], tag[27]]);
        let surround_z = i32::from_be_bytes([tag[28], tag[29], tag[30], tag[31]]);

        let illuminant_type = u32::from_be_bytes([tag[32], tag[33], tag[34], tag[35]]);

        let illuminant = Xyz::new(
            s15_fixed16_number_to_float(illuminant_x),
            s15_fixed16_number_to_float(illuminant_y),
            s15_fixed16_number_to_float(illuminant_z),
        );

        let surround = Xyz::new(
            s15_fixed16_number_to_float(surround_x),
            s15_fixed16_number_to_float(surround_y),
            s15_fixed16_number_to_float(surround_z),
        );

        let observer = StandardObserver::from(illuminant_type);

        Ok(Some(ViewingConditions {
            illuminant,
            surround,
            observer,
        }))
    }

    pub(crate) fn read_string_tag(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<Option<ProfileText>, CmsError> {
        let tag_size = if tag_size == 0 { TAG_SIZE } else { tag_size };
        if tag_size < 4 {
            return Ok(None);
        }
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..last_tag_offset];
        if tag.len() < 8 {
            return Ok(None);
        }
        let tag_type =
            TagTypeDefinition::from(u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]));
        // Ignore unknown
        if tag_type == TagTypeDefinition::Text {
            let sliced_from_to_end = &tag[8..tag.len()];
            let str = String::from_utf8_lossy(sliced_from_to_end)
                .trim_end_matches('\0')
                .to_string();
            return Ok(Some(ProfileText::PlainString(str)));
        } else if tag_type == TagTypeDefinition::MultiLocalizedUnicode {
            if tag.len() < 28 {
                return Err(CmsError::InvalidProfile);
            }
            // let record_size = u32::from_be_bytes([tag[12], tag[13], tag[14], tag[15]]) as usize;
            // // Record size is reserved to be 12.
            // if record_size != 12 {
            //     return Err(CmsError::InvalidIcc);
            // }
            let records_count = u32::from_be_bytes([tag[8], tag[9], tag[10], tag[11]]) as usize;
            let primary_language_code = String::from_utf8_lossy(&[tag[16], tag[17]]).to_string();
            let primary_country_code = String::from_utf8_lossy(&[tag[18], tag[19]]).to_string();
            let first_string_record_length =
                u32::from_be_bytes([tag[20], tag[21], tag[22], tag[23]]) as usize;
            let first_record_offset =
                u32::from_be_bytes([tag[24], tag[25], tag[26], tag[27]]) as usize;

            if tag.len() < first_record_offset.safe_add(first_string_record_length)? {
                return Ok(None);
            }

            let resliced =
                &tag[first_record_offset..first_record_offset + first_string_record_length];
            let cvt = utf16be_to_utf16(resliced)?;
            let string_record = String::from_utf16_lossy(&cvt);

            let mut records = vec![LocalizableString {
                language: primary_language_code,
                country: primary_country_code,
                value: string_record,
            }];

            for record in 1..records_count {
                // Localizable header must be at least 12 bytes
                let localizable_header_offset = if record == 1 {
                    28
                } else {
                    28 + 12 * (record - 1)
                };
                if tag.len() < localizable_header_offset + 12 {
                    return Err(CmsError::InvalidProfile);
                }
                let choked = &tag[localizable_header_offset..localizable_header_offset + 12];

                let language_code = String::from_utf8_lossy(&[choked[0], choked[1]]).to_string();
                let country_code = String::from_utf8_lossy(&[choked[2], choked[3]]).to_string();
                let record_length =
                    u32::from_be_bytes([choked[4], choked[5], choked[6], choked[7]]) as usize;
                let string_offset =
                    u32::from_be_bytes([choked[8], choked[9], choked[10], choked[11]]) as usize;

                if tag.len() < string_offset.safe_add(record_length)? {
                    return Ok(None);
                }
                let resliced = &tag[string_offset..string_offset + record_length];
                let cvt = utf16be_to_utf16(resliced)?;
                let string_record = String::from_utf16_lossy(&cvt);
                records.push(LocalizableString {
                    country: country_code,
                    language: language_code,
                    value: string_record,
                });
            }

            return Ok(Some(ProfileText::Localizable(records)));
        } else if tag_type == TagTypeDefinition::Description {
            if tag.len() < 12 {
                return Err(CmsError::InvalidProfile);
            }
            let ascii_length = u32::from_be_bytes([tag[8], tag[9], tag[10], tag[11]]) as usize;
            let ascii_end = 12usize.safe_add(ascii_length)?;
            if tag.len() < ascii_end {
                return Err(CmsError::InvalidProfile);
            }
            let sliced = &tag[12..ascii_end];
            let ascii_string = String::from_utf8_lossy(sliced)
                .trim_end_matches('\0')
                .to_string();

            // Tolerate truncated desc tags — the Unicode and ScriptCode
            // sections may be missing (common in non-conforming v4 profiles,
            // but also seen in some v2 profiles in the wild).
            let unicode_offset = ascii_end;
            let (unicode_code, unicode_string) =
                self::read_desc_unicode(tag, unicode_offset)?.unwrap_or((0, String::new()));

            return Ok(Some(ProfileText::Description(DescriptionString {
                ascii_string,
                unicode_language_code: unicode_code,
                unicode_string,
                mac_string: "".to_string(),
                script_code_code: -1,
            })));
        }
        Ok(None)
    }

    fn read_lut_table_f32(table: &[u8], lut_type: LutType) -> Result<LutStore, CmsError> {
        if lut_type == LutType::Lut16 {
            let mut clut = try_vec![0u16; table.len() / 2];
            for (src, dst) in table.chunks_exact(2).zip(clut.iter_mut()) {
                *dst = u16::from_be_bytes([src[0], src[1]]);
            }
            Ok(LutStore::Store16(clut))
        } else if lut_type == LutType::Lut8 {
            let mut clut = try_vec![0u8; table.len()];
            for (&src, dst) in table.iter().zip(clut.iter_mut()) {
                *dst = src;
            }
            Ok(LutStore::Store8(clut))
        } else {
            unreachable!("This should never happen, report to https://github.com/awxkee/moxcms")
        }
    }

    fn read_nested_tone_curves(
        slice: &[u8],
        offset: usize,
        length: usize,
        options: &ParsingOptions,
    ) -> Result<Option<Vec<ToneReprCurve>>, CmsError> {
        let mut curve_offset: usize = offset;
        let mut curves = Vec::new();
        for _ in 0..length {
            if slice.len() < curve_offset.safe_add(12)? {
                return Err(CmsError::InvalidProfile);
            }
            let mut tag_size = 0usize;
            let new_curve = Self::read_trc_tag(slice, curve_offset, 0, &mut tag_size, options)?;
            match new_curve {
                None => return Err(CmsError::InvalidProfile),
                Some(curve) => curves.push(curve),
            }
            curve_offset += tag_size;
            // 4 byte aligned
            if curve_offset % 4 != 0 {
                curve_offset += 4 - curve_offset % 4;
            }
        }
        Ok(Some(curves))
    }

    pub(crate) fn read_lut_abm_type(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
        to_pcs: bool,
        options: &ParsingOptions,
    ) -> Result<Option<LutWarehouse>, CmsError> {
        if tag_size < 48 {
            return Ok(None);
        }
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..last_tag_offset];
        if tag.len() < 48 {
            return Err(CmsError::InvalidProfile);
        }
        let tag_type = u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]);
        let tag_type_definition = TagTypeDefinition::from(tag_type);
        if tag_type_definition != TagTypeDefinition::MabLut
            && tag_type_definition != TagTypeDefinition::MbaLut
        {
            return Ok(None);
        }
        let in_channels = tag[8];
        let out_channels = tag[9];
        if in_channels > 4 && out_channels > 4 {
            return Ok(None);
        }
        let a_curve_offset = u32::from_be_bytes([tag[28], tag[29], tag[30], tag[31]]) as usize;
        let clut_offset = u32::from_be_bytes([tag[24], tag[25], tag[26], tag[27]]) as usize;
        let m_curve_offset = u32::from_be_bytes([tag[20], tag[21], tag[22], tag[23]]) as usize;
        let matrix_offset = u32::from_be_bytes([tag[16], tag[17], tag[18], tag[19]]) as usize;
        let b_curve_offset = u32::from_be_bytes([tag[12], tag[13], tag[14], tag[15]]) as usize;

        let transform: Matrix3d;
        let bias: Vector3d;
        if matrix_offset != 0 {
            let matrix_end = matrix_offset.safe_add(12 * 4)?;
            if tag.len() < matrix_end {
                return Err(CmsError::InvalidProfile);
            }

            let m_tag = &tag[matrix_offset..matrix_end];

            bias = read_vector_3d(&m_tag[36..48])?;
            transform = read_matrix_3d(m_tag)?;
        } else {
            transform = Matrix3d::IDENTITY;
            bias = Vector3d::default();
        }

        let mut grid_points: [u8; 16] = [0; 16];

        let clut_table: Option<LutStore> =
            if clut_offset != 0 {
                // Check if CLUT formed correctly
                if clut_offset.safe_add(20)? > tag.len() {
                    return Err(CmsError::InvalidProfile);
                }

                let clut_sizes_slice = &tag[clut_offset..clut_offset.safe_add(16)?];
                for (&s, v) in clut_sizes_slice.iter().zip(grid_points.iter_mut()) {
                    *v = s;
                }

                let mut clut_size = 1u32;
                for &i in grid_points.iter().take(in_channels as usize) {
                    clut_size = clut_size.safe_mul(i as u32)?;
                }
                clut_size = clut_size.safe_mul(out_channels as u32)?;

                if clut_size == 0 {
                    return Err(CmsError::IncorrectlyFormedLut(
                        "Clut size was zero when it shouldn't".to_string(),
                    ));
                }

                if clut_size > 10_000_000 {
                    return Err(CmsError::IncorrectlyFormedLut(
                        "Clut size exceeded 10_000_000 points what is too big".to_string(),
                    ));
                }

                // check LUT dimensions
                let mut grid_stride: usize = 1usize;
                let mut last_index: usize = 0;
                for &dim in grid_points.iter().take(in_channels as usize).rev() {
                    let dim_usize = dim as usize;
                    if dim_usize == 0 {
                        return Err(CmsError::IncorrectlyFormedLut(
                            "One of grid dimensions is zero".to_string(),
                        ));
                    }
                    let l = match dim_usize
                        .checked_sub(1)
                        .ok_or(CmsError::OverflowingError)?
                        .safe_mul(grid_stride)
                        .and_then(|x| x.safe_add(last_index))
                    {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(CmsError::IncorrectlyFormedLut(
                                "Pointer size overflow on LUT dimensions".to_string(),
                            ));
                        }
                    };
                    last_index = l;

                    // Multiply stride by next dimension (check for overflow)
                    grid_stride = grid_stride.checked_mul(dim_usize).ok_or(
                        CmsError::IncorrectlyFormedLut("Overflow on grid dimensions".to_string()),
                    )?;
                }

                last_index = last_index.checked_mul(out_channels as usize).ok_or(
                    CmsError::IncorrectlyFormedLut("Overflow on grid dimensions".to_string()),
                )?;
                if last_index >= clut_size as usize {
                    return Err(CmsError::IncorrectlyFormedLut(format!(
                        "Clut size should be at least {last_index}, but it was {last_index}"
                    )));
                }

                let clut_offset20 = clut_offset.safe_add(20)?;

                let clut_header = &tag[clut_offset..clut_offset20];
                let entry_size = clut_header[16];
                if entry_size != 1 && entry_size != 2 {
                    return Err(CmsError::InvalidProfile);
                }

                let clut_end =
                    clut_offset20.safe_add(clut_size.safe_mul(entry_size as u32)? as usize)?;

                if tag.len() < clut_end {
                    return Err(CmsError::InvalidProfile);
                }

                let shaped_clut_table = &tag[clut_offset20..clut_end];
                Some(Self::read_lut_table_f32(
                    shaped_clut_table,
                    if entry_size == 1 {
                        LutType::Lut8
                    } else {
                        LutType::Lut16
                    },
                )?)
            } else {
                None
            };

        let a_curves = if a_curve_offset == 0 {
            Vec::new()
        } else {
            Self::read_nested_tone_curves(
                tag,
                a_curve_offset,
                if to_pcs {
                    in_channels as usize
                } else {
                    out_channels as usize
                },
                options,
            )?
            .ok_or(CmsError::InvalidProfile)?
        };

        let m_curves = if m_curve_offset == 0 {
            Vec::new()
        } else {
            Self::read_nested_tone_curves(
                tag,
                m_curve_offset,
                if to_pcs {
                    out_channels as usize
                } else {
                    in_channels as usize
                },
                options,
            )?
            .ok_or(CmsError::InvalidProfile)?
        };

        let b_curves = if b_curve_offset == 0 {
            Vec::new()
        } else {
            Self::read_nested_tone_curves(
                tag,
                b_curve_offset,
                if to_pcs {
                    out_channels as usize
                } else {
                    in_channels as usize
                },
                options,
            )?
            .ok_or(CmsError::InvalidProfile)?
        };

        let wh = LutWarehouse::Multidimensional(LutMultidimensionalType {
            num_input_channels: in_channels,
            num_output_channels: out_channels,
            matrix: transform,
            clut: clut_table,
            a_curves,
            b_curves,
            m_curves,
            grid_points,
            bias,
        });
        Ok(Some(wh))
    }

    pub(crate) fn read_lut_a_to_b_type(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
        parsing_options: &ParsingOptions,
    ) -> Result<Option<LutWarehouse>, CmsError> {
        if tag_size < 48 {
            return Ok(None);
        }
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..last_tag_offset];
        if tag.len() < 48 {
            return Err(CmsError::InvalidProfile);
        }
        let tag_type = u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]);
        let lut_type = LutType::try_from(tag_type)?;
        assert!(lut_type == LutType::Lut8 || lut_type == LutType::Lut16);

        if lut_type == LutType::Lut16 && tag.len() < 52 {
            return Err(CmsError::InvalidProfile);
        }

        let num_input_table_entries: u16 = match lut_type {
            LutType::Lut8 => 256,
            LutType::Lut16 => u16::from_be_bytes([tag[48], tag[49]]),
            _ => unreachable!(),
        };
        let num_output_table_entries: u16 = match lut_type {
            LutType::Lut8 => 256,
            LutType::Lut16 => u16::from_be_bytes([tag[50], tag[51]]),
            _ => unreachable!(),
        };

        if !(2..=4096).contains(&num_input_table_entries)
            || !(2..=4096).contains(&num_output_table_entries)
        {
            return Err(CmsError::InvalidProfile);
        }

        let input_offset: usize = match lut_type {
            LutType::Lut8 => 48,
            LutType::Lut16 => 52,
            _ => unreachable!(),
        };
        let entry_size: usize = match lut_type {
            LutType::Lut8 => 1,
            LutType::Lut16 => 2,
            _ => unreachable!(),
        };

        let in_chan = tag[8];
        let out_chan = tag[9];
        let is_3_to_4 = in_chan == 3 || out_chan == 4;
        let is_4_to_3 = in_chan == 4 || out_chan == 3;
        if !is_3_to_4 && !is_4_to_3 {
            return Err(CmsError::InvalidProfile);
        }
        let grid_points = tag[10];
        let clut_size = (grid_points as u32).safe_powi(in_chan as u32)? as usize;

        if !(1..=parsing_options.max_allowed_clut_size).contains(&clut_size) {
            return Err(CmsError::InvalidProfile);
        }

        assert!(tag.len() >= 48);

        let transform = read_matrix_3d(&tag[12..48])?;

        let lut_input_size = num_input_table_entries.safe_mul(in_chan as u16)? as usize;

        let linearization_table_end = lut_input_size
            .safe_mul(entry_size)?
            .safe_add(input_offset)?;
        if tag.len() < linearization_table_end {
            return Err(CmsError::InvalidProfile);
        }
        let shaped_input_table = &tag[input_offset..linearization_table_end];
        let linearization_table = Self::read_lut_table_f32(shaped_input_table, lut_type)?;

        let clut_offset = linearization_table_end;

        let clut_data_size = clut_size
            .safe_mul(out_chan as usize)?
            .safe_mul(entry_size)?;

        if tag.len() < clut_offset.safe_add(clut_data_size)? {
            return Err(CmsError::InvalidProfile);
        }

        let shaped_clut_table = &tag[clut_offset..clut_offset + clut_data_size];
        let clut_table = Self::read_lut_table_f32(shaped_clut_table, lut_type)?;

        let output_offset = clut_offset.safe_add(clut_data_size)?;

        let output_size = (num_output_table_entries as usize).safe_mul(out_chan as usize)?;

        let shaped_output = output_offset.safe_add(output_size.safe_mul(entry_size)?)?;
        if tag.len() < shaped_output {
            return Err(CmsError::InvalidProfile);
        }

        let shaped_output_table = &tag[output_offset..shaped_output];
        let gamma_table = Self::read_lut_table_f32(shaped_output_table, lut_type)?;

        let wh = LutWarehouse::Lut(LutDataType {
            num_input_table_entries,
            num_output_table_entries,
            num_input_channels: in_chan,
            num_output_channels: out_chan,
            num_clut_grid_points: grid_points,
            matrix: transform,
            input_table: linearization_table,
            clut_table,
            output_table: gamma_table,
            lut_type,
        });
        Ok(Some(wh))
    }

    pub(crate) fn read_lut_tag(
        slice: &[u8],
        tag_entry: u32,
        tag_size: usize,
        parsing_options: &ParsingOptions,
    ) -> Result<Option<LutWarehouse>, CmsError> {
        let lut_type = Self::read_lut_type(slice, tag_entry as usize, tag_size)?;
        Ok(if lut_type == LutType::Lut8 || lut_type == LutType::Lut16 {
            Self::read_lut_a_to_b_type(slice, tag_entry as usize, tag_size, parsing_options)?
        } else if lut_type == LutType::LutMba || lut_type == LutType::LutMab {
            Self::read_lut_abm_type(
                slice,
                tag_entry as usize,
                tag_size,
                lut_type == LutType::LutMab,
                parsing_options,
            )?
        } else {
            None
        })
    }

    pub(crate) fn read_trc_tag_s(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
        options: &ParsingOptions,
    ) -> Result<Option<ToneReprCurve>, CmsError> {
        let mut _empty = 0usize;
        Self::read_trc_tag(slice, entry, tag_size, &mut _empty, options)
    }

    pub(crate) fn read_trc_tag(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
        read_size: &mut usize,
        options: &ParsingOptions,
    ) -> Result<Option<ToneReprCurve>, CmsError> {
        if slice.len() < entry.safe_add(4)? {
            return Ok(None);
        }
        let small_tag = &slice[entry..entry + 4];
        // We require always recognize tone curves.
        let curve_type = TagTypeDefinition::from(u32::from_be_bytes([
            small_tag[0],
            small_tag[1],
            small_tag[2],
            small_tag[3],
        ]));
        if tag_size != 0 && tag_size < TAG_SIZE {
            return Ok(None);
        }
        let last_tag_offset = if tag_size != 0 {
            tag_size + entry
        } else {
            slice.len()
        };
        if last_tag_offset > slice.len() {
            return Err(CmsError::MalformedTrcCurve("Data exhausted".to_string()));
        }
        let tag = &slice[entry..last_tag_offset];
        if tag.len() < TAG_SIZE {
            return Err(CmsError::MalformedTrcCurve("Data exhausted".to_string()));
        }
        if curve_type == TagTypeDefinition::LutToneCurve {
            let entry_count = u32::from_be_bytes([tag[8], tag[9], tag[10], tag[11]]) as usize;
            if entry_count == 0 {
                return Ok(Some(ToneReprCurve::Lut(vec![])));
            }
            if entry_count > options.max_allowed_trc_size {
                return Err(CmsError::CurveLutIsTooLarge);
            }
            let curve_end = entry_count.safe_mul(size_of::<u16>())?.safe_add(12)?;
            if tag.len() < curve_end {
                return Err(CmsError::MalformedTrcCurve(
                    "Curve end ends to early".to_string(),
                ));
            }
            let curve_sliced = &tag[12..curve_end];
            let mut curve_values = try_vec![0u16; entry_count];
            for (value, curve_value) in curve_sliced.chunks_exact(2).zip(curve_values.iter_mut()) {
                let gamma_s15 = u16::from_be_bytes([value[0], value[1]]);
                *curve_value = gamma_s15;
            }
            *read_size = curve_end;
            Ok(Some(ToneReprCurve::Lut(curve_values)))
        } else if curve_type == TagTypeDefinition::ParametricToneCurve {
            let entry_count = u16::from_be_bytes([tag[8], tag[9]]) as usize;
            if entry_count > 4 {
                return Err(CmsError::MalformedTrcCurve(
                    "Parametric curve has unknown entries count".to_string(),
                ));
            }

            const COUNT_TO_LENGTH: [usize; 5] = [1, 3, 4, 5, 7]; //PARAMETRIC_CURVE_TYPE

            if tag.len() < 12 + COUNT_TO_LENGTH[entry_count] * size_of::<u32>() {
                return Err(CmsError::MalformedTrcCurve(
                    "Parametric curve has unknown entries count exhaust data too early".to_string(),
                ));
            }
            let curve_sliced = &tag[12..12 + COUNT_TO_LENGTH[entry_count] * size_of::<u32>()];
            let mut params = try_vec![0f32; COUNT_TO_LENGTH[entry_count]];
            for (value, param_value) in curve_sliced.chunks_exact(4).zip(params.iter_mut()) {
                let parametric_value = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
                *param_value = s15_fixed16_number_to_float(parametric_value);
            }
            if entry_count == 1 || entry_count == 2 {
                // we have a type 1 or type 2 function that has a division by `a`
                let a: f32 = params[1];
                if a == 0.0 {
                    return Err(CmsError::ParametricCurveZeroDivision);
                }
            }
            *read_size = 12 + COUNT_TO_LENGTH[entry_count] * 4;
            Ok(Some(ToneReprCurve::Parametric(params)))
        } else {
            Err(CmsError::MalformedTrcCurve(
                "Unknown parametric curve tag".to_string(),
            ))
        }
    }

    pub(crate) fn read_chad_tag(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<Option<Matrix3d>, CmsError> {
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        if slice[entry..].len() < 8 {
            return Err(CmsError::InvalidProfile);
        }
        if tag_size < 8 {
            return Ok(None);
        }
        if (tag_size - 8) / 4 != 9 {
            return Ok(None);
        }
        let tag0 = &slice[entry..entry.safe_add(8)?];
        let c_type =
            TagTypeDefinition::from(u32::from_be_bytes([tag0[0], tag0[1], tag0[2], tag0[3]]));
        if c_type != TagTypeDefinition::S15Fixed16Array {
            return Err(CmsError::InvalidProfile);
        }
        if slice.len() < 9 * size_of::<u32>() + 8 {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry + 8..last_tag_offset];
        if tag.len() != size_of::<Matrix3f>() {
            return Err(CmsError::InvalidProfile);
        }
        let matrix = read_matrix_3d(tag)?;
        Ok(Some(matrix))
    }

    #[inline]
    pub(crate) fn read_tech_tag(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<Option<TechnologySignatures>, CmsError> {
        if tag_size < TAG_SIZE {
            return Err(CmsError::InvalidProfile);
        }
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..entry.safe_add(12)?];
        let tag_type = u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]);
        let def = TagTypeDefinition::from(tag_type);
        if def == TagTypeDefinition::Signature {
            let sig = u32::from_be_bytes([tag[8], tag[9], tag[10], tag[11]]);
            let tech_sig = TechnologySignatures::from(sig);
            return Ok(Some(tech_sig));
        }
        Ok(None)
    }

    pub(crate) fn read_date_time_tag(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<Option<ColorDateTime>, CmsError> {
        if tag_size < 20 {
            return Ok(None);
        }
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..entry.safe_add(20)?];
        let tag_type = u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]);
        let def = TagTypeDefinition::from(tag_type);
        if def == TagTypeDefinition::DateTime {
            let tag_value = &slice[8..20];
            let time = ColorDateTime::new_from_slice(tag_value)?;
            return Ok(Some(time));
        }
        Ok(None)
    }

    #[inline]
    pub(crate) fn read_meas_tag(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<Option<Measurement>, CmsError> {
        if tag_size < TAG_SIZE {
            return Ok(None);
        }
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..entry + 12];
        let tag_type = u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]);
        let def = TagTypeDefinition::from(tag_type);
        if def != TagTypeDefinition::Measurement {
            return Ok(None);
        }
        if 36 + entry > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..entry + 36];
        let observer =
            StandardObserver::from(u32::from_be_bytes([tag[8], tag[9], tag[10], tag[11]]));
        let q15_16_x = i32::from_be_bytes([tag[12], tag[13], tag[14], tag[15]]);
        let q15_16_y = i32::from_be_bytes([tag[16], tag[17], tag[18], tag[19]]);
        let q15_16_z = i32::from_be_bytes([tag[20], tag[21], tag[22], tag[23]]);
        let x = s15_fixed16_number_to_float(q15_16_x);
        let y = s15_fixed16_number_to_float(q15_16_y);
        let z = s15_fixed16_number_to_float(q15_16_z);
        let xyz = Xyz::new(x, y, z);
        let geometry =
            MeasurementGeometry::from(u32::from_be_bytes([tag[24], tag[25], tag[26], tag[27]]));
        let flare =
            uint16_number_to_float(u32::from_be_bytes([tag[28], tag[29], tag[30], tag[31]]));
        let illuminant =
            StandardIlluminant::from(u32::from_be_bytes([tag[32], tag[33], tag[34], tag[35]]));
        Ok(Some(Measurement {
            flare,
            illuminant,
            geometry,
            observer,
            backing: xyz,
        }))
    }

    #[inline]
    pub(crate) fn read_xyz_tag(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<Xyzd, CmsError> {
        if tag_size < TAG_SIZE {
            return Ok(Xyzd::default());
        }
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..entry + 12];
        let tag_type = u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]);
        let def = TagTypeDefinition::from(tag_type);
        if def != TagTypeDefinition::Xyz {
            return Ok(Xyzd::default());
        }

        let tag = &slice[entry..last_tag_offset];
        if tag.len() < 20 {
            return Err(CmsError::InvalidProfile);
        }
        let q15_16_x = i32::from_be_bytes([tag[8], tag[9], tag[10], tag[11]]);
        let q15_16_y = i32::from_be_bytes([tag[12], tag[13], tag[14], tag[15]]);
        let q15_16_z = i32::from_be_bytes([tag[16], tag[17], tag[18], tag[19]]);
        let x = s15_fixed16_number_to_double(q15_16_x);
        let y = s15_fixed16_number_to_double(q15_16_y);
        let z = s15_fixed16_number_to_double(q15_16_z);
        Ok(Xyzd { x, y, z })
    }

    pub(crate) fn read_cicp_tag(
        slice: &[u8],
        entry: usize,
        tag_size: usize,
    ) -> Result<Option<CicpProfile>, CmsError> {
        if tag_size < TAG_SIZE {
            return Ok(None);
        }
        let last_tag_offset = tag_size.safe_add(entry)?;
        if last_tag_offset > slice.len() {
            return Err(CmsError::InvalidProfile);
        }
        let tag = &slice[entry..last_tag_offset];
        if tag.len() < 12 {
            return Err(CmsError::InvalidProfile);
        }
        let tag_type = u32::from_be_bytes([tag[0], tag[1], tag[2], tag[3]]);
        let def = TagTypeDefinition::from(tag_type);
        if def != TagTypeDefinition::Cicp {
            return Ok(None);
        }
        let primaries = CicpColorPrimaries::try_from(tag[8])?;
        let transfer_characteristics = TransferCharacteristics::try_from(tag[9])?;
        let matrix_coefficients = MatrixCoefficients::try_from(tag[10])?;
        let full_range = tag[11] == 1;
        Ok(Some(CicpProfile {
            color_primaries: primaries,
            transfer_characteristics,
            matrix_coefficients,
            full_range,
        }))
    }
}
