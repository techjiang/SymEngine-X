use symx_core::prelude::*;

#[no_mangle]
pub extern "C" fn symx_version() -> *const u8 {
    symx_core::VERSION.as_ptr()
}

#[no_mangle]
pub extern "C" fn symx_parse_to_latex(input: *const u8, len: usize) -> *mut u8 {
    let bytes = unsafe { std::slice::from_raw_parts(input, len) };
    let source = String::from_utf8_lossy(bytes);
    let output = parse(&source).map(|expr| to_latex(&expr)).unwrap_or_else(|err| format!("error: {err}"));
    let mut boxed = output.into_bytes().into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    std::mem::forget(boxed);
    ptr
}
