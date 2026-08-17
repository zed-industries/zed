// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of wincodec.h
use ctypes::c_double;
use shared::basetsd::{UINT32, ULONG_PTR};
use shared::dxgiformat::DXGI_FORMAT;
use shared::dxgitype::{
    DXGI_JPEG_AC_HUFFMAN_TABLE, DXGI_JPEG_DC_HUFFMAN_TABLE,
    DXGI_JPEG_QUANTIZATION_TABLE
};
use shared::guiddef::{CLSID, GUID, REFCLSID, REFGUID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, INT, LPVOID, UINT, ULONG};
use shared::ntdef::{LPCWSTR, LPWSTR, PCWSTR, WCHAR};
use shared::windef::{HBITMAP, HICON, HPALETTE};
use shared::winerror::{
    E_ABORT, E_ACCESSDENIED, E_FAIL, E_INVALIDARG, E_NOTIMPL, E_OUTOFMEMORY, HRESULT,
    SEVERITY_ERROR
};
use um::d2d1::ID2D1Image;
use um::d2d1_1::ID2D1Device;
use um::dcommon::D2D1_PIXEL_FORMAT;
use um::objidlbase::{IEnumString, IEnumUnknown, IStream, IStreamVtbl};
use um::ocidl::IPropertyBag2;
use um::propidl::PROPVARIANT;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, ULARGE_INTEGER};
DEFINE_GUID!{CLSID_WICImagingFactory,
    0xcacaf262, 0x9370, 0x4615, 0xa1, 0x3b, 0x9f, 0x55, 0x39, 0xda, 0x4c, 0xa}
DEFINE_GUID!{CLSID_WICImagingFactory1,
    0xcacaf262, 0x9370, 0x4615, 0xa1, 0x3b, 0x9f, 0x55, 0x39, 0xda, 0x4c, 0xa}
DEFINE_GUID!{CLSID_WICImagingFactory2,
    0x317d06e8, 0x5f24, 0x433d, 0xbd, 0xf7, 0x79, 0xce, 0x68, 0xd8, 0xab, 0xc2}
DEFINE_GUID!{GUID_VendorMicrosoft,
    0xf0e749ca, 0xedef, 0x4589, 0xa7, 0x3a, 0xee, 0xe, 0x62, 0x6a, 0x2a, 0x2b}
DEFINE_GUID!{GUID_VendorMicrosoftBuiltIn,
    0x257a30fd, 0x6b6, 0x462b, 0xae, 0xa4, 0x63, 0xf7, 0xb, 0x86, 0xe5, 0x33}
DEFINE_GUID!{CLSID_WICPngDecoder,
    0x389ea17b, 0x5078, 0x4cde, 0xb6, 0xef, 0x25, 0xc1, 0x51, 0x75, 0xc7, 0x51}
DEFINE_GUID!{CLSID_WICPngDecoder1,
    0x389ea17b, 0x5078, 0x4cde, 0xb6, 0xef, 0x25, 0xc1, 0x51, 0x75, 0xc7, 0x51}
DEFINE_GUID!{CLSID_WICPngDecoder2,
    0xe018945b, 0xaa86, 0x4008, 0x9b, 0xd4, 0x67, 0x77, 0xa1, 0xe4, 0x0c, 0x11}
DEFINE_GUID!{CLSID_WICBmpDecoder,
    0x6b462062, 0x7cbf, 0x400d, 0x9f, 0xdb, 0x81, 0x3d, 0xd1, 0x0f, 0x27, 0x78}
DEFINE_GUID!{CLSID_WICIcoDecoder,
    0xc61bfcdf, 0x2e0f, 0x4aad, 0xa8, 0xd7, 0xe0, 0x6b, 0xaf, 0xeb, 0xcd, 0xfe}
DEFINE_GUID!{CLSID_WICJpegDecoder,
    0x9456a480, 0xe88b, 0x43ea, 0x9e, 0x73, 0x0b, 0x2d, 0x9b, 0x71, 0xb1, 0xca}
DEFINE_GUID!{CLSID_WICGifDecoder,
    0x381dda3c, 0x9ce9, 0x4834, 0xa2, 0x3e, 0x1f, 0x98, 0xf8, 0xfc, 0x52, 0xbe}
DEFINE_GUID!{CLSID_WICTiffDecoder,
    0xb54e85d9, 0xfe23, 0x499f, 0x8b, 0x88, 0x6a, 0xce, 0xa7, 0x13, 0x75, 0x2b}
DEFINE_GUID!{CLSID_WICWmpDecoder,
    0xa26cec36, 0x234c, 0x4950, 0xae, 0x16, 0xe3, 0x4a, 0xac, 0xe7, 0x1d, 0x0d}
DEFINE_GUID!{CLSID_WICDdsDecoder,
    0x9053699f, 0xa341, 0x429d, 0x9e, 0x90, 0xee, 0x43, 0x7c, 0xf8, 0x0c, 0x73}
DEFINE_GUID!{CLSID_WICBmpEncoder,
    0x69be8bb4, 0xd66d, 0x47c8, 0x86, 0x5a, 0xed, 0x15, 0x89, 0x43, 0x37, 0x82}
DEFINE_GUID!{CLSID_WICPngEncoder,
    0x27949969, 0x876a, 0x41d7, 0x94, 0x47, 0x56, 0x8f, 0x6a, 0x35, 0xa4, 0xdc}
DEFINE_GUID!{CLSID_WICJpegEncoder,
    0x1a34f5c1, 0x4a5a, 0x46dc, 0xb6, 0x44, 0x1f, 0x45, 0x67, 0xe7, 0xa6, 0x76}
DEFINE_GUID!{CLSID_WICGifEncoder,
    0x114f5598, 0x0b22, 0x40a0, 0x86, 0xa1, 0xc8, 0x3e, 0xa4, 0x95, 0xad, 0xbd}
DEFINE_GUID!{CLSID_WICTiffEncoder,
    0x0131be10, 0x2001, 0x4c5f, 0xa9, 0xb0, 0xcc, 0x88, 0xfa, 0xb6, 0x4c, 0xe8}
DEFINE_GUID!{CLSID_WICWmpEncoder,
    0xac4ce3cb, 0xe1c1, 0x44cd, 0x82, 0x15, 0x5a, 0x16, 0x65, 0x50, 0x9e, 0xc2}
DEFINE_GUID!{CLSID_WICDdsEncoder,
    0xa61dde94, 0x66ce, 0x4ac1, 0x88, 0x1b, 0x71, 0x68, 0x05, 0x88, 0x89, 0x5e}
DEFINE_GUID!{CLSID_WICAdngDecoder,
    0x981d9411, 0x909e, 0x42a7, 0x8f, 0x5d, 0xa7, 0x47, 0xff, 0x05, 0x2e, 0xdb}
DEFINE_GUID!{CLSID_WICJpegQualcommPhoneEncoder,
    0x68ed5c62, 0xf534, 0x4979, 0xb2, 0xb3, 0x68, 0x6a, 0x12, 0xb2, 0xb3, 0x4c}
DEFINE_GUID!{GUID_ContainerFormatBmp,
    0x0af1d87e, 0xfcfe, 0x4188, 0xbd, 0xeb, 0xa7, 0x90, 0x64, 0x71, 0xcb, 0xe3}
DEFINE_GUID!{GUID_ContainerFormatPng,
    0x1b7cfaf4, 0x713f, 0x473c, 0xbb, 0xcd, 0x61, 0x37, 0x42, 0x5f, 0xae, 0xaf}
DEFINE_GUID!{GUID_ContainerFormatIco,
    0xa3a860c4, 0x338f, 0x4c17, 0x91, 0x9a, 0xfb, 0xa4, 0xb5, 0x62, 0x8f, 0x21}
DEFINE_GUID!{GUID_ContainerFormatJpeg,
    0x19e4a5aa, 0x5662, 0x4fc5, 0xa0, 0xc0, 0x17, 0x58, 0x02, 0x8e, 0x10, 0x57}
DEFINE_GUID!{GUID_ContainerFormatTiff,
    0x163bcc30, 0xe2e9, 0x4f0b, 0x96, 0x1d, 0xa3, 0xe9, 0xfd, 0xb7, 0x88, 0xa3}
DEFINE_GUID!{GUID_ContainerFormatGif,
    0x1f8a5601, 0x7d4d, 0x4cbd, 0x9c, 0x82, 0x1b, 0xc8, 0xd4, 0xee, 0xb9, 0xa5}
DEFINE_GUID!{GUID_ContainerFormatWmp,
    0x57a37caa, 0x367a, 0x4540, 0x91, 0x6b, 0xf1, 0x83, 0xc5, 0x09, 0x3a, 0x4b}
DEFINE_GUID!{GUID_ContainerFormatDds,
    0x9967cb95, 0x2e85, 0x4ac8, 0x8c, 0xa2, 0x83, 0xd7, 0xcc, 0xd4, 0x25, 0xc9}
DEFINE_GUID!{GUID_ContainerFormatAdng,
    0xf3ff6d0d, 0x38c0, 0x41c4, 0xb1, 0xfe, 0x1f, 0x38, 0x24, 0xf1, 0x7b, 0x84}
DEFINE_GUID!{CLSID_WICImagingCategories,
    0xfae3d380, 0xfea4, 0x4623, 0x8c, 0x75, 0xc6, 0xb6, 0x11, 0x10, 0xb6, 0x81}
DEFINE_GUID!{CATID_WICBitmapDecoders,
    0x7ed96837, 0x96f0, 0x4812, 0xb2, 0x11, 0xf1, 0x3c, 0x24, 0x11, 0x7e, 0xd3}
DEFINE_GUID!{CATID_WICBitmapEncoders,
    0xac757296, 0x3522, 0x4e11, 0x98, 0x62, 0xc1, 0x7b, 0xe5, 0xa1, 0x76, 0x7e}
DEFINE_GUID!{CATID_WICPixelFormats,
    0x2b46e70f, 0xcda7, 0x473e, 0x89, 0xf6, 0xdc, 0x96, 0x30, 0xa2, 0x39, 0x0b}
DEFINE_GUID!{CATID_WICFormatConverters,
    0x7835eae8, 0xbf14, 0x49d1, 0x93, 0xce, 0x53, 0x3a, 0x40, 0x7b, 0x22, 0x48}
DEFINE_GUID!{CATID_WICMetadataReader,
    0x05af94d8, 0x7174, 0x4cd2, 0xbe, 0x4a, 0x41, 0x24, 0xb8, 0x0e, 0xe4, 0xb8}
DEFINE_GUID!{CATID_WICMetadataWriter,
    0xabe3b9a4, 0x257d, 0x4b97, 0xbd, 0x1a, 0x29, 0x4a, 0xf4, 0x96, 0x22, 0x2e}
DEFINE_GUID!{CLSID_WICDefaultFormatConverter,
    0x1a3f11dc, 0xb514, 0x4b17, 0x8c, 0x5f, 0x21, 0x54, 0x51, 0x38, 0x52, 0xf1}
DEFINE_GUID!{CLSID_WICFormatConverterHighColor,
    0xac75d454, 0x9f37, 0x48f8, 0xb9, 0x72, 0x4e, 0x19, 0xbc, 0x85, 0x60, 0x11}
DEFINE_GUID!{CLSID_WICFormatConverterNChannel,
    0xc17cabb2, 0xd4a3, 0x47d7, 0xa5, 0x57, 0x33, 0x9b, 0x2e, 0xfb, 0xd4, 0xf1}
DEFINE_GUID!{CLSID_WICFormatConverterWMPhoto,
    0x9cb5172b, 0xd600, 0x46ba, 0xab, 0x77, 0x77, 0xbb, 0x7e, 0x3a, 0x00, 0xd9}
DEFINE_GUID!{CLSID_WICPlanarFormatConverter,
    0x184132b8, 0x32f8, 0x4784, 0x91, 0x31, 0xdd, 0x72, 0x24, 0xb2, 0x34, 0x38}
pub type WICColor = UINT32;
STRUCT!{struct WICRect {
    X: INT,
    Y: INT,
    Width: INT,
    Height: INT,
}}
pub type WICInProcPointer = *mut BYTE;
ENUM!{enum WICColorContextType {
    WICColorContextUninitialized = 0x00000000,
    WICColorContextProfile = 0x00000001,
    WICColorContextExifColorSpace = 0x00000002,
}}
pub const CODEC_FORCE_DWORD: DWORD = 0x7FFFFFFF;
pub const WIC_JPEG_MAX_COMPONENT_COUNT: UINT = 4;
pub const WIC_JPEG_MAX_TABLE_INDEX: UINT = 3;
pub const WIC_JPEG_SAMPLE_FACTORS_ONE: DWORD = 0x00000011;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_420: DWORD = 0x00111122;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_422: DWORD = 0x00111121;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_440: DWORD = 0x00111112;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_444: DWORD = 0x00111111;
pub const WIC_JPEG_QUANTIZATION_BASELINE_ONE: DWORD = 0x00000000;
pub const WIC_JPEG_QUANTIZATION_BASELINE_THREE: DWORD = 0x00010100;
pub const WIC_JPEG_HUFFMAN_BASELINE_ONE: DWORD = 0x00000000;
pub const WIC_JPEG_HUFFMAN_BASELINE_THREE: DWORD = 0x00111100;
pub type REFWICPixelFormatGUID = REFGUID;
pub type WICPixelFormatGUID = GUID;
DEFINE_GUID!{GUID_WICPixelFormatDontCare,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x00}
DEFINE_GUID!{GUID_WICPixelFormat1bppIndexed,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x01}
DEFINE_GUID!{GUID_WICPixelFormat2bppIndexed,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x02}
DEFINE_GUID!{GUID_WICPixelFormat4bppIndexed,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x03}
DEFINE_GUID!{GUID_WICPixelFormat8bppIndexed,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x04}
DEFINE_GUID!{GUID_WICPixelFormatBlackWhite,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x05}
DEFINE_GUID!{GUID_WICPixelFormat2bppGray,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x06}
DEFINE_GUID!{GUID_WICPixelFormat4bppGray,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x07}
DEFINE_GUID!{GUID_WICPixelFormat8bppGray,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x08}
DEFINE_GUID!{GUID_WICPixelFormat8bppAlpha,
    0xe6cd0116, 0xeeba, 0x4161, 0xaa, 0x85, 0x27, 0xdd, 0x9f, 0xb3, 0xa8, 0x95}
DEFINE_GUID!{GUID_WICPixelFormat16bppBGR555,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x09}
DEFINE_GUID!{GUID_WICPixelFormat16bppBGR565,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x0a}
DEFINE_GUID!{GUID_WICPixelFormat16bppBGRA5551,
    0x05ec7c2b, 0xf1e6, 0x4961, 0xad, 0x46, 0xe1, 0xcc, 0x81, 0x0a, 0x87, 0xd2}
DEFINE_GUID!{GUID_WICPixelFormat16bppGray,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x0b}
DEFINE_GUID!{GUID_WICPixelFormat24bppBGR,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x0c}
DEFINE_GUID!{GUID_WICPixelFormat24bppRGB,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x0d}
DEFINE_GUID!{GUID_WICPixelFormat32bppBGR,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x0e}
DEFINE_GUID!{GUID_WICPixelFormat32bppBGRA,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x0f}
DEFINE_GUID!{GUID_WICPixelFormat32bppPBGRA,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x10}
DEFINE_GUID!{GUID_WICPixelFormat32bppGrayFloat,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x11}
DEFINE_GUID!{GUID_WICPixelFormat32bppRGB,
    0xd98c6b95, 0x3efe, 0x47d6, 0xbb, 0x25, 0xeb, 0x17, 0x48, 0xab, 0x0c, 0xf1}
DEFINE_GUID!{GUID_WICPixelFormat32bppRGBA,
    0xf5c7ad2d, 0x6a8d, 0x43dd, 0xa7, 0xa8, 0xa2, 0x99, 0x35, 0x26, 0x1a, 0xe9}
DEFINE_GUID!{GUID_WICPixelFormat32bppPRGBA,
    0x3cc4a650, 0xa527, 0x4d37, 0xa9, 0x16, 0x31, 0x42, 0xc7, 0xeb, 0xed, 0xba}
DEFINE_GUID!{GUID_WICPixelFormat48bppRGB,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x15}
DEFINE_GUID!{GUID_WICPixelFormat48bppBGR,
    0xe605a384, 0xb468, 0x46ce, 0xbb, 0x2e, 0x36, 0xf1, 0x80, 0xe6, 0x43, 0x13}
DEFINE_GUID!{GUID_WICPixelFormat64bppRGB,
    0xa1182111, 0x186d, 0x4d42, 0xbc, 0x6a, 0x9c, 0x83, 0x03, 0xa8, 0xdf, 0xf9}
DEFINE_GUID!{GUID_WICPixelFormat64bppRGBA,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x16}
DEFINE_GUID!{GUID_WICPixelFormat64bppBGRA,
    0x1562ff7c, 0xd352, 0x46f9, 0x97, 0x9e, 0x42, 0x97, 0x6b, 0x79, 0x22, 0x46}
DEFINE_GUID!{GUID_WICPixelFormat64bppPRGBA,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x17}
DEFINE_GUID!{GUID_WICPixelFormat64bppPBGRA,
    0x8c518e8e, 0xa4ec, 0x468b, 0xae, 0x70, 0xc9, 0xa3, 0x5a, 0x9c, 0x55, 0x30}
DEFINE_GUID!{GUID_WICPixelFormat16bppGrayFixedPoint,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x13}
DEFINE_GUID!{GUID_WICPixelFormat32bppBGR101010,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x14}
DEFINE_GUID!{GUID_WICPixelFormat48bppRGBFixedPoint,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x12}
DEFINE_GUID!{GUID_WICPixelFormat48bppBGRFixedPoint,
    0x49ca140e, 0xcab6, 0x493b, 0x9d, 0xdf, 0x60, 0x18, 0x7c, 0x37, 0x53, 0x2a}
DEFINE_GUID!{GUID_WICPixelFormat96bppRGBFixedPoint,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x18}
DEFINE_GUID!{GUID_WICPixelFormat96bppRGBFloat,
    0xe3fed78f, 0xe8db, 0x4acf, 0x84, 0xc1, 0xe9, 0x7f, 0x61, 0x36, 0xb3, 0x27}
DEFINE_GUID!{GUID_WICPixelFormat128bppRGBAFloat,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x19}
DEFINE_GUID!{GUID_WICPixelFormat128bppPRGBAFloat,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x1a}
DEFINE_GUID!{GUID_WICPixelFormat128bppRGBFloat,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x1b}
DEFINE_GUID!{GUID_WICPixelFormat32bppCMYK,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x1c}
DEFINE_GUID!{GUID_WICPixelFormat64bppRGBAFixedPoint,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x1d}
DEFINE_GUID!{GUID_WICPixelFormat64bppBGRAFixedPoint,
    0x356de33c, 0x54d2, 0x4a23, 0xbb, 0x4, 0x9b, 0x7b, 0xf9, 0xb1, 0xd4, 0x2d}
DEFINE_GUID!{GUID_WICPixelFormat64bppRGBFixedPoint,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x40}
DEFINE_GUID!{GUID_WICPixelFormat128bppRGBAFixedPoint,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x1e}
DEFINE_GUID!{GUID_WICPixelFormat128bppRGBFixedPoint,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x41}
DEFINE_GUID!{GUID_WICPixelFormat64bppRGBAHalf,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x3a}
DEFINE_GUID!{GUID_WICPixelFormat64bppPRGBAHalf,
    0x58ad26c2, 0xc623, 0x4d9d, 0xb3, 0x20, 0x38, 0x7e, 0x49, 0xf8, 0xc4, 0x42}
DEFINE_GUID!{GUID_WICPixelFormat64bppRGBHalf,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x42}
DEFINE_GUID!{GUID_WICPixelFormat48bppRGBHalf,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x3b}
DEFINE_GUID!{GUID_WICPixelFormat32bppRGBE,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x3d}
DEFINE_GUID!{GUID_WICPixelFormat16bppGrayHalf,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x3e}
DEFINE_GUID!{GUID_WICPixelFormat32bppGrayFixedPoint,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x3f}
DEFINE_GUID!{GUID_WICPixelFormat32bppRGBA1010102,
    0x25238D72, 0xFCF9, 0x4522, 0xb5, 0x14, 0x55, 0x78, 0xe5, 0xad, 0x55, 0xe0}
DEFINE_GUID!{GUID_WICPixelFormat32bppRGBA1010102XR,
    0x00DE6B9A, 0xC101, 0x434b, 0xb5, 0x02, 0xd0, 0x16, 0x5e, 0xe1, 0x12, 0x2c}
DEFINE_GUID!{GUID_WICPixelFormat64bppCMYK,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x1f}
DEFINE_GUID!{GUID_WICPixelFormat24bpp3Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x20}
DEFINE_GUID!{GUID_WICPixelFormat32bpp4Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x21}
DEFINE_GUID!{GUID_WICPixelFormat40bpp5Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x22}
DEFINE_GUID!{GUID_WICPixelFormat48bpp6Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x23}
DEFINE_GUID!{GUID_WICPixelFormat56bpp7Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x24}
DEFINE_GUID!{GUID_WICPixelFormat64bpp8Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x25}
DEFINE_GUID!{GUID_WICPixelFormat48bpp3Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x26}
DEFINE_GUID!{GUID_WICPixelFormat64bpp4Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x27}
DEFINE_GUID!{GUID_WICPixelFormat80bpp5Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x28}
DEFINE_GUID!{GUID_WICPixelFormat96bpp6Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x29}
DEFINE_GUID!{GUID_WICPixelFormat112bpp7Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x2a}
DEFINE_GUID!{GUID_WICPixelFormat128bpp8Channels,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x2b}
DEFINE_GUID!{GUID_WICPixelFormat40bppCMYKAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x2c}
DEFINE_GUID!{GUID_WICPixelFormat80bppCMYKAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x2d}
DEFINE_GUID!{GUID_WICPixelFormat32bpp3ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x2e}
DEFINE_GUID!{GUID_WICPixelFormat40bpp4ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x2f}
DEFINE_GUID!{GUID_WICPixelFormat48bpp5ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x30}
DEFINE_GUID!{GUID_WICPixelFormat56bpp6ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x31}
DEFINE_GUID!{GUID_WICPixelFormat64bpp7ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x32}
DEFINE_GUID!{GUID_WICPixelFormat72bpp8ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x33}
DEFINE_GUID!{GUID_WICPixelFormat64bpp3ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x34}
DEFINE_GUID!{GUID_WICPixelFormat80bpp4ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x35}
DEFINE_GUID!{GUID_WICPixelFormat96bpp5ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x36}
DEFINE_GUID!{GUID_WICPixelFormat112bpp6ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x37}
DEFINE_GUID!{GUID_WICPixelFormat128bpp7ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x38}
DEFINE_GUID!{GUID_WICPixelFormat144bpp8ChannelsAlpha,
    0x6fddc324, 0x4e03, 0x4bfe, 0xb1, 0x85, 0x3d, 0x77, 0x76, 0x8d, 0xc9, 0x39}
DEFINE_GUID!{GUID_WICPixelFormat8bppY,
    0x91B4DB54, 0x2DF9, 0x42F0, 0xB4, 0x49, 0x29, 0x09, 0xBB, 0x3D, 0xF8, 0x8E}
DEFINE_GUID!{GUID_WICPixelFormat8bppCb,
    0x1339F224, 0x6BFE, 0x4C3E, 0x93, 0x02, 0xE4, 0xF3, 0xA6, 0xD0, 0xCA, 0x2A}
DEFINE_GUID!{GUID_WICPixelFormat8bppCr,
    0xB8145053, 0x2116, 0x49F0, 0x88, 0x35, 0xED, 0x84, 0x4B, 0x20, 0x5C, 0x51}
DEFINE_GUID!{GUID_WICPixelFormat16bppCbCr,
    0xFF95BA6E, 0x11E0, 0x4263, 0xBB, 0x45, 0x01, 0x72, 0x1F, 0x34, 0x60, 0xA4}
DEFINE_GUID!{GUID_WICPixelFormat16bppYQuantizedDctCoefficients,
    0xA355F433, 0x48E8, 0x4A42, 0x84, 0xD8, 0xE2, 0xAA, 0x26, 0xCA, 0x80, 0xA4}
DEFINE_GUID!{GUID_WICPixelFormat16bppCbQuantizedDctCoefficients,
    0xD2C4FF61, 0x56A5, 0x49C2, 0x8B, 0x5C, 0x4C, 0x19, 0x25, 0x96, 0x48, 0x37}
DEFINE_GUID!{GUID_WICPixelFormat16bppCrQuantizedDctCoefficients,
    0x2FE354F0, 0x1680, 0x42D8, 0x92, 0x31, 0xE7, 0x3C, 0x05, 0x65, 0xBF, 0xC1}
ENUM!{enum WICBitmapCreateCacheOption {
    WICBitmapNoCache = 0x00000000,
    WICBitmapCacheOnDemand = 0x00000001,
    WICBitmapCacheOnLoad = 0x00000002,
    WICBITMAPCREATECACHEOPTION_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICDecodeOptions {
    WICDecodeMetadataCacheOnDemand = 0x00000000,
    WICDecodeMetadataCacheOnLoad = 0x00000001,
    WICMETADATACACHEOPTION_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICBitmapEncoderCacheOption {
    WICBitmapEncoderCacheInMemory = 0x00000000,
    WICBitmapEncoderCacheTempFile = 0x00000001,
    WICBitmapEncoderNoCache = 0x00000002,
    WICBITMAPENCODERCACHEOPTION_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICComponentType {
    WICDecoder = 0x00000001,
    WICEncoder = 0x00000002,
    WICPixelFormatConverter = 0x00000004,
    WICMetadataReader = 0x00000008,
    WICMetadataWriter = 0x00000010,
    WICPixelFormat = 0x00000020,
    WICAllComponents = 0x0000003F,
    WICCOMPONENTTYPE_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICComponentEnumerateOptions {
    WICComponentEnumerateDefault = 0x00000000,
    WICComponentEnumerateRefresh = 0x00000001,
    WICComponentEnumerateDisabled = 0x80000000,
    WICComponentEnumerateUnsigned = 0x40000000,
    WICComponentEnumerateBuiltInOnly = 0x20000000,
    WICCOMPONENTENUMERATEOPTIONS_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
STRUCT!{struct WICBitmapPattern {
    Position: ULARGE_INTEGER,
    Length: ULONG,
    Pattern: *mut BYTE,
    Mask: *mut BYTE,
    EndOfStream: BOOL,
}}
ENUM!{enum WICBitmapInterpolationMode {
    WICBitmapInterpolationModeNearestNeighbor = 0x00000000,
    WICBitmapInterpolationModeLinear = 0x00000001,
    WICBitmapInterpolationModeCubic = 0x00000002,
    WICBitmapInterpolationModeFant = 0x00000003,
    WICBitmapInterpolationModeHighQualityCubic = 0x00000004,
    WICBITMAPINTERPOLATIONMODE_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICBitmapPaletteType {
    WICBitmapPaletteTypeCustom = 0x00000000,
    WICBitmapPaletteTypeMedianCut = 0x00000001,
    WICBitmapPaletteTypeFixedBW = 0x00000002,
    WICBitmapPaletteTypeFixedHalftone8 = 0x00000003,
    WICBitmapPaletteTypeFixedHalftone27 = 0x00000004,
    WICBitmapPaletteTypeFixedHalftone64 = 0x00000005,
    WICBitmapPaletteTypeFixedHalftone125 = 0x00000006,
    WICBitmapPaletteTypeFixedHalftone216 = 0x00000007,
    WICBitmapPaletteTypeFixedWebPalette = WICBitmapPaletteTypeFixedHalftone216,
    WICBitmapPaletteTypeFixedHalftone252 = 0x00000008,
    WICBitmapPaletteTypeFixedHalftone256 = 0x00000009,
    WICBitmapPaletteTypeFixedGray4 = 0x0000000A,
    WICBitmapPaletteTypeFixedGray16 = 0x0000000B,
    WICBitmapPaletteTypeFixedGray256 = 0x0000000C,
    WICBITMAPPALETTETYPE_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICBitmapDitherType {
    WICBitmapDitherTypeNone = 0x00000000,
    WICBitmapDitherTypeSolid = 0x00000000,
    WICBitmapDitherTypeOrdered4x4 = 0x00000001,
    WICBitmapDitherTypeOrdered8x8 = 0x00000002,
    WICBitmapDitherTypeOrdered16x16 = 0x00000003,
    WICBitmapDitherTypeSpiral4x4 = 0x00000004,
    WICBitmapDitherTypeSpiral8x8 = 0x00000005,
    WICBitmapDitherTypeDualSpiral4x4 = 0x00000006,
    WICBitmapDitherTypeDualSpiral8x8 = 0x00000007,
    WICBitmapDitherTypeErrorDiffusion = 0x00000008,
    WICBITMAPDITHERTYPE_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICBitmapAlphaChannelOption {
    WICBitmapUseAlpha = 0x00000000,
    WICBitmapUsePremultipliedAlpha = 0x00000001,
    WICBitmapIgnoreAlpha = 0x00000002,
    WICBITMAPALPHACHANNELOPTIONS_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICBitmapTransformOptions {
    WICBitmapTransformRotate0 = 0x00000000,
    WICBitmapTransformRotate90 = 0x00000001,
    WICBitmapTransformRotate180 = 0x00000002,
    WICBitmapTransformRotate270 = 0x00000003,
    WICBitmapTransformFlipHorizontal = 0x00000008,
    WICBitmapTransformFlipVertical = 0x00000010,
    WICBITMAPTRANSFORMOPTIONS_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICBitmapLockFlags {
    WICBitmapLockRead = 0x00000001,
    WICBitmapLockWrite = 0x00000002,
    WICBITMAPLOCKFLAGS_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICBitmapDecoderCapabilities {
    WICBitmapDecoderCapabilitySameEncoder = 0x00000001,
    WICBitmapDecoderCapabilityCanDecodeAllImages = 0x00000002,
    WICBitmapDecoderCapabilityCanDecodeSomeImages = 0x00000004,
    WICBitmapDecoderCapabilityCanEnumerateMetadata = 0x00000008,
    WICBitmapDecoderCapabilityCanDecodeThumbnail = 0x00000010,
    WICBITMAPDECODERCAPABILITIES_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICProgressOperation {
    WICProgressOperationCopyPixels = 0x00000001,
    WICProgressOperationWritePixels = 0x00000002,
    WICProgressOperationAll = 0x0000FFFF,
    WICPROGRESSOPERATION_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICProgressNotification {
    WICProgressNotificationBegin = 0x00010000,
    WICProgressNotificationEnd = 0x00020000,
    WICProgressNotificationFrequent = 0x00040000,
    WICProgressNotificationAll = 0xFFFF0000,
    WICPROGRESSNOTIFICATION_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICComponentSigning {
    WICComponentSigned = 0x00000001,
    WICComponentUnsigned = 0x00000002,
    WICComponentSafe = 0x00000004,
    WICComponentDisabled = 0x80000000,
    WICCOMPONENTSIGNING_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICGifLogicalScreenDescriptorProperties {
    WICGifLogicalScreenSignature = 0x00000001,
    WICGifLogicalScreenDescriptorWidth = 0x00000002,
    WICGifLogicalScreenDescriptorHeight = 0x00000003,
    WICGifLogicalScreenDescriptorGlobalColorTableFlag = 0x00000004,
    WICGifLogicalScreenDescriptorColorResolution = 0x00000005,
    WICGifLogicalScreenDescriptorSortFlag = 0x00000006,
    WICGifLogicalScreenDescriptorGlobalColorTableSize = 0x00000007,
    WICGifLogicalScreenDescriptorBackgroundColorIndex = 0x00000008,
    WICGifLogicalScreenDescriptorPixelAspectRatio = 0x00000009,
    WICGifLogicalScreenDescriptorProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICGifImageDescriptorProperties {
    WICGifImageDescriptorLeft = 0x00000001,
    WICGifImageDescriptorTop = 0x00000002,
    WICGifImageDescriptorWidth = 0x00000003,
    WICGifImageDescriptorHeight = 0x00000004,
    WICGifImageDescriptorLocalColorTableFlag = 0x00000005,
    WICGifImageDescriptorInterlaceFlag = 0x00000006,
    WICGifImageDescriptorSortFlag = 0x00000007,
    WICGifImageDescriptorLocalColorTableSize = 0x00000008,
    WICGifImageDescriptorProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICGifGraphicControlExtensionProperties {
    WICGifGraphicControlExtensionDisposal = 0x00000001,
    WICGifGraphicControlExtensionUserInputFlag = 0x00000002,
    WICGifGraphicControlExtensionTransparencyFlag = 0x00000003,
    WICGifGraphicControlExtensionDelay = 0x00000004,
    WICGifGraphicControlExtensionTransparentColorIndex = 0x00000005,
    WICGifGraphicControlExtensionProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICGifApplicationExtensionProperties {
    WICGifApplicationExtensionApplication = 0x00000001,
    WICGifApplicationExtensionData = 0x00000002,
    WICGifApplicationExtensionProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICGifCommentExtensionProperties {
    WICGifCommentExtensionText = 0x00000001,
    WICGifCommentExtensionProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICJpegCommentProperties {
    WICJpegCommentText = 0x00000001,
    WICJpegCommentProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICJpegLuminanceProperties {
    WICJpegLuminanceTable = 0x00000001,
    WICJpegLuminanceProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICJpegChrominanceProperties {
    WICJpegChrominanceTable = 0x00000001,
    WICJpegChrominanceProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WIC8BIMIptcProperties {
    WIC8BIMIptcPString = 0x00000000,
    WIC8BIMIptcEmbeddedIPTC = 0x00000001,
    WIC8BIMIptcProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WIC8BIMResolutionInfoProperties {
    WIC8BIMResolutionInfoPString = 0x00000001,
    WIC8BIMResolutionInfoHResolution = 0x00000002,
    WIC8BIMResolutionInfoHResolutionUnit = 0x00000003,
    WIC8BIMResolutionInfoWidthUnit = 0x00000004,
    WIC8BIMResolutionInfoVResolution = 0x00000005,
    WIC8BIMResolutionInfoVResolutionUnit = 0x00000006,
    WIC8BIMResolutionInfoHeightUnit = 0x00000007,
    WIC8BIMResolutionInfoProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WIC8BIMIptcDigestProperties {
    WIC8BIMIptcDigestPString = 0x00000001,
    WIC8BIMIptcDigestIptcDigest = 0x00000002,
    WIC8BIMIptcDigestProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngGamaProperties {
    WICPngGamaGamma = 0x00000001,
    WICPngGamaProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngBkgdProperties {
    WICPngBkgdBackgroundColor = 0x00000001,
    WICPngBkgdProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngItxtProperties {
    WICPngItxtKeyword = 0x00000001,
    WICPngItxtCompressionFlag = 0x00000002,
    WICPngItxtLanguageTag = 0x00000003,
    WICPngItxtTranslatedKeyword = 0x00000004,
    WICPngItxtText = 0x00000005,
    WICPngItxtProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngChrmProperties {
    WICPngChrmWhitePointX = 0x00000001,
    WICPngChrmWhitePointY = 0x00000002,
    WICPngChrmRedX = 0x00000003,
    WICPngChrmRedY = 0x00000004,
    WICPngChrmGreenX = 0x00000005,
    WICPngChrmGreenY = 0x00000006,
    WICPngChrmBlueX = 0x00000007,
    WICPngChrmBlueY = 0x0000008,
    WICPngChrmProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngHistProperties {
    WICPngHistFrequencies = 0x00000001,
    WICPngHistProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngIccpProperties {
    WICPngIccpProfileName = 0x00000001,
    WICPngIccpProfileData = 0x00000002,
    WICPngIccpProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngSrgbProperties {
    WICPngSrgbRenderingIntent = 0x00000001,
    WICPngSrgbProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngTimeProperties {
    WICPngTimeYear = 0x00000001,
    WICPngTimeMonth = 0x00000002,
    WICPngTimeDay = 0x00000003,
    WICPngTimeHour = 0x00000004,
    WICPngTimeMinute = 0x00000005,
    WICPngTimeSecond = 0x00000006,
    WICPngTimeProperties_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICSectionAccessLevel {
    WICSectionAccessLevelRead = 0x00000001,
    WICSectionAccessLevelReadWrite = 0x00000003,
    WICSectionAccessLevel_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPixelFormatNumericRepresentation {
    WICPixelFormatNumericRepresentationUnspecified = 0x00000000,
    WICPixelFormatNumericRepresentationIndexed = 0x00000001,
    WICPixelFormatNumericRepresentationUnsignedInteger = 0x00000002,
    WICPixelFormatNumericRepresentationSignedInteger = 0x00000003,
    WICPixelFormatNumericRepresentationFixed = 0x00000004,
    WICPixelFormatNumericRepresentationFloat = 0x00000005,
    WICPixelFormatNumericRepresentation_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPlanarOptions {
    WICPlanarOptionsDefault = 0x00000000,
    WICPlanarOptionsPreserveSubsampling = 0x00000001,
    WICPLANAROPTIONS_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICJpegIndexingOptions {
    WICJpegIndexingOptionsGenerateOnDemand = 0x00000000,
    WICJpegIndexingOptionsGenerateOnLoad = 0x00000001,
    WICJpegIndexingOptions_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICJpegTransferMatrix {
    WICJpegTransferMatrixIdentity = 0x00000000,
    WICJpegTransferMatrixBT601 = 0x00000001,
    WICJpegTransferMatrix_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICJpegScanType {
    WICJpegScanTypeInterleaved = 0x00000000,
    WICJpegScanTypePlanarComponents = 0x00000001,
    WICJpegScanTypeProgressive = 0x00000002,
    WICJpegScanType_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
STRUCT!{struct WICImageParameters {
    PixelFormat: D2D1_PIXEL_FORMAT,
    DpiX: FLOAT,
    DpiY: FLOAT,
    Top: FLOAT,
    Left: FLOAT,
    PixelWidth: UINT32,
    PixelHeight: UINT32,
}}
STRUCT!{struct WICBitmapPlaneDescription {
    Format: WICPixelFormatGUID,
    Width: UINT,
    Height: UINT,
}}
STRUCT!{struct WICBitmapPlane {
    Format: WICPixelFormatGUID,
    pbBuffer: *mut BYTE,
    cbStride: UINT,
    cbBufferSize: UINT,
}}
STRUCT!{struct WICJpegFrameHeader {
    Width: UINT,
    Height: UINT,
    TransferMatrix: WICJpegTransferMatrix,
    ScanType: WICJpegScanType,
    cComponents: UINT,
    ComponentIdentifiers: DWORD,
    SampleFactors: DWORD,
    QuantizationTableIndices: DWORD,
}}
STRUCT!{struct WICJpegScanHeader {
    cComponents: UINT,
    RestartInterval: UINT,
    ComponentSelectors: DWORD,
    HuffmanTableIndices: DWORD,
    StartSpectralSelection: BYTE,
    EndSpectralSelection: BYTE,
    SuccessiveApproximationHigh: BYTE,
    SuccessiveApproximationLow: BYTE,
}}
RIDL!{#[uuid(0x00000040, 0xa8f2, 0x4877, 0xba, 0x0a, 0xfd, 0x2b, 0x66, 0x45, 0xfb, 0x94)]
interface IWICPalette(IWICPaletteVtbl): IUnknown(IUnknownVtbl) {
    fn InitializePredefined(
        ePaletteType: WICBitmapPaletteType,
        fAddTransparentColor: BOOL,
    ) -> HRESULT,
    fn InitializeCustom(
        pColors: *const WICColor,
        cCount: UINT,
    ) -> HRESULT,
    fn InitializeFromBitmap(
        pISurface: *const IWICBitmapSource,
        cCount: UINT,
        fAddTransparentColor: BOOL,
    ) -> HRESULT,
    fn InitializeFromPalette(
        pIPalette: *const IWICPalette,
    ) -> HRESULT,
    fn GetType(
        pePaletteType: *mut WICBitmapPaletteType,
    ) -> HRESULT,
    fn GetColorCount(
        pcCount: *mut UINT,
    ) -> HRESULT,
    fn GetColors(
        cCount: UINT,
        pColors: *mut WICColor,
        pcActualColors: *mut UINT,
    ) -> HRESULT,
    fn IsBlackWhite(
        pfIsBlackWhite: *mut BOOL,
    ) -> HRESULT,
    fn IsGrayscale(
        pfIsGrayscale: *mut BOOL,
    ) -> HRESULT,
    fn HasAlpha(
        pfHasAlpha: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000120, 0xa8f2, 0x4877, 0xba, 0x0a, 0xfd, 0x2b, 0x66, 0x45, 0xfb, 0x94)]
interface IWICBitmapSource(IWICBitmapSourceVtbl): IUnknown(IUnknownVtbl) {
    fn GetSize(
        puiWidth: *mut UINT,
        puiHeight: *mut UINT,
    ) -> HRESULT,
    fn GetPixelFormat(
        pPixelFormat: *mut WICPixelFormatGUID,
    ) -> HRESULT,
    fn GetResolution(
        pDpiX: *mut c_double,
        pDpiY: *mut c_double,
    ) -> HRESULT,
    fn CopyPalette(
        pIPalette: *mut IWICPalette,
    ) -> HRESULT,
    fn CopyPixels(
        prc: *const WICRect,
        cbStride: UINT,
        cbBufferSize: UINT,
        pbBuffer: *mut BYTE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000301, 0xa8f2, 0x4877, 0xba, 0x0a, 0xfd, 0x2b, 0x66, 0x45, 0xfb, 0x94)]
interface IWICFormatConverter(IWICFormatConverterVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        pISource: *const IWICBitmapSource,
        dstFormat: REFWICPixelFormatGUID,
        dither: WICBitmapDitherType,
        pIPalette: *const IWICPalette,
        alphaThresholdPercent: c_double,
        paletteTranslate: WICBitmapPaletteType,
    ) -> HRESULT,
    fn CanConvert(
        srcPixelFormat: REFWICPixelFormatGUID,
        dstPixelFormat: REFWICPixelFormatGUID,
        pfCanConvert: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbebee9cb, 0x83b0, 0x4dcc, 0x81, 0x32, 0xb0, 0xaa, 0xa5, 0x5e, 0xac, 0x96)]
interface IWICPlanarFormatConverter(IWICPlanarFormatConverterVtbl):
    IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        ppPlanes: *const *const IWICBitmapSource,
        cPlanes: UINT,
        dstFormat: REFWICPixelFormatGUID,
        dither: WICBitmapDitherType,
        pIPalette: *const IWICPalette,
        alphaThresholdPercent: c_double,
        paletteTranslate: WICBitmapPaletteType,
    ) -> HRESULT,
    fn CanConvert(
        pSrcPixelFormats: *const WICPixelFormatGUID,
        cSrcPlanes: UINT,
        dstPixelFormat: REFWICPixelFormatGUID,
        pfCanConvert: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000302, 0xa8f2, 0x4877, 0xba, 0x0a, 0xfd, 0x2b, 0x66, 0x45, 0xfb, 0x94)]
interface IWICBitmapScaler(IWICBitmapScalerVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        pISource: *const IWICBitmapSource,
        uiWidth: UINT,
        uiHeight: UINT,
        mode: WICBitmapInterpolationMode,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe4fbcf03, 0x223d, 0x4e81, 0x93, 0x33, 0xd6, 0x35, 0x55, 0x6d, 0xd1, 0xb5)]
interface IWICBitmapClipper(IWICBitmapClipperVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        pISource: *const IWICBitmapSource,
        prc: *const WICRect,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5009834f, 0x2d6a, 0x41ce, 0x9e, 0x1b, 0x17, 0xc5, 0xaf, 0xf7, 0xa7, 0x82)]
interface IWICBitmapFlipRotator(IWICBitmapFlipRotatorVtbl):
    IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        pISource: *const IWICBitmapSource,
        options: WICBitmapTransformOptions,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000123, 0xa8f2, 0x4877, 0xba, 0x0a, 0xfd, 0x2b, 0x66, 0x45, 0xfb, 0x94)]
interface IWICBitmapLock(IWICBitmapLockVtbl): IUnknown(IUnknownVtbl) {
    fn GetSize(
        puiWidth: *mut UINT,
        puiHeight: *mut UINT,
    ) -> HRESULT,
    fn GetStride(
        pcbStride: *mut UINT,
    ) -> HRESULT,
    fn GetDataPointer(
        pcbBufferSize: *mut UINT,
        ppbData: *mut WICInProcPointer,
    ) -> HRESULT,
    fn GetPixelFormat(
        pPixelFormat: *mut WICPixelFormatGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000121, 0xa8f2, 0x4877, 0xba, 0x0a, 0xfd, 0x2b, 0x66, 0x45, 0xfb, 0x94)]
interface IWICBitmap(IWICBitmapVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Lock(
        prcLock: *const WICRect,
        flags: DWORD,
        ppILock: *mut *mut IWICBitmapLock,
    ) -> HRESULT,
    fn SetPalette(
        pIPalette: *const IWICPalette,
    ) -> HRESULT,
    fn SetResolution(
        dpiX: c_double,
        dpiY: c_double,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3c613a02, 0x34b2, 0x44ea, 0x9a, 0x7c, 0x45, 0xae, 0xa9, 0xc6, 0xfd, 0x6d)]
interface IWICColorContext(IWICColorContextVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeFromFilename(
        wzFilename: LPCWSTR,
    ) -> HRESULT,
    fn InitializeFromMemory(
        pbBuffer: *const BYTE,
        cbBufferSize: UINT,
    ) -> HRESULT,
    fn InitializeFromExifColorSpace(
        value: UINT,
    ) -> HRESULT,
    fn GetType(
        pType: *mut WICColorContextType,
    ) -> HRESULT,
    fn GetProfileBytes(
        cbBuffer: UINT,
        pbBuffer: *mut BYTE,
        pcbActual: *mut UINT,
    ) -> HRESULT,
    fn GetExifColorSpace(
        pValue: *mut UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb66f034f, 0xd0e2, 0x40ab, 0xb4, 0x36, 0x6d, 0xe3, 0x9e, 0x32, 0x1a, 0x94)]
interface IWICColorTransform(IWICColorTransformVtbl): IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn Initialize(
        pIBitmapSource: *const IWICBitmapSource,
        pIContextSource: *const IWICColorContext,
        pIContextDest: *const IWICColorContext,
        pixelFmtDest: REFWICPixelFormatGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb84e2c09, 0x78c9, 0x4ac4, 0x8b, 0xd3, 0x52, 0x4a, 0xe1, 0x66, 0x3a, 0x2f)]
interface IWICFastMetadataEncoder(IWICFastMetadataEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn Commit() -> HRESULT,
    fn GetMetadataQueryWriter(
        ppIMetadataQueryWriter: *mut *mut IWICMetadataQueryWriter,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x135ff860, 0x22b7, 0x4ddf, 0xb0, 0xf6, 0x21, 0x8f, 0x4f, 0x29, 0x9a, 0x43)]
interface IWICStream(IWICStreamVtbl): IStream(IStreamVtbl) {
    fn InitializeFromIStream(
        pIStream: *const IStream,
    ) -> HRESULT,
    fn InitializeFromFilename(
        wzFileName: LPCWSTR,
        dwDesiredAccess: DWORD,
    ) -> HRESULT,
    fn InitializeFromMemory(
        pbBuffer: WICInProcPointer,
        cbBufferSize: DWORD,
    ) -> HRESULT,
    fn InitializeFromIStreamRegion(
        pIStream: *const IStream,
        ulOffset: ULARGE_INTEGER,
        ulMaxSize: ULARGE_INTEGER,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdc2bb46d, 0x3f07, 0x481e, 0x86, 0x25, 0x22, 0x0c, 0x4a, 0xed, 0xbb, 0x33)]
interface IWICEnumMetadataItem(IWICEnumMetadataItemVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgeltSchema: *mut PROPVARIANT,
        rgeltId: *mut PROPVARIANT,
        rgeltValue: *mut PROPVARIANT,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppIEnumMetadataItem: *mut *mut IWICEnumMetadataItem,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x30989668, 0xe1c9, 0x4597, 0xb3, 0x95, 0x45, 0x8e, 0xed, 0xb8, 0x08, 0xdf)]
interface IWICMetadataQueryReader(IWICMetadataQueryReaderVtbl): IUnknown(IUnknownVtbl) {
    fn GetContainerFormat(
        pguidContainerFormat: *mut GUID,
    ) -> HRESULT,
    fn GetLocation(
        cchMaxLength: UINT,
        wzNamespace: *mut WCHAR,
        pcchActualLength: *mut UINT,
    ) -> HRESULT,
    fn GetMetadataByName(
        wzName: LPCWSTR,
        pvarValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetEnumerator(
        ppIEnumString: *mut *mut IEnumString,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa721791a, 0x0def, 0x4d06, 0xbd, 0x91, 0x21, 0x18, 0xbf, 0x1d, 0xb1, 0x0b)]
interface IWICMetadataQueryWriter(IWICMetadataQueryWriterVtbl):
    IWICMetadataQueryReader(IWICMetadataQueryReaderVtbl) {
    fn SetMetadataByName(
        wzName: LPCWSTR,
        pvarValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn RemoveMetadataByName(
        wzName: LPCWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000103, 0xa8f2, 0x4877, 0xba, 0x0a, 0xfd, 0x2b, 0x66, 0x45, 0xfb, 0x94)]
interface IWICBitmapEncoder(IWICBitmapEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pIStream: *const IStream,
        cacheOption: WICBitmapEncoderCacheOption,
    ) -> HRESULT,
    fn GetContainerFormat(
        pguidContainerFormat: *mut GUID,
    ) -> HRESULT,
    fn GetEncoderInfo(
        ppIEncoderInfo: *mut *mut IWICBitmapEncoderInfo,
    ) -> HRESULT,
    fn SetColorContexts(
        cCount: UINT,
        ppIColorContext: *const *const IWICColorContext,
    ) -> HRESULT,
    fn SetPalette(
        pIPalette: *const IWICPalette,
    ) -> HRESULT,
    fn SetThumbnail(
        pIThumbnail: *const IWICBitmapSource,
    ) -> HRESULT,
    fn SetPreview(
        pIPreview: *const IWICBitmapSource,
    ) -> HRESULT,
    fn CreateNewFrame(
        ppIFrameEncode: *mut *mut IWICBitmapFrameEncode,
        ppIEncoderOptions: *mut *mut IPropertyBag2,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
    fn GetMetadataQueryWriter(
        ppIMetadataQueryWriter: *mut *mut IWICMetadataQueryWriter,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000105, 0xa8f2, 0x4877, 0xba, 0x0a, 0xfd, 0x2b, 0x66, 0x45, 0xfb, 0x94)]
interface IWICBitmapFrameEncode(IWICBitmapFrameEncodeVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pIEncoderOptions: *const IPropertyBag2,
    ) -> HRESULT,
    fn SetSize(
        uiWidth: UINT,
        uiHeight: UINT,
    ) -> HRESULT,
    fn SetResolution(
        dpiX: c_double,
        dpiY: c_double,
    ) -> HRESULT,
    fn SetPixelFormat(
        pPixelFormat: *mut WICPixelFormatGUID,
    ) -> HRESULT,
    fn SetColorContexts(
        cCount: UINT,
        ppIColorContext: *const *const IWICColorContext,
    ) -> HRESULT,
    fn SetPalette(
        pIPalette: *const IWICPalette,
    ) -> HRESULT,
    fn SetThumbnail(
        pIThumbnail: *const IWICBitmapSource,
    ) -> HRESULT,
    fn WritePixels(
        lineCount: UINT,
        cbStride: UINT,
        cbBufferSize: UINT,
        pbPixels: *const BYTE,
    ) -> HRESULT,
    fn WriteSource(
        pIBitmapSource: *const IWICBitmapSource,
        prc: *const WICRect,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
    fn GetMetadataQueryWriter(
        ppIMetadataQueryWriter: *mut *mut IWICMetadataQueryWriter,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf928b7b8, 0x2221, 0x40c1, 0xb7, 0x2e, 0x7e, 0x82, 0xf1, 0x97, 0x4d, 0x1a)]
interface IWICPlanarBitmapFrameEncode(IWICPlanarBitmapFrameEncodeVtbl): IUnknown(IUnknownVtbl) {
    fn WritePixels(
        lineCount: UINT,
        pPlanes: *const WICBitmapPlane,
        cPlanes: UINT,
    ) -> HRESULT,
    fn WriteSource(
        ppPlanes: *const *const IWICBitmapSource,
        cPlanes: UINT,
        prcSource: *const WICRect,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x04c75bf8, 0x3ce1, 0x473b, 0xac, 0xc5, 0x3c, 0xc4, 0xf5, 0xe9, 0x49, 0x99)]
interface IWICImageEncoder(IWICImageEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn WriteFrame(
        pImage: *const ID2D1Image,
        pFrameEncode: *const IWICBitmapFrameEncode,
        pImageParameters: *const WICImageParameters,
    ) -> HRESULT,
    fn WriteFrameThumbnail(
        pImage: *const ID2D1Image,
        pFrameEncode: *const IWICBitmapFrameEncode,
        pImageParameters: *const WICImageParameters,
    ) -> HRESULT,
    fn WriteThumbnail(
        pImage: *const ID2D1Image,
        pEncoder: *const IWICBitmapEncoder,
        pImageParameters: *const WICImageParameters,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9edde9e7, 0x8dee, 0x47ea, 0x99, 0xdf, 0xe6, 0xfa, 0xf2, 0xed, 0x44, 0xbf)]
interface IWICBitmapDecoder(IWICBitmapDecoderVtbl): IUnknown(IUnknownVtbl) {
    fn QueryCapability(
        pIStream: *const IStream,
        pdwCapability: *mut DWORD,
    ) -> HRESULT,
    fn Initialize(
        pIStream: *const IStream,
        cacheOptions: WICDecodeOptions,
    ) -> HRESULT,
    fn GetContainerFormat(
        pguidContainerFormat: *mut GUID,
    ) -> HRESULT,
    fn GetDecoderInfo(
        ppIDecoderInfo: *mut *mut IWICBitmapDecoderInfo,
    ) -> HRESULT,
    fn CopyPalette(
        pIPalette: *const IWICPalette,
    ) -> HRESULT,
    fn GetMetadataQueryReader(
        ppIMetadataQueryReader: *mut *mut IWICMetadataQueryReader,
    ) -> HRESULT,
    fn GetPreview(
        ppIBitmapSource: *mut *mut IWICBitmapSource,
    ) -> HRESULT,
    fn GetColorContexts(
        cCount: UINT,
        ppIColorContexts: *mut *mut IWICColorContext,
        pcActualCount: *mut UINT,
    ) -> HRESULT,
    fn GetThumbnail(
        ppIThumbnail: *mut *mut IWICBitmapSource,
    ) -> HRESULT,
    fn GetFrameCount(
        pCount: *mut UINT,
    ) -> HRESULT,
    fn GetFrame(
        index: UINT,
        ppIBitmapFrame: *mut *mut IWICBitmapFrameDecode,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3b16811b, 0x6a43, 0x4ec9, 0xb7, 0x13, 0x3d, 0x5a, 0x0c, 0x13, 0xb9, 0x40)]
interface IWICBitmapSourceTransform(IWICBitmapSourceTransformVtbl): IUnknown(IUnknownVtbl) {
    fn CopyPixels(
        prc: *const WICRect,
        uiWidth: UINT,
        uiHeight: UINT,
        pguidDstFormat: *const WICPixelFormatGUID,
        dstTransform: WICBitmapTransformOptions,
        nStride: UINT,
        cbBufferSize: UINT,
        pbBuffer: *mut BYTE,
    ) -> HRESULT,
    fn GetClosestSize(
        puiWidth: *mut UINT,
        puiHeight: *mut UINT,
    ) -> HRESULT,
    fn GetClosestPixelFormat(
        pguidDstFormat: *mut WICPixelFormatGUID,
    ) -> HRESULT,
    fn DoesSupportTransform(
        dstTransform: WICBitmapTransformOptions,
        pfIsSupported: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3aff9cce, 0xbe95, 0x4303, 0xb9, 0x27, 0xe7, 0xd1, 0x6f, 0xf4, 0xa6, 0x13)]
interface IWICPlanarBitmapSourceTransform(IWICPlanarBitmapSourceTransformVtbl):
    IUnknown(IUnknownVtbl) {
    fn DoesSupportTransform(
        puiWidth: *mut UINT,
        puiHeight: *mut UINT,
        dstTransform: WICBitmapTransformOptions,
        dstPlanarOptions: WICPlanarOptions,
        pguidDstFormats: *const WICPixelFormatGUID,
        pPlaneDescriptions: *mut WICBitmapPlaneDescription,
        cPlanes: UINT,
        pfIsSupported: *mut BOOL,
    ) -> HRESULT,
    fn CopyPixels(
        prcSource: *const WICRect,
        uiWidth: UINT,
        uiHeight: UINT,
        dstTransform: WICBitmapTransformOptions,
        dstPlanarOptions: WICPlanarOptions,
        pDstPlanes: *const WICBitmapPlane,
        cPlanes: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3b16811b, 0x6a43, 0x4ec9, 0xa8, 0x13, 0x3d, 0x93, 0x0c, 0x13, 0xb9, 0x40)]
interface IWICBitmapFrameDecode(IWICBitmapFrameDecodeVtbl):
    IWICBitmapSource(IWICBitmapSourceVtbl) {
    fn GetMetadataQueryReader(
        ppIMetadataQueryReader: *mut *mut IWICMetadataQueryReader,
    ) -> HRESULT,
    fn GetColorContexts(
        cCount: UINT,
        ppIColorContexts: *mut *mut IWICColorContext,
        pcActualCount: *mut UINT,
    ) -> HRESULT,
    fn GetThumbnail(
        ppIThumbnail: *mut *mut IWICBitmapSource,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdaac296f, 0x7aa5, 0x4dbf, 0x8d, 0x15, 0x22, 0x5c, 0x59, 0x76, 0xf8, 0x91)]
interface IWICProgressiveLevelControl(IWICProgressiveLevelControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetLevelCount(
        pcLevels: *mut UINT,
    ) -> HRESULT,
    fn GetCurrentLevel(
        pnLevel: *mut UINT,
    ) -> HRESULT,
    fn SetCurrentLevel(
        nLevel: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4776f9cd, 0x9517, 0x45fa, 0xbf, 0x24, 0xe8, 0x9c, 0x5e, 0xc5, 0xc6, 0x0c)]
interface IWICProgressCallback(IWICProgressCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Notify(
        uFrameNum: ULONG,
        operation: WICProgressOperation,
        dblProgress: c_double,
    ) -> HRESULT,
}}
FN!{stdcall PFNProgressNotification(
    pvData: LPVOID,
    uFrameNum: ULONG,
    operation: WICProgressOperation,
    dblProgress: c_double,
) -> HRESULT}
RIDL!{#[uuid(0x64c1024e, 0xc3cf, 0x4462, 0x80, 0x78, 0x88, 0xc2, 0xb1, 0x1c, 0x46, 0xd9)]
interface IWICBitmapCodecProgressNotification(IWICBitmapCodecProgressNotificationVtbl):
    IUnknown(IUnknownVtbl) {
    fn RegisterProgressNotification(
        pfnProgressNotification: PFNProgressNotification,
        pvData: LPVOID,
        dwProgressFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x23bc3f0a, 0x698b, 0x4357, 0x88, 0x6b, 0xf2, 0x4d, 0x50, 0x67, 0x13, 0x34)]
interface IWICComponentInfo(IWICComponentInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetComponentType(
        pType: *mut WICComponentType,
    ) -> HRESULT,
    fn GetCLSID(
        pclsid: *mut CLSID,
    ) -> HRESULT,
    fn GetSigningStatus(
        pStatus: *mut DWORD,
    ) -> HRESULT,
    fn GetAuthor(
        cchAuthor: UINT,
        wzAuthor: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
    fn GetVendorGUID(
        pguidVendor: *mut GUID,
    ) -> HRESULT,
    fn GetVersion(
        cchVersion: UINT,
        wzVersion: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
    fn GetSpecVersion(
        cchSpecVersion: UINT,
        wzSpecVersion: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
    fn GetFriendlyName(
        cchFriendlyName: UINT,
        wzFriendlyName: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9f34fb65, 0x13f4, 0x4f15, 0xbc, 0x57, 0x37, 0x26, 0xb5, 0xe5, 0x3d, 0x9f)]
interface IWICFormatConverterInfo(IWICFormatConverterInfoVtbl):
    IWICComponentInfo(IWICComponentInfoVtbl) {
    fn GetPixelFormats(
        cFormats: UINT,
        pPixelFormatGUIDs: *mut WICPixelFormatGUID,
        pcActual: *mut UINT,
    ) -> HRESULT,
    fn CreateInstance(
        ppIConverter: *mut *mut IWICFormatConverter,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe87a44c4, 0xb76e, 0x4c47, 0x8b, 0x09, 0x29, 0x8e, 0xb1, 0x2a, 0x27, 0x14)]
interface IWICBitmapCodecInfo(IWICBitmapCodecInfoVtbl): IWICComponentInfo(IWICComponentInfoVtbl) {
    fn GetContainerFormat(
        pguidContainerFormat: *mut GUID,
    ) -> HRESULT,
    fn GetPixelFormats(
        cFormats: UINT,
        pguidPixelFormats: *mut GUID,
        pcActual: *mut UINT,
    ) -> HRESULT,
    fn GetColorManagementVersion(
        cchColorManagementVersion: UINT,
        wzColorManagementVersion: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
    fn GetDeviceManufacturer(
        cchDeviceManufacturer: UINT,
        wzDeviceManufacturer: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
    fn GetDeviceModels(
        cchDeviceModels: UINT,
        wzDeviceModels: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
    fn GetMimeTypes(
        cchMimeTypes: UINT,
        wzMimeTypes: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
    fn GetFileExtensions(
        cchFileExtensions: UINT,
        wzFileExtensions: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT,
    fn DoesSupportAnimation(
        pfSupportAnimation: *mut BOOL,
    ) -> HRESULT,
    fn DoesSupportChromakey(
        pfSupportChromakey: *mut BOOL,
    ) -> HRESULT,
    fn DoesSupportLossless(
        pfSupportLossless: *mut BOOL,
    ) -> HRESULT,
    fn DoesSupportMultiframe(
        pfSupportMultiframe: *mut BOOL,
    ) -> HRESULT,
    fn MatchesMimeType(
        wzMimeType: LPCWSTR,
        pfMatches: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x94c9b4ee, 0xa09f, 0x4f92, 0x8a, 0x1e, 0x4a, 0x9b, 0xce, 0x7e, 0x76, 0xfb)]
interface IWICBitmapEncoderInfo(IWICBitmapEncoderInfoVtbl):
    IWICBitmapCodecInfo(IWICBitmapCodecInfoVtbl) {
    fn CreateInstance(
        ppIBitmapEncoder: *mut *mut IWICBitmapEncoder,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd8cd007f, 0xd08f, 0x4191, 0x9b, 0xfc, 0x23, 0x6e, 0xa7, 0xf0, 0xe4, 0xb5)]
interface IWICBitmapDecoderInfo(IWICBitmapDecoderInfoVtbl):
    IWICBitmapCodecInfo(IWICBitmapCodecInfoVtbl) {
    fn GetPatterns(
        cbSizePatterns: UINT,
        pPatterns: *mut WICBitmapPattern,
        pcPatterns: *mut UINT,
        pcbPatternsActual: *mut UINT,
    ) -> HRESULT,
    fn MatchesPattern(
        pIStream: *const IStream,
        pfMatches: *mut BOOL,
    ) -> HRESULT,
    fn CreateInstance(
        ppIBitmapDecoder: *mut *mut IWICBitmapDecoder,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe8eda601, 0x3d48, 0x431a, 0xab, 0x44, 0x69, 0x05, 0x9b, 0xe8, 0x8b, 0xbe)]
interface IWICPixelFormatInfo(IWICPixelFormatInfoVtbl): IWICComponentInfo(IWICComponentInfoVtbl) {
    fn GetFormatGUID(
        pFormat: *mut GUID,
    ) -> HRESULT,
    fn GetColorContext(
        ppIColorContext: *mut *mut IWICColorContext,
    ) -> HRESULT,
    fn GetBitsPerPixel(
        puiBitsPerPixel: *mut UINT,
    ) -> HRESULT,
    fn GetChannelCount(
        puiChannelCount: *mut UINT,
    ) -> HRESULT,
    fn GetChannelMask(
        uiChannelIndex: UINT,
        cbMaskBuffer: UINT,
        pbMaskBuffer: *mut BYTE,
        pcbActual: *mut UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa9db33a2, 0xaf5f, 0x43c7, 0xb6, 0x79, 0x74, 0xf5, 0x98, 0x4b, 0x5a, 0xa4)]
interface IWICPixelFormatInfo2(IWICPixelFormatInfo2Vtbl):
    IWICPixelFormatInfo(IWICPixelFormatInfoVtbl) {
    fn SupportsTransparency(
        pfSupportsTransparency: *mut BOOL,
    ) -> HRESULT,
    fn GetNumericRepresentation(
        pNumericRepresentation: *mut WICPixelFormatNumericRepresentation,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xec5ec8a9, 0xc395, 0x4314, 0x9c, 0x77, 0x54, 0xd7, 0xa9, 0x35, 0xff, 0x70)]
interface IWICImagingFactory(IWICImagingFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateDecoderFromFilename(
        wzFilename: LPCWSTR,
        pguidVendor: *const GUID,
        dwDesiredAccess: DWORD,
        metadataOptions: WICDecodeOptions,
        ppIDecoder: *mut *mut IWICBitmapDecoder,
    ) -> HRESULT,
    fn CreateDecoderFromStream(
        pIStream: *const IStream,
        pguidVendor: *const GUID,
        metadataOptions: WICDecodeOptions,
        ppIDecoder: *mut *mut IWICBitmapDecoder,
    ) -> HRESULT,
    fn CreateDecoderFromFileHandle(
        hFile: ULONG_PTR,
        pguidVendor: *const GUID,
        metadataOptions: WICDecodeOptions,
        ppIDecoder: *mut *mut IWICBitmapDecoder,
    ) -> HRESULT,
    fn CreateComponentInfo(
        clsidComponent: REFCLSID,
        ppIInfo: *mut *mut IWICComponentInfo,
    ) -> HRESULT,
    fn CreateDecoder(
        guidContainerFormat: REFGUID,
        pguidVendor: *const GUID,
        ppIDecoder: *mut *mut IWICBitmapDecoder,
    ) -> HRESULT,
    fn CreateEncoder(
        guidContainerFormat: REFGUID,
        pguidVendor: *const GUID,
        ppIEncoder: *mut *mut IWICBitmapEncoder,
    ) -> HRESULT,
    fn CreatePalette(
        ppIPalette: *mut *mut IWICPalette,
    ) -> HRESULT,
    fn CreateFormatConverter(
        ppIFormatConverter: *mut *mut IWICFormatConverter,
    ) -> HRESULT,
    fn CreateBitmapScaler(
        ppIBitmapScaler: *mut *mut IWICBitmapScaler,
    ) -> HRESULT,
    fn CreateBitmapClipper(
        ppIBitmapClipper: *mut *mut IWICBitmapClipper,
    ) -> HRESULT,
    fn CreateBitmapFlipRotator(
        ppIBitmapFlipRotator: *mut *mut IWICBitmapFlipRotator,
    ) -> HRESULT,
    fn CreateStream(
        ppIWICStream: *mut *mut IWICStream,
    ) -> HRESULT,
    fn CreateColorContext(
        ppIWICColorContext: *mut *mut IWICColorContext,
    ) -> HRESULT,
    fn CreateColorTransformer(
        ppIWICColorTransform: *mut *mut IWICColorTransform,
    ) -> HRESULT,
    fn CreateBitmap(
        uiWidth: UINT,
        uiHeight: UINT,
        pixelFormat: REFWICPixelFormatGUID,
        option: WICBitmapCreateCacheOption,
        ppIBitmap: *mut *mut IWICBitmap,
    ) -> HRESULT,
    fn CreateBitmapFromSource(
        pIBitmapSource: *const IWICBitmapSource,
        option: WICBitmapCreateCacheOption,
        ppIBitmap: *mut *mut IWICBitmap,
    ) -> HRESULT,
    fn CreateBitmapFromSourceRect(
        pIBitmapSource: *const IWICBitmapSource,
        x: UINT,
        y: UINT,
        width: UINT,
        height: UINT,
        ppIBitmap: *mut *mut IWICBitmap,
    ) -> HRESULT,
    fn CreateBitmapFromMemory(
        uiWidth: UINT,
        uiHeight: UINT,
        pixelFormat: REFWICPixelFormatGUID,
        cbStride: UINT,
        cbBufferSize: UINT,
        pbBuffer: *const BYTE,
        ppIBitmap: *mut *mut IWICBitmap,
    ) -> HRESULT,
    fn CreateBitmapFromHBITMAP(
        hBitmap: HBITMAP,
        hPalette: HPALETTE,
        options: WICBitmapAlphaChannelOption,
        ppIBitmap: *mut *mut IWICBitmap,
    ) -> HRESULT,
    fn CreateBitmapFromHICON(
        hIcon: HICON,
        ppIBitmap: *mut *mut IWICBitmap,
    ) -> HRESULT,
    fn CreateComponentEnumerator(
        componentTypes: DWORD,
        options: DWORD,
        ppIEnumUnknown: *mut *mut IEnumUnknown,
    ) -> HRESULT,
    fn CreateFastMetadataEncoderFromDecoder(
        pIDecoder: *const IWICBitmapDecoder,
        ppIFastEncoder: *mut *mut IWICFastMetadataEncoder,
    ) -> HRESULT,
    fn CreateFastMetadataEncoderFromFrameDecode(
        pIFrameDecoder: *const IWICBitmapFrameDecode,
        ppIFastEncoder: *mut *mut IWICFastMetadataEncoder,
    ) -> HRESULT,
    fn CreateQueryWriter(
        guidMetadataFormat: REFGUID,
        pguidVendor: *const GUID,
        ppIQueryWriter: *mut *mut IWICMetadataQueryWriter,
    ) -> HRESULT,
    fn CreateQueryWriterFromReader(
        pIQueryReader: *const IWICMetadataQueryReader,
        pguidVendor: *const GUID,
        ppIQueryWriter: *mut *mut IWICMetadataQueryWriter,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7b816b45, 0x1996, 0x4476, 0xb1, 0x32, 0xde, 0x9e, 0x24, 0x7c, 0x8a, 0xf0)]
interface IWICImagingFactory2(IWICImagingFactory2Vtbl):
    IWICImagingFactory(IWICImagingFactoryVtbl) {
    fn CreateImageEncoder(
        pD2DDevice: *const ID2D1Device,
        ppWICImageEncoder: *mut *mut IWICImageEncoder,
    ) -> HRESULT,
}}
extern "system" {
    pub fn WICConvertBitmapSource(
        dstFormat: REFWICPixelFormatGUID,
        pISrc: *const IWICBitmapSource,
        ppIDst: *mut *mut IWICBitmapSource,
    ) -> HRESULT;
    pub fn WICCreateBitmapFromSection(
        width: UINT,
        height: UINT,
        pixelFormat: REFWICPixelFormatGUID,
        hSection: HANDLE,
        stride: UINT,
        offset: UINT,
        ppIBitmap: *mut *mut IWICBitmap,
    ) -> HRESULT;
    pub fn WICCreateBitmapFromSectionEx(
        width: UINT,
        height: UINT,
        pixelFormat: REFWICPixelFormatGUID,
        hSection: HANDLE,
        stride: UINT,
        offset: UINT,
        desiredAccessLevel: WICSectionAccessLevel,
        ppIBitmap: *mut *mut IWICBitmap,
    ) -> HRESULT;
    pub fn WICMapGuidToShortName(
        guid: REFGUID,
        cchName: UINT,
        wzName: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT;
    pub fn WICMapShortNameToGuid(
        wzName: PCWSTR,
        pguid: *mut GUID,
    ) -> HRESULT;
    pub fn WICMapSchemaToName(
        guidMetadataFormat: REFGUID,
        pwzSchema: LPWSTR,
        cchName: UINT,
        wzName: *mut WCHAR,
        pcchActual: *mut UINT,
    ) -> HRESULT;
}
pub const FACILITY_WINCODEC_ERR: HRESULT = 0x898;
pub const WINCODEC_ERR_BASE: HRESULT = 0x2000;
/// intsafe.h, 0x216 = 534 = ERROR_ARITHMETIC_OVERFLOW
pub const INTSAFE_E_ARITHMETIC_OVERFLOW: HRESULT = 0x80070216;
#[inline]
pub fn MAKE_WINCODECHR(severity: HRESULT, code: HRESULT) -> HRESULT {
    MAKE_HRESULT!(severity, FACILITY_WINCODEC_ERR, WINCODEC_ERR_BASE + code)
}
#[inline]
pub fn MAKE_WINCODECHR_ERR(code: HRESULT) -> HRESULT {
    MAKE_WINCODECHR(SEVERITY_ERROR, code)
}
pub const WINCODEC_ERR_GENERIC_ERROR: HRESULT = E_FAIL;
pub const WINCODEC_ERR_INVALIDPARAMETER: HRESULT = E_INVALIDARG;
pub const WINCODEC_ERR_OUTOFMEMORY: HRESULT = E_OUTOFMEMORY;
pub const WINCODEC_ERR_NOTIMPLEMENTED: HRESULT = E_NOTIMPL;
pub const WINCODEC_ERR_ABORTED: HRESULT = E_ABORT;
pub const WINCODEC_ERR_ACCESSDENIED: HRESULT = E_ACCESSDENIED;
pub const WINCODEC_ERR_VALUEOVERFLOW: HRESULT = INTSAFE_E_ARITHMETIC_OVERFLOW;
ENUM!{enum WICTiffCompressionOption {
    WICTiffCompressionDontCare = 0x00000000,
    WICTiffCompressionNone = 0x00000001,
    WICTiffCompressionCCITT3 = 0x00000002,
    WICTiffCompressionCCITT4 = 0x00000003,
    WICTiffCompressionLZW = 0x00000004,
    WICTiffCompressionRLE = 0x00000005,
    WICTiffCompressionZIP = 0x00000006,
    WICTiffCompressionLZWHDifferencing = 0x00000007,
    WICTIFFCOMPRESSIONOPTION_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICJpegYCrCbSubsamplingOption {
    WICJpegYCrCbSubsamplingDefault = 0x00000000,
    WICJpegYCrCbSubsampling420 = 0x00000001,
    WICJpegYCrCbSubsampling422 = 0x00000002,
    WICJpegYCrCbSubsampling444 = 0x00000003,
    WICJpegYCrCbSubsampling440 = 0x00000004,
    WICJPEGYCRCBSUBSAMPLING_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICPngFilterOption {
    WICPngFilterUnspecified = 0x00000000,
    WICPngFilterNone = 0x00000001,
    WICPngFilterSub = 0x00000002,
    WICPngFilterUp = 0x00000003,
    WICPngFilterAverage = 0x00000004,
    WICPngFilterPaeth = 0x00000005,
    WICPngFilterAdaptive = 0x00000006,
    WICPNGFILTEROPTION_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICNamedWhitePoint {
    WICWhitePointDefault = 0x00000001,
    WICWhitePointDaylight = 0x00000002,
    WICWhitePointCloudy = 0x00000004,
    WICWhitePointShade = 0x00000008,
    WICWhitePointTungsten = 0x00000010,
    WICWhitePointFluorescent = 0x00000020,
    WICWhitePointFlash = 0x00000040,
    WICWhitePointUnderwater = 0x00000080,
    WICWhitePointCustom = 0x00000100,
    WICWhitePointAutoWhiteBalance = 0x00000200,
    WICWhitePointAsShot = WICWhitePointDefault,
    WICNAMEDWHITEPOINT_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICRawCapabilities {
    WICRawCapabilityNotSupported = 0x00000000,
    WICRawCapabilityGetSupported = 0x00000001,
    WICRawCapabilityFullySupported = 0x00000002,
    WICRAWCAPABILITIES_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICRawRotationCapabilities {
    WICRawRotationCapabilityNotSupported = 0x00000000,
    WICRawRotationCapabilityGetSupported = 0x00000001,
    WICRawRotationCapabilityNinetyDegreesSupported = 0x00000002,
    WICRawRotationCapabilityFullySupported = 0x00000003,
    WICRAWROTATIONCAPABILITIES_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
STRUCT!{struct WICRawCapabilitiesInfo {
    cbSize: UINT,
    CodecMajorVersion: UINT,
    CodecMinorVersion: UINT,
    ExposureCompensationSupport: WICRawCapabilities,
    ContrastSupport: WICRawCapabilities,
    RGBWhitePointSupport: WICRawCapabilities,
    NamedWhitePointSupport: WICRawCapabilities,
    NamedWhitePointSupportMask: UINT,
    KelvinWhitePointSupport: WICRawCapabilities,
    GammaSupport: WICRawCapabilities,
    TintSupport: WICRawCapabilities,
    SaturationSupport: WICRawCapabilities,
    SharpnessSupport: WICRawCapabilities,
    NoiseReductionSupport: WICRawCapabilities,
    DestinationColorProfileSupport: WICRawCapabilities,
    ToneCurveSupport: WICRawCapabilities,
    RotationSupport: WICRawRotationCapabilities,
    RenderModeSupport: WICRawCapabilities,
}}
ENUM!{enum WICRawParameterSet {
    WICAsShotParameterSet = 0x00000001,
    WICUserAdjustedParameterSet = 0x00000002,
    WICAutoAdjustedParameterSet = 0x00000003,
    WICRAWPARAMETERSET_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICRawRenderMode {
    WICRawRenderModeDraft = 0x00000001,
    WICRawRenderModeNormal = 0x00000002,
    WICRawRenderModeBestQuality = 0x00000003,
    WICRAWRENDERMODE_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
STRUCT!{struct WICRawToneCurvePoint {
    Input: c_double,
    Output: c_double,
}}
STRUCT!{struct WICRawToneCurve {
    cPoints: UINT,
    aPoints: [WICRawToneCurvePoint; 1],
}}
pub const WICRawChangeNotification_ExposureCompensation: UINT = 0x00000001;
pub const WICRawChangeNotification_NamedWhitePoint: UINT = 0x00000002;
pub const WICRawChangeNotification_KelvinWhitePoint: UINT = 0x00000004;
pub const WICRawChangeNotification_RGBWhitePoint: UINT = 0x00000008;
pub const WICRawChangeNotification_Contrast: UINT = 0x00000010;
pub const WICRawChangeNotification_Gamma: UINT = 0x00000020;
pub const WICRawChangeNotification_Sharpness: UINT = 0x00000040;
pub const WICRawChangeNotification_Saturation: UINT = 0x00000080;
pub const WICRawChangeNotification_Tint: UINT = 0x00000100;
pub const WICRawChangeNotification_NoiseReduction: UINT = 0x00000200;
pub const WICRawChangeNotification_DestinationColorContext: UINT = 0x00000400;
pub const WICRawChangeNotification_ToneCurve: UINT = 0x00000800;
pub const WICRawChangeNotification_Rotation: UINT = 0x00001000;
pub const WICRawChangeNotification_RenderMode: UINT = 0x00002000;
RIDL!{#[uuid(0x95c75a6e, 0x3e8c, 0x4ec2, 0x85, 0xa8, 0xae, 0xbc, 0xc5, 0x51, 0xe5, 0x9b)]
interface IWICDevelopRawNotificationCallback(IWICDevelopRawNotificationCallbackVtbl):
    IUnknown(IUnknownVtbl) {
    fn Notify(
        NotificationMask: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfbec5e44, 0xf7be, 0x4b65, 0xb7, 0xf8, 0xc0, 0xc8, 0x1f, 0xef, 0x02, 0x6d)]
interface IWICDevelopRaw(IWICDevelopRawVtbl): IWICBitmapFrameDecode(IWICBitmapFrameDecodeVtbl) {
    fn QueryRawCapabilitiesInfo(
        pInfo: *mut WICRawCapabilitiesInfo,
    ) -> HRESULT,
    fn LoadParameterSet(
        ParameterSet: WICRawParameterSet,
    ) -> HRESULT,
    fn GetCurrentParameterSet(
        ppCurrentParameterSet: *mut *mut IPropertyBag2,
    ) -> HRESULT,
    fn SetExposureCompensation(
        ev: c_double,
    ) -> HRESULT,
    fn GetExposureCompensation(
        pEV: *mut c_double,
    ) -> HRESULT,
    fn SetWhitePointRGB(
        Red: UINT,
        Green: UINT,
        Blue: UINT,
    ) -> HRESULT,
    fn GetWhitePointRGB(
        pRed: *mut UINT,
        pGreen: *mut UINT,
        pBlue: *mut UINT,
    ) -> HRESULT,
    fn SetNamedWhitePoint(
        WhitePoint: WICNamedWhitePoint,
    ) -> HRESULT,
    fn GetNamedWhitePoint(
        pWhitePoint: *mut WICNamedWhitePoint,
    ) -> HRESULT,
    fn SetWhitePointKelvin(
        WhitePointKelvin: UINT,
    ) -> HRESULT,
    fn GetWhitePointKelvin(
        pWhitePointKelvin: *mut UINT,
    ) -> HRESULT,
    fn GetKelvinRangeInfo(
        pMinKelvinTemp: *mut UINT,
        pMaxKelvinTemp: *mut UINT,
        pKelvinTempStepValue: *mut UINT,
    ) -> HRESULT,
    fn SetContrast(
        Contrast: c_double,
    ) -> HRESULT,
    fn GetContrast(
        pContrast: *mut c_double,
    ) -> HRESULT,
    fn SetGamma(
        Gamma: c_double,
    ) -> HRESULT,
    fn GetGamma(
        pGamma: *mut c_double,
    ) -> HRESULT,
    fn SetSharpness(
        Sharpness: c_double,
    ) -> HRESULT,
    fn GetSharpness(
        pSharpness: *mut c_double,
    ) -> HRESULT,
    fn SetSaturation(
        Saturation: c_double,
    ) -> HRESULT,
    fn GetSaturation(
        pSaturation: *mut c_double,
    ) -> HRESULT,
    fn SetTint(
        Tint: c_double,
    ) -> HRESULT,
    fn GetTint(
        pTint: *mut c_double,
    ) -> HRESULT,
    fn SetNoiseReduction(
        NoiseReduction: c_double,
    ) -> HRESULT,
    fn GetNoiseReduction(
        pNoiseReduction: *mut c_double,
    ) -> HRESULT,
    fn SetDestinationColorContext(
        pColorContext: *const IWICColorContext,
    ) -> HRESULT,
    fn SetToneCurve(
        cbToneCurveSize: UINT,
        pToneCurve: *const WICRawToneCurve,
    ) -> HRESULT,
    fn GetToneCurve(
        cbToneCurveBufferSize: UINT,
        pToneCurve: *mut WICRawToneCurve,
        pcbActualToneCurveBufferSize: *mut UINT,
    ) -> HRESULT,
    fn SetRotation(
        Rotation: c_double,
    ) -> HRESULT,
    fn GetRotation(
        pRotation: *mut c_double,
    ) -> HRESULT,
    fn SetRenderMode(
        RenderMode: WICRawRenderMode,
    ) -> HRESULT,
    fn GetRenderMode(
        pRenderMode: *mut WICRawRenderMode,
    ) -> HRESULT,
    fn SetNotificationCallback(
        pCallback: *const IWICDevelopRawNotificationCallback,
    ) -> HRESULT,
}}
ENUM!{enum WICDdsDimension {
    WICDdsTexture1D = 0x00000000,
    WICDdsTexture2D = 0x00000001,
    WICDdsTexture3D = 0x00000002,
    WICDdsTextureCube = 0x00000003,
    WICDDSTEXTURE_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
ENUM!{enum WICDdsAlphaMode {
    WICDdsAlphaModeUnknown = 0x00000000,
    WICDdsAlphaModeStraight = 0x00000001,
    WICDdsAlphaModePremultiplied = 0x00000002,
    WICDdsAlphaModeOpaque = 0x00000003,
    WICDdsAlphaModeCustom = 0x00000004,
    WICDDSALPHAMODE_FORCE_DWORD = CODEC_FORCE_DWORD,
}}
STRUCT!{struct WICDdsParameters {
    Width: UINT,
    Height: UINT,
    Depth: UINT,
    MipLevels: UINT,
    ArraySize: UINT,
    DxgiFormat: DXGI_FORMAT,
    Dimension: WICDdsDimension,
    AlphaMode: WICDdsAlphaMode,
}}
RIDL!{#[uuid(0x409cd537, 0x8532, 0x40cb, 0x97, 0x74, 0xe2, 0xfe, 0xb2, 0xdf, 0x4e, 0x9c)]
interface IWICDdsDecoder(IWICDdsDecoderVtbl): IUnknown(IUnknownVtbl) {
    fn GetParameters(
        pParameters: *mut WICDdsParameters,
    ) -> HRESULT,
    fn GetFrame(
        arrayIndex: UINT,
        mipLevel: UINT,
        sliceIndex: UINT,
        ppIBitmapFrame: *mut *mut IWICBitmapFrameDecode,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5cacdb4c, 0x407e, 0x41b3, 0xb9, 0x36, 0xd0, 0xf0, 0x10, 0xcd, 0x67, 0x32)]
interface IWICDdsEncoder(IWICDdsEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn SetParameters(
        pParameters: *const WICDdsParameters,
    ) -> HRESULT,
    fn GetParameters(
        pParameters: *mut WICDdsParameters,
    ) -> HRESULT,
    fn CreateNewFrame(
        ppIFrameEncode: *mut *mut IWICBitmapFrameEncode,
        pArrayIndex: *mut UINT,
        pMipLevel: *mut UINT,
        pSliceIndex: *mut UINT,
    ) -> HRESULT,
}}
STRUCT!{struct WICDdsFormatInfo {
    DxgiFormat: DXGI_FORMAT,
    BytesPerBlock: UINT,
    BlockWidth: UINT,
    BlockHeight: UINT,
}}
RIDL!{#[uuid(0x3d4c0c61, 0x18a4, 0x41e4, 0xbd, 0x80, 0x48, 0x1a, 0x4f, 0xc9, 0xf4, 0x64)]
interface IWICDdsFrameDecode(IWICDdsFrameDecodeVtbl): IUnknown(IUnknownVtbl) {
    fn GetSizeInBlocks(
        pWidthInBlocks: *mut UINT,
        pHeightInBlocks: *mut UINT,
    ) -> HRESULT,
    fn GetFormatInfo(
        pFormatInfo: *mut WICDdsFormatInfo,
    ) -> HRESULT,
    fn CopyBlocks(
        prcBoundsInBlocks: *const WICRect,
        cbStride: UINT,
        cbBufferSize: UINT,
        pbBuffer: *mut BYTE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8939f66e, 0xc46a, 0x4c21, 0xa9, 0xd1, 0x98, 0xb3, 0x27, 0xce, 0x16, 0x79)]
interface IWICJpegFrameDecode(IWICJpegFrameDecodeVtbl): IUnknown(IUnknownVtbl) {
    fn DoesSupportIndexing(
        pfIndexingSupported: *mut BOOL,
    ) -> HRESULT,
    fn SetIndexing(
        options: WICJpegIndexingOptions,
        horizontalIntervalSize: UINT,
    ) -> HRESULT,
    fn ClearIndexing() -> HRESULT,
    fn GetAcHuffmanTable(
        scanIndex: UINT,
        tableIndex: UINT,
        pAcHuffmanTable: *mut DXGI_JPEG_AC_HUFFMAN_TABLE,
    ) -> HRESULT,
    fn GetDcHuffmanTable(
        scanIndex: UINT,
        tableIndex: UINT,
        pDcHuffmanTable: *mut DXGI_JPEG_DC_HUFFMAN_TABLE,
    ) -> HRESULT,
    fn GetQuantizationTable(
        scanIndex: UINT,
        tableIndex: UINT,
        pQuantizationTable: *mut DXGI_JPEG_QUANTIZATION_TABLE,
    ) -> HRESULT,
    fn GetFrameHeader(
        pFrameHeader: *mut WICJpegFrameHeader,
    ) -> HRESULT,
    fn GetScanHeader(
        scanIndex: UINT,
        pScanHeader: *mut WICJpegScanHeader,
    ) -> HRESULT,
    fn CopyScan(
        scanIndex: UINT,
        scanOffset: UINT,
        cbScanData: UINT,
        pbScanData: *mut BYTE,
        pcbScanDataActual: *mut UINT,
    ) -> HRESULT,
    fn CopyMinimalStream(
        streamOffset: UINT,
        cbStreamData: UINT,
        pbStreamData: *mut BYTE,
        pcbStreamDataActual: *mut UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2f0c601f, 0xd2c6, 0x468c, 0xab, 0xfa, 0x49, 0x49, 0x5d, 0x98, 0x3e, 0xd1)]
interface IWICJpegFrameEncode(IWICJpegFrameEncodeVtbl): IUnknown(IUnknownVtbl) {
    fn GetAcHuffmanTable(
        scanIndex: UINT,
        tableIndex: UINT,
        pAcHuffmanTable: *mut DXGI_JPEG_AC_HUFFMAN_TABLE,
    ) -> HRESULT,
    fn GetDcHuffmanTable(
        scanIndex: UINT,
        tableIndex: UINT,
        pDcHuffmanTable: *mut DXGI_JPEG_DC_HUFFMAN_TABLE,
    ) -> HRESULT,
    fn GetQuantizationTable(
        scanIndex: UINT,
        tableIndex: UINT,
        pQuantizationTable: *mut DXGI_JPEG_QUANTIZATION_TABLE,
    ) -> HRESULT,
    fn WriteScan(
        cbScanData: UINT,
        pbScanData: *const BYTE,
    ) -> HRESULT,
}}
