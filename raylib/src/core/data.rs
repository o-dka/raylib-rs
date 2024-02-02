//! Data manipulation functions. Compress and Decompress with DEFLATE
use std::ffi::CString;

use crate::ffi;

/// Compress data (DEFLATE algorythm)
/// ```rust
/// use raylib::prelude::*;
/// let data = compress_data(b"1111111111");
/// let expected: &[u8] = &[61, 193, 33, 1, 0, 0, 0, 128, 160, 77, 254, 63, 103, 3, 98];
/// assert_eq!(data, Ok(expected));
/// ```
pub fn compress_data(data: &[u8]) -> Result<&'static [u8], String> {
    let mut out_length: i32 = 0;
    // CompressData doesn't actually modify the data, but the header is wrong
    let buffer = {
        unsafe { ffi::CompressData(data.as_ptr() as *mut _, data.len() as i32, &mut out_length) }
    };
    if buffer.is_null() {
        return Err("could not compress data".to_string());
    }
    let buffer = unsafe { std::slice::from_raw_parts(buffer, out_length as usize) };
    return Ok(buffer);
}

/// Decompress data (DEFLATE algorythm)
/// ```rust
/// use raylib::prelude::*;
/// let input: &[u8] = &[61, 193, 33, 1, 0, 0, 0, 128, 160, 77, 254, 63, 103, 3, 98];
/// let expected: &[u8] = b"1111111111";
/// let data = decompress_data(input);
/// assert_eq!(data, Ok(expected));
/// ```
pub fn decompress_data(data: &[u8]) -> Result<&'static [u8], String> {
    let mut out_length: i32 = 0;
    // CompressData doesn't actually modify the data, but the header is wrong
    let buffer = {
        unsafe { ffi::DecompressData(data.as_ptr() as *mut _, data.len() as i32, &mut out_length) }
    };
    if buffer.is_null() {
        return Err("could not compress data".to_string());
    }
    let buffer = unsafe { std::slice::from_raw_parts(buffer, out_length as usize) };
    return Ok(buffer);
}

/// Export data to code (.h), returns true on success
pub fn export_data_as_code<A>(data: &[u8], file_name: A) -> bool
where
    A: Into<String>,
{
    let file_name = file_name.into();
    let c_str = CString::new(file_name).unwrap();

    unsafe { ffi::ExportDataAsCode(data.as_ptr(), data.len() as i32, c_str.as_ptr()) }
}

/// Encode data to Base64 string
pub fn encode_data_base64(data: &[u8]) -> Vec<i8> {
    let mut output_size = 0;
    let bytes =
        unsafe { ffi::EncodeDataBase64(data.as_ptr(), data.len() as i32, &mut output_size) };

    let s = unsafe { std::slice::from_raw_parts(bytes, output_size as usize) };
    if s.contains(&0) {
        // Work around a bug in Rust's from_raw_parts function
        let mut keep = true;
        let b: Vec<i8> = s
            .iter()
            .filter(|f| {
                if **f == 0 {
                    keep = false;
                }
                keep
            })
            .map(|f| *f)
            .collect();
        b
    } else {
        s.to_vec()
    }
}

// Decode Base64 data
pub fn decode_data_base64(data: &[u8]) -> Vec<u8> {
    let mut output_size = 0;

    let bytes = unsafe { ffi::DecodeDataBase64(data.as_ptr(), &mut output_size) };

    let s = unsafe { std::slice::from_raw_parts(bytes, output_size as usize) };
    if s.contains(&0) {
        // Work around a bug in Rust's from_raw_parts function
        let mut keep = true;
        let b: Vec<u8> = s
            .iter()
            .filter(|f| {
                if **f == 0 {
                    keep = false;
                }
                keep
            })
            .map(|f| *f)
            .collect();
        b
    } else {
        s.to_vec()
    }
}
