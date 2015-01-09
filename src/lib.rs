#![crate_name="gdal"]
#![crate_type="lib"]
#![feature(unsafe_destructor)]

extern crate libc;
#[cfg(test)] extern crate test;

use libc::c_char;
use std::ffi::CString;

mod utils;
pub mod raster;
pub mod vector;
pub mod proj;
pub mod geom;
pub mod warp;


#[link(name="gdal")]
extern {
    fn GDALVersionInfo(key: *const c_char) -> *const c_char;
}


pub fn version_info(key: &str) -> String {
    let c_key = CString::from_slice(key.as_bytes());
    return utils::_string(unsafe { GDALVersionInfo(c_key.as_ptr()) });
}


#[cfg(test)]
mod tests {
    use super::version_info;


    #[test]
    fn test_version_info() {
        let release_date = version_info("RELEASE_DATE");
        let release_name = version_info("RELEASE_NAME");
        let version_text = version_info("--version");

        let expected_text: String = format!(
            "GDAL {}, released {}/{}/{}",
            release_name,
            release_date.as_slice().slice(0, 4),
            release_date.as_slice().slice(4, 6),
            release_date.as_slice().slice(6, 8),
        );

        assert_eq!(version_text, expected_text);
    }
}
