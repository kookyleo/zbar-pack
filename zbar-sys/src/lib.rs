#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Auto-generated bindings from zbar.h via bindgen
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        unsafe {
            let mut major = 0u32;
            let mut minor = 0u32;
            let mut patch = 0u32;
            zbar_version(&mut major, &mut minor, &mut patch);
            assert!(major == 0 && minor == 23, "ZBar version should be 0.23.x");
        }
    }
}
