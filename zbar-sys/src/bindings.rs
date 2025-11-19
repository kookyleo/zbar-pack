// 预生成的 ZBar FFI 绑定
// 从 zbar.h 生成
//
// 注意: 这是一个简化的绑定文件,包含核心 API
// 完整版本应使用 bindgen 从完整的 zbar.h 生成

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};

// 颜色类型
pub type zbar_color_t = u32;
pub const zbar_color_e_ZBAR_SPACE: zbar_color_t = 0;
pub const zbar_color_e_ZBAR_BAR: zbar_color_t = 1;

// 符号类型
pub type zbar_symbol_type_t = u32;
pub const zbar_symbol_type_e_ZBAR_NONE: zbar_symbol_type_t = 0;
pub const zbar_symbol_type_e_ZBAR_PARTIAL: zbar_symbol_type_t = 1;
pub const zbar_symbol_type_e_ZBAR_EAN2: zbar_symbol_type_t = 2;
pub const zbar_symbol_type_e_ZBAR_EAN5: zbar_symbol_type_t = 5;
pub const zbar_symbol_type_e_ZBAR_EAN8: zbar_symbol_type_t = 8;
pub const zbar_symbol_type_e_ZBAR_UPCE: zbar_symbol_type_t = 9;
pub const zbar_symbol_type_e_ZBAR_ISBN10: zbar_symbol_type_t = 10;
pub const zbar_symbol_type_e_ZBAR_UPCA: zbar_symbol_type_t = 12;
pub const zbar_symbol_type_e_ZBAR_EAN13: zbar_symbol_type_t = 13;
pub const zbar_symbol_type_e_ZBAR_ISBN13: zbar_symbol_type_t = 14;
pub const zbar_symbol_type_e_ZBAR_COMPOSITE: zbar_symbol_type_t = 15;
pub const zbar_symbol_type_e_ZBAR_I25: zbar_symbol_type_t = 25;
pub const zbar_symbol_type_e_ZBAR_DATABAR: zbar_symbol_type_t = 34;
pub const zbar_symbol_type_e_ZBAR_DATABAR_EXP: zbar_symbol_type_t = 35;
pub const zbar_symbol_type_e_ZBAR_CODABAR: zbar_symbol_type_t = 38;
pub const zbar_symbol_type_e_ZBAR_CODE39: zbar_symbol_type_t = 39;
pub const zbar_symbol_type_e_ZBAR_PDF417: zbar_symbol_type_t = 57;
pub const zbar_symbol_type_e_ZBAR_QRCODE: zbar_symbol_type_t = 64;
pub const zbar_symbol_type_e_ZBAR_SQCODE: zbar_symbol_type_t = 80;
pub const zbar_symbol_type_e_ZBAR_CODE93: zbar_symbol_type_t = 93;
pub const zbar_symbol_type_e_ZBAR_CODE128: zbar_symbol_type_t = 128;

// 配置类型
pub type zbar_config_t = u32;
pub const zbar_config_e_ZBAR_CFG_ENABLE: zbar_config_t = 0;
pub const zbar_config_e_ZBAR_CFG_ADD_CHECK: zbar_config_t = 1;
pub const zbar_config_e_ZBAR_CFG_EMIT_CHECK: zbar_config_t = 2;
pub const zbar_config_e_ZBAR_CFG_ASCII: zbar_config_t = 3;
pub const zbar_config_e_ZBAR_CFG_MIN_LEN: zbar_config_t = 32;
pub const zbar_config_e_ZBAR_CFG_MAX_LEN: zbar_config_t = 33;
pub const zbar_config_e_ZBAR_CFG_X_DENSITY: zbar_config_t = 256;
pub const zbar_config_e_ZBAR_CFG_Y_DENSITY: zbar_config_t = 257;

// 不透明类型
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_symbol_s {
    _unused: [u8; 0],
}
pub type zbar_symbol_t = zbar_symbol_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_symbol_set_s {
    _unused: [u8; 0],
}
pub type zbar_symbol_set_t = zbar_symbol_set_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_image_s {
    _unused: [u8; 0],
}
pub type zbar_image_t = zbar_image_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_image_scanner_s {
    _unused: [u8; 0],
}
pub type zbar_image_scanner_t = zbar_image_scanner_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_decoder_s {
    _unused: [u8; 0],
}
pub type zbar_decoder_t = zbar_decoder_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_scanner_s {
    _unused: [u8; 0],
}
pub type zbar_scanner_t = zbar_scanner_s;

// FourCC 宏
pub const fn zbar_fourcc(a: u8, b: u8, c: u8, d: u8) -> c_ulong {
    ((a as c_ulong) | ((b as c_ulong) << 8) | ((c as c_ulong) << 16) | ((d as c_ulong) << 24))
}

extern "C" {
    // 版本信息
    pub fn zbar_version(major: *mut c_uint, minor: *mut c_uint) -> c_uint;

    // 符号 API
    pub fn zbar_symbol_get_type(symbol: *const zbar_symbol_t) -> zbar_symbol_type_t;
    pub fn zbar_symbol_get_data(symbol: *const zbar_symbol_t) -> *const c_char;
    pub fn zbar_symbol_get_data_length(symbol: *const zbar_symbol_t) -> c_uint;
    pub fn zbar_symbol_get_quality(symbol: *const zbar_symbol_t) -> c_int;
    pub fn zbar_symbol_next(symbol: *const zbar_symbol_t) -> *const zbar_symbol_t;
    pub fn zbar_symbol_ref(symbol: *const zbar_symbol_t, refs: c_int);

    // 图像 API
    pub fn zbar_image_create() -> *mut zbar_image_t;
    pub fn zbar_image_destroy(image: *mut zbar_image_t);
    pub fn zbar_image_ref(image: *mut zbar_image_t, refs: c_int);
    pub fn zbar_image_set_format(image: *mut zbar_image_t, format: c_ulong);
    pub fn zbar_image_set_size(image: *mut zbar_image_t, width: c_uint, height: c_uint);
    pub fn zbar_image_set_data(
        image: *mut zbar_image_t,
        data: *const c_void,
        data_byte_length: c_ulong,
        cleanup: Option<unsafe extern "C" fn(image: *mut zbar_image_t)>,
    );
    pub fn zbar_image_get_width(image: *const zbar_image_t) -> c_uint;
    pub fn zbar_image_get_height(image: *const zbar_image_t) -> c_uint;
    pub fn zbar_image_get_data(image: *const zbar_image_t) -> *const c_void;
    pub fn zbar_image_get_symbols(image: *const zbar_image_t) -> *const zbar_symbol_set_t;
    pub fn zbar_image_first_symbol(image: *const zbar_image_t) -> *const zbar_symbol_t;

    // 图像扫描器 API
    pub fn zbar_image_scanner_create() -> *mut zbar_image_scanner_t;
    pub fn zbar_image_scanner_destroy(scanner: *mut zbar_image_scanner_t);
    pub fn zbar_image_scanner_set_config(
        scanner: *mut zbar_image_scanner_t,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: c_int,
    ) -> c_int;
    pub fn zbar_scan_image(
        scanner: *mut zbar_image_scanner_t,
        image: *mut zbar_image_t,
    ) -> c_int;

    // 解码器 API
    pub fn zbar_decoder_create() -> *mut zbar_decoder_t;
    pub fn zbar_decoder_destroy(decoder: *mut zbar_decoder_t);
    pub fn zbar_decoder_set_config(
        decoder: *mut zbar_decoder_t,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: c_int,
    ) -> c_int;

    // 扫描器 API
    pub fn zbar_scanner_create(decoder: *mut zbar_decoder_t) -> *mut zbar_scanner_t;
    pub fn zbar_scanner_destroy(scanner: *mut zbar_scanner_t);
}
