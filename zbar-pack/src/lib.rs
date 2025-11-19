//! Safe Rust bindings for ZBar barcode scanner library
//!
//! This crate provides a safe Rust wrapper around the ZBar C library,
//! with static linking of vendored ZBar source code by default.
//!
//! # Features
//!
//! - `vendored` (default): Compile and statically link bundled ZBar source
//! - `system`: Use system-installed ZBar library
//! - `dynamic`: Dynamically link system ZBar library
//! - `all-codecs` (default): Enable all codecs
//! - `minimal`: Minimal build, manually enable required codecs
//!
//! # License Compliance
//!
//! ZBar is licensed under LGPLv2.1+. When statically linking, this project must:
//! - Provide complete ZBar source code and build instructions (see zbar-src/)
//! - Allow end users to replace/relink the library
//! - See COMPLIANCE.md for details
//!
//! # Example
//!
//! ```no_run
//! use zbar_pack::{ImageScanner, Image, SymbolType};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut scanner = ImageScanner::new()?;
//! scanner.set_config(SymbolType::QRCODE, 0, 1)?;
//!
//! // Example: 100x100 grayscale image
//! let data: Vec<u8> = vec![0; 100 * 100];
//! let image = Image::from_gray(&data, 100, 100)?;
//! let symbols = scanner.scan_image(&image)?;
//!
//! for symbol in symbols {
//!     println!("Type: {:?}, Data: {}", symbol.symbol_type(), symbol.data());
//! }
//! # Ok(())
//! # }
//! ```

use std::fmt;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::ptr::NonNull;

use zbar_sys as ffi;

/// ZBar error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid argument
    InvalidArgument,
    /// Out of memory
    OutOfMemory,
    /// Internal error
    InternalError,
    /// System error
    SystemError,
    /// Unsupported operation
    Unsupported,
    /// Unknown error
    Unknown(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidArgument => write!(f, "invalid argument"),
            Error::OutOfMemory => write!(f, "out of memory"),
            Error::InternalError => write!(f, "internal error"),
            Error::SystemError => write!(f, "system error"),
            Error::Unsupported => write!(f, "unsupported operation"),
            Error::Unknown(code) => write!(f, "unknown error: {}", code),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Symbol types (barcode/QR code types)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SymbolType {
    None = ffi::ZBAR_NONE,
    Partial = ffi::ZBAR_PARTIAL,
    EAN2 = ffi::ZBAR_EAN2,
    EAN5 = ffi::ZBAR_EAN5,
    EAN8 = ffi::ZBAR_EAN8,
    UPCE = ffi::ZBAR_UPCE,
    ISBN10 = ffi::ZBAR_ISBN10,
    UPCA = ffi::ZBAR_UPCA,
    EAN13 = ffi::ZBAR_EAN13,
    ISBN13 = ffi::ZBAR_ISBN13,
    COMPOSITE = ffi::ZBAR_COMPOSITE,
    I25 = ffi::ZBAR_I25,
    DATABAR = ffi::ZBAR_DATABAR,
    DatabarExp = ffi::ZBAR_DATABAR_EXP,
    CODABAR = ffi::ZBAR_CODABAR,
    CODE39 = ffi::ZBAR_CODE39,
    PDF417 = ffi::ZBAR_PDF417,
    QRCODE = ffi::ZBAR_QRCODE,
    SQCODE = ffi::ZBAR_SQCODE,
    CODE93 = ffi::ZBAR_CODE93,
    CODE128 = ffi::ZBAR_CODE128,
}

/// Image object
pub struct Image {
    raw: NonNull<ffi::zbar_image_t>,
    owned: bool,
}

impl Image {
    /// Create from grayscale image data
    pub fn from_gray(data: &[u8], width: u32, height: u32) -> Result<Self> {
        unsafe {
            let img = ffi::zbar_image_create();
            if img.is_null() {
                return Err(Error::OutOfMemory);
            }

            // Y800 format (grayscale): fourcc('Y', '8', '0', '0')
            let format =
                ('Y' as u64) | (('8' as u64) << 8) | (('0' as u64) << 16) | (('0' as u64) << 24);
            ffi::zbar_image_set_format(img, format);
            ffi::zbar_image_set_size(img, width, height);
            ffi::zbar_image_set_data(
                img,
                data.as_ptr() as *const _,
                (width * height) as u64,
                None,
            );

            Ok(Image {
                raw: NonNull::new_unchecked(img),
                owned: true,
            })
        }
    }

    /// Get raw pointer
    pub(crate) fn as_ptr(&self) -> *mut ffi::zbar_image_t {
        self.raw.as_ptr()
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::zbar_image_destroy(self.raw.as_ptr());
            }
        }
    }
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

/// Scanned symbol
#[derive(Clone)]
pub struct Symbol<'a> {
    raw: *const ffi::zbar_symbol_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Symbol<'a> {
    /// Get symbol type
    pub fn symbol_type(&self) -> SymbolType {
        unsafe {
            let t = ffi::zbar_symbol_get_type(self.raw);
            std::mem::transmute(t)
        }
    }

    /// Get symbol data
    pub fn data(&self) -> &str {
        unsafe {
            let ptr = ffi::zbar_symbol_get_data(self.raw);
            let len = ffi::zbar_symbol_get_data_length(self.raw) as usize;
            let slice = std::slice::from_raw_parts(ptr as *const u8, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    /// Get symbol quality
    pub fn quality(&self) -> i32 {
        unsafe { ffi::zbar_symbol_get_quality(self.raw) }
    }
}

/// Symbol iterator
pub struct SymbolIter<'a> {
    current: *const ffi::zbar_symbol_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for SymbolIter<'a> {
    type Item = Symbol<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        let symbol = Symbol {
            raw: self.current,
            _phantom: PhantomData,
        };

        unsafe {
            self.current = ffi::zbar_symbol_next(self.current);
        }

        Some(symbol)
    }
}

/// Image scanner
pub struct ImageScanner {
    raw: NonNull<ffi::zbar_image_scanner_t>,
}

impl ImageScanner {
    /// Create new image scanner
    pub fn new() -> Result<Self> {
        unsafe {
            let scanner = ffi::zbar_image_scanner_create();
            if scanner.is_null() {
                return Err(Error::OutOfMemory);
            }

            Ok(ImageScanner {
                raw: NonNull::new_unchecked(scanner),
            })
        }
    }

    /// Set configuration
    pub fn set_config(&mut self, symbol_type: SymbolType, config: u32, value: c_int) -> Result<()> {
        unsafe {
            let ret = ffi::zbar_image_scanner_set_config(
                self.raw.as_ptr(),
                symbol_type as u32,
                config,
                value,
            );
            if ret != 0 {
                return Err(Error::InvalidArgument);
            }
            Ok(())
        }
    }

    /// Scan image
    pub fn scan_image<'a>(&mut self, image: &'a Image) -> Result<SymbolIter<'a>> {
        unsafe {
            let n = ffi::zbar_scan_image(self.raw.as_ptr(), image.as_ptr());
            if n < 0 {
                return Err(Error::InternalError);
            }

            let first_symbol = ffi::zbar_image_first_symbol(image.as_ptr());

            Ok(SymbolIter {
                current: first_symbol,
                _phantom: PhantomData,
            })
        }
    }
}

impl Default for ImageScanner {
    fn default() -> Self {
        Self::new().expect("failed to create image scanner")
    }
}

impl Drop for ImageScanner {
    fn drop(&mut self) {
        unsafe {
            ffi::zbar_image_scanner_destroy(self.raw.as_ptr());
        }
    }
}

unsafe impl Send for ImageScanner {}
unsafe impl Sync for ImageScanner {}

/// Get ZBar version information
pub fn version() -> (u32, u32) {
    unsafe {
        let mut major = 0u32;
        let mut minor = 0u32;
        ffi::zbar_version(&mut major, &mut minor, std::ptr::null_mut());
        (major, minor)
    }
}

/// Set global library debug/verbosity level
///
/// Controls the amount of debug output from ZBar library.
///
/// # Arguments
///
/// * `verbosity` - Debug level (0 = silent, higher values = more output)
///
/// # Example
///
/// ```
/// use zbar_pack::set_verbosity;
///
/// // Disable all debug output
/// set_verbosity(0);
/// ```
pub fn set_verbosity(verbosity: i32) {
    unsafe {
        ffi::zbar_set_verbosity(verbosity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let (major, minor) = version();
        assert_eq!(major, 0, "ZBar major version should be 0");
        assert_eq!(minor, 23, "ZBar minor version should be 23");
        println!("ZBar version: {}.{}", major, minor);
    }

    #[test]
    fn test_scanner_creation() {
        let scanner = ImageScanner::new();
        assert!(scanner.is_ok(), "Failed to create scanner");
    }
}
