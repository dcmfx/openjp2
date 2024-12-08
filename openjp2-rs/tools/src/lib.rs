#[cfg(feature = "lcms2")]
extern crate lcms2;
extern crate log;
///! Common code shared between the openjp2 CLI tools.
extern crate openjp2;

pub mod color;

// Compression formats
pub const J2K_CFMT: i32 = 0;
pub const JP2_CFMT: i32 = 1;
pub const JPT_CFMT: i32 = 2;

// Decompression formats
pub const PXM_DFMT: i32 = 10;
pub const PGX_DFMT: i32 = 11;
pub const BMP_DFMT: i32 = 12;
pub const YUV_DFMT: i32 = 13;
pub const TIF_DFMT: i32 = 14;
pub const RAW_DFMT: i32 = 15; // MSB / Big Endian
pub const TGA_DFMT: i32 = 16;
pub const PNG_DFMT: i32 = 17;
pub const RAWL_DFMT: i32 = 18; // LSB / Little Endian
