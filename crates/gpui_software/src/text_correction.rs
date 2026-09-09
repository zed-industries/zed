use collections::FxHashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontCorrection {
    pub gamma_ratios: [f32; 4],
    pub grayscale_enhanced_contrast: f32,
    pub subpixel_enhanced_contrast: f32,
    pub is_bgr: bool,
}

impl Default for FontCorrection {
    fn default() -> Self {
        Self {
            gamma_ratios: [0.0; 4],
            grayscale_enhanced_contrast: 0.0,
            subpixel_enhanced_contrast: 0.0,
            is_bgr: false,
        }
    }
}

#[derive(Clone)]
pub(crate) struct AlphaLut(pub [u32; 256]);

#[derive(Clone)]
pub(crate) struct AlphaLut3(pub [[u32; 256]; 3]);

#[derive(Default)]
pub(crate) struct LutCache {
    mono: FxHashMap<u32, usize>,
    subpixel: FxHashMap<u32, usize>,
    pub mono_luts: Vec<AlphaLut>,
    pub subpixel_luts: Vec<AlphaLut3>,
}

impl LutCache {
    pub fn trim(&mut self) {
        if self.mono_luts.len() > 256 {
            self.mono = Default::default();
            self.mono_luts = Vec::new();
        }
        if self.subpixel_luts.len() > 256 {
            self.subpixel = Default::default();
            self.subpixel_luts = Vec::new();
        }
    }

    pub fn mono(&mut self, color: u32, correction: FontCorrection) -> usize {
        if let Some(index) = self.mono.get(&color) {
            return *index;
        }
        let index = self.mono_luts.len();
        self.mono_luts.push(build_mono_lut(color, correction));
        self.mono.insert(color, index);
        index
    }

    pub fn subpixel(&mut self, color: u32, correction: FontCorrection) -> usize {
        if let Some(index) = self.subpixel.get(&color) {
            return *index;
        }
        let index = self.subpixel_luts.len();
        self.subpixel_luts
            .push(build_subpixel_lut(color, correction));
        self.subpixel.insert(color, index);
        index
    }
}

fn channels(color: u32) -> [f32; 4] {
    [
        ((color >> 16) & 0xff) as f32 / 255.0,
        ((color >> 8) & 0xff) as f32 / 255.0,
        (color & 0xff) as f32 / 255.0,
        ((color >> 24) & 0xff) as f32 / 255.0,
    ]
}

fn corrected_alpha(sample: f32, brightness: f32, contrast: f32, gamma: [f32; 4]) -> f32 {
    let contrasted = sample * (contrast + 1.0) / (sample * contrast + 1.0);
    let adjustment = gamma[0] * brightness + gamma[1];
    let correction = adjustment * contrasted + (gamma[2] * brightness + gamma[3]);
    (contrasted + contrasted * (1.0 - contrasted) * correction).clamp(0.0, 1.0)
}

fn contrast(color: [f32; 4], enhanced_contrast: f32) -> f32 {
    let brightness = 0.30 * color[0] + 0.59 * color[1] + 0.11 * color[2];
    enhanced_contrast * (4.0 * (0.75 - brightness)).clamp(0.0, 1.0)
}

fn build_mono_lut(color: u32, correction: FontCorrection) -> AlphaLut {
    let color = channels(color);
    let brightness = 0.30 * color[0] + 0.59 * color[1] + 0.11 * color[2];
    let contrast = contrast(color, correction.grayscale_enhanced_contrast);
    AlphaLut(std::array::from_fn(|sample| {
        (corrected_alpha(
            sample as f32 / 255.0,
            brightness,
            contrast,
            correction.gamma_ratios,
        ) * color[3]
            * 255.0)
            .round() as u32
    }))
}

fn build_subpixel_lut(color: u32, correction: FontCorrection) -> AlphaLut3 {
    let color = channels(color);
    let contrast = contrast(color, correction.subpixel_enhanced_contrast);
    AlphaLut3(std::array::from_fn(|channel| {
        std::array::from_fn(|sample| {
            (corrected_alpha(
                sample as f32 / 255.0,
                color[channel],
                contrast,
                correction.gamma_ratios,
            ) * color[3]
                * 255.0)
                .round() as u32
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correction_tables_match_float_reference() {
        let correction = FontCorrection {
            gamma_ratios: [0.15, -0.1, 0.2, -0.04],
            grayscale_enhanced_contrast: 0.5,
            subpixel_enhanced_contrast: 0.75,
            is_bgr: true,
        };
        for color in [0xff00_0000, 0xffff_ffff, 0x8042_91e1] {
            let mono = build_mono_lut(color, correction);
            let subpixel = build_subpixel_lut(color, correction);
            let rgba = channels(color);
            let brightness = 0.30 * rgba[0] + 0.59 * rgba[1] + 0.11 * rgba[2];
            let enhanced = contrast(rgba, correction.grayscale_enhanced_contrast);
            let subpixel_contrast = contrast(rgba, correction.subpixel_enhanced_contrast);
            for sample in 0..=255 {
                let expected = (corrected_alpha(
                    sample as f32 / 255.0,
                    brightness,
                    enhanced,
                    correction.gamma_ratios,
                ) * rgba[3]
                    * 255.0)
                    .round() as i16;
                assert!((mono.0[sample] as i16 - expected).abs() <= 1);
                for channel in 0..3 {
                    let expected = (corrected_alpha(
                        sample as f32 / 255.0,
                        rgba[channel],
                        subpixel_contrast,
                        correction.gamma_ratios,
                    ) * rgba[3]
                        * 255.0)
                        .round() as i16;
                    assert!((subpixel.0[channel][sample] as i16 - expected).abs() <= 1);
                }
            }
        }
    }
}
