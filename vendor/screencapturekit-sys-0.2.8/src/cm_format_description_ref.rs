#![allow(non_upper_case_globals)]

use crate::macros::declare_ref_type;
declare_ref_type!(CMFormatDescriptionRef);

impl CMFormatDescriptionRef {
    pub fn audio_format_description_get_stream_basic_description(
        &self,
    ) -> Option<&AudioStreamBasicDescription> {
        unsafe {
            let ptr = CMAudioFormatDescriptionGetStreamBasicDescription(self);
            if ptr.is_null() {
                return None;
            }
            Some(&*ptr)
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AudioStreamBasicDescription {
    pub sample_rate: f64,
    pub format_id: ::std::os::raw::c_uint,
    pub format_flags: ::std::os::raw::c_uint,
    pub bytes_per_packet: ::std::os::raw::c_uint,
    pub frames_per_packet: ::std::os::raw::c_uint,
    pub bytes_per_frame: ::std::os::raw::c_uint,
    pub channels_per_frame: ::std::os::raw::c_uint,
    pub bits_per_channel: ::std::os::raw::c_uint,
    pub reserved: ::std::os::raw::c_uint,
}

impl AudioStreamBasicDescription {
    pub fn get_flag_names(&self) -> Vec<&'static str> {
        let mut flag_strings = Vec::new();
        let flags = self.format_flags;

        if flags & kAudioFormatFlagIsFloat != 0 {
            flag_strings.push("kAudioFormatFlagIsFloat");
        }
        if flags & kAudioFormatFlagIsBigEndian != 0 {
            flag_strings.push("kAudioFormatFlagIsBigEndian");
        }
        if flags & kAudioFormatFlagIsSignedInteger != 0 {
            flag_strings.push("kAudioFormatFlagIsSignedInteger");
        }
        if flags & kAudioFormatFlagIsPacked != 0 {
            flag_strings.push("kAudioFormatFlagIsPacked");
        }
        if flags & kAudioFormatFlagIsAlignedHigh != 0 {
            flag_strings.push("kAudioFormatFlagIsAlignedHigh");
        }
        if flags & kAudioFormatFlagIsNonInterleaved != 0 {
            flag_strings.push("kAudioFormatFlagIsNonInterleaved");
        }
        if flags & kAudioFormatFlagIsNonMixable != 0 {
            flag_strings.push("kAudioFormatFlagIsNonMixable");
        }

        // kAudioFormatFlagsAreAllClear flag is a special case and should be checked last.
        if flags & kAudioFormatFlagsAreAllClear != 0 {
            flag_strings.push("All flags are clear");
        }

        flag_strings
    }

    pub fn get_format_name(&self) -> Option<&str> {
        match self.format_id {
            kAudioFormatLinearPCM => Some("kAudioFormatLinearPCM"),
            kAudioFormatAC3 => Some("kAudioFormatAC3"),
            kAudioFormat60958AC3 => Some("kAudioFormat60958AC3"),
            kAudioFormatAppleIMA4 => Some("kAudioFormatAppleIMA4"),
            kAudioFormatMPEG4AAC => Some("kAudioFormatMPEG4AAC"),
            kAudioFormatMPEG4CELP => Some("kAudioFormatMPEG4CELP"),
            kAudioFormatMPEG4HVXC => Some("kAudioFormatMPEG4HVXC"),
            kAudioFormatMPEG4TwinVQ => Some("kAudioFormatMPEG4TwinVQ"),
            kAudioFormatMACE3 => Some("kAudioFormatMACE3"),
            kAudioFormatMACE6 => Some("kAudioFormatMACE6"),
            kAudioFormatULaw => Some("kAudioFormatULaw"),
            kAudioFormatALaw => Some("kAudioFormatALaw"),
            kAudioFormatQDesign => Some("kAudioFormatQDesign"),
            kAudioFormatQDesign2 => Some("kAudioFormatQDesign2"),
            kAudioFormatQUALCOMM => Some("kAudioFormatQUALCOMM"),
            kAudioFormatMPEGLayer1 => Some("kAudioFormatMPEGLayer1"),
            kAudioFormatMPEGLayer2 => Some("kAudioFormatMPEGLayer2"),
            kAudioFormatMPEGLayer3 => Some("kAudioFormatMPEGLayer3"),
            kAudioFormatTimeCode => Some("kAudioFormatTimeCode"),
            kAudioFormatMIDIStream => Some("kAudioFormatMIDIStream"),
            kAudioFormatParameterValueStream => Some("kAudioFormatParameterValueStream"),
            kAudioFormatAppleLossless => Some("kAudioFormatAppleLossless"),
            kAudioFormatMPEG4AAC_HE => Some("kAudioFormatMPEG4AAC_HE"),
            kAudioFormatMPEG4AAC_LD => Some("kAudioFormatMPEG4AAC_LD"),
            kAudioFormatMPEG4AAC_ELD => Some("kAudioFormatMPEG4AAC_ELD"),
            kAudioFormatMPEG4AAC_ELD_SBR => Some("kAudioFormatMPEG4AAC_ELD_SBR"),
            kAudioFormatMPEG4AAC_ELD_V2 => Some("kAudioFormatMPEG4AAC_ELD_V2"),
            kAudioFormatMPEG4AAC_HE_V2 => Some("kAudioFormatMPEG4AAC_HE_V2"),
            kAudioFormatMPEG4AAC_Spatial => Some("kAudioFormatMPEG4AAC_Spatial"),
            kAudioFormatMPEGD_USAC => Some("kAudioFormatMPEGD_USAC"),
            kAudioFormatAMR => Some("kAudioFormatAMR"),
            kAudioFormatAMR_WB => Some("kAudioFormatAMR_WB"),
            kAudioFormatAudible => Some("kAudioFormatAudible"),
            kAudioFormatiLBC => Some("kAudioFormatiLBC"),
            kAudioFormatDVIIntelIMA => Some("kAudioFormatDVIIntelIMA"),
            kAudioFormatMicrosoftGSM => Some("kAudioFormatMicrosoftGSM"),
            kAudioFormatAES3 => Some("kAudioFormatAES3"),
            kAudioFormatEnhancedAC3 => Some("kAudioFormatEnhancedAC3"),
            kAudioFormatFLAC => Some("kAudioFormatFLAC"),
            kAudioFormatOpus => Some("kAudioFormatOpus"),
            _ => None,
        }
    }
}

pub const kAudioFormatFlagIsFloat: u32 = 1 << 0;
pub const kAudioFormatFlagIsBigEndian: u32 = 1 << 1;
pub const kAudioFormatFlagIsSignedInteger: u32 = 1 << 2;
pub const kAudioFormatFlagIsPacked: u32 = 1 << 3;
pub const kAudioFormatFlagIsAlignedHigh: u32 = 1 << 4;
pub const kAudioFormatFlagIsNonInterleaved: u32 = 1 << 5;
pub const kAudioFormatFlagIsNonMixable: u32 = 1 << 6;
pub const kAudioFormatFlagsAreAllClear: u32 = 1 << 31;

pub const kLinearPCMFormatFlagIsFloat: u32 = kAudioFormatFlagIsFloat;
pub const kLinearPCMFormatFlagIsBigEndian: u32 = kAudioFormatFlagIsBigEndian;
pub const kLinearPCMFormatFlagIsSignedInteger: u32 = kAudioFormatFlagIsSignedInteger;
pub const kLinearPCMFormatFlagIsPacked: u32 = kAudioFormatFlagIsPacked;
pub const kLinearPCMFormatFlagIsAlignedHigh: u32 = kAudioFormatFlagIsAlignedHigh;
pub const kLinearPCMFormatFlagIsNonInterleaved: u32 = kAudioFormatFlagIsNonInterleaved;
pub const kLinearPCMFormatFlagIsNonMixable: u32 = kAudioFormatFlagIsNonMixable;
pub const kLinearPCMFormatFlagsAreAllClear: u32 = kAudioFormatFlagsAreAllClear;

pub const kAppleLosslessFormatFlag16BitSourceData: u32 = 1;
pub const kAppleLosslessFormatFlag20BitSourceData: u32 = 2;
pub const kAppleLosslessFormatFlag24BitSourceData: u32 = 3;
pub const kAppleLosslessFormatFlag32BitSourceData: u32 = 4;

pub const kAudioFormatLinearPCM: ::std::os::raw::c_uint = 1819304813;
pub const kAudioFormatAC3: ::std::os::raw::c_uint = 1633889587;
pub const kAudioFormat60958AC3: ::std::os::raw::c_uint = 1667326771;
pub const kAudioFormatAppleIMA4: ::std::os::raw::c_uint = 1768775988;
pub const kAudioFormatMPEG4AAC: ::std::os::raw::c_uint = 1633772320;
pub const kAudioFormatMPEG4CELP: ::std::os::raw::c_uint = 1667591280;
pub const kAudioFormatMPEG4HVXC: ::std::os::raw::c_uint = 1752594531;
pub const kAudioFormatMPEG4TwinVQ: ::std::os::raw::c_uint = 1953986161;
pub const kAudioFormatMACE3: ::std::os::raw::c_uint = 1296122675;
pub const kAudioFormatMACE6: ::std::os::raw::c_uint = 1296122678;
pub const kAudioFormatULaw: ::std::os::raw::c_uint = 1970037111;
pub const kAudioFormatALaw: ::std::os::raw::c_uint = 1634492791;
pub const kAudioFormatQDesign: ::std::os::raw::c_uint = 1363430723;
pub const kAudioFormatQDesign2: ::std::os::raw::c_uint = 1363430706;
pub const kAudioFormatQUALCOMM: ::std::os::raw::c_uint = 1365470320;
pub const kAudioFormatMPEGLayer1: ::std::os::raw::c_uint = 778924081;
pub const kAudioFormatMPEGLayer2: ::std::os::raw::c_uint = 778924082;
pub const kAudioFormatMPEGLayer3: ::std::os::raw::c_uint = 778924083;
pub const kAudioFormatTimeCode: ::std::os::raw::c_uint = 1953066341;
pub const kAudioFormatMIDIStream: ::std::os::raw::c_uint = 1835623529;
pub const kAudioFormatParameterValueStream: ::std::os::raw::c_uint = 1634760307;
pub const kAudioFormatAppleLossless: ::std::os::raw::c_uint = 1634492771;
pub const kAudioFormatMPEG4AAC_HE: ::std::os::raw::c_uint = 1633772392;
pub const kAudioFormatMPEG4AAC_LD: ::std::os::raw::c_uint = 1633772396;
pub const kAudioFormatMPEG4AAC_ELD: ::std::os::raw::c_uint = 1633772389;
pub const kAudioFormatMPEG4AAC_ELD_SBR: ::std::os::raw::c_uint = 1633772390;
pub const kAudioFormatMPEG4AAC_ELD_V2: ::std::os::raw::c_uint = 1633772391;
pub const kAudioFormatMPEG4AAC_HE_V2: ::std::os::raw::c_uint = 1633772400;
pub const kAudioFormatMPEG4AAC_Spatial: ::std::os::raw::c_uint = 1633772403;
pub const kAudioFormatMPEGD_USAC: ::std::os::raw::c_uint = 1970495843;
pub const kAudioFormatAMR: ::std::os::raw::c_uint = 1935764850;
pub const kAudioFormatAMR_WB: ::std::os::raw::c_uint = 1935767394;
pub const kAudioFormatAudible: ::std::os::raw::c_uint = 1096107074;
pub const kAudioFormatiLBC: ::std::os::raw::c_uint = 1768710755;
pub const kAudioFormatDVIIntelIMA: ::std::os::raw::c_uint = 1836253201;
pub const kAudioFormatMicrosoftGSM: ::std::os::raw::c_uint = 1836253233;
pub const kAudioFormatAES3: ::std::os::raw::c_uint = 1634038579;
pub const kAudioFormatEnhancedAC3: ::std::os::raw::c_uint = 1700998451;
pub const kAudioFormatFLAC: ::std::os::raw::c_uint = 1718378851;
pub const kAudioFormatOpus: ::std::os::raw::c_uint = 1869641075;

extern "C" {
    pub fn CMAudioFormatDescriptionGetStreamBasicDescription(
        desc: *const CMFormatDescriptionRef,
    ) -> *const AudioStreamBasicDescription;
}
