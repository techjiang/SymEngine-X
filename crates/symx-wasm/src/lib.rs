use symx_core::prelude::*;

#[no_mangle]
pub extern "C" fn symx_eval_latex_ptr(input: *const u8, len: usize) -> *mut u8 {
    let bytes = unsafe { std::slice::from_raw_parts(input, len) };
    let source = String::from_utf8_lossy(bytes);
    let output = parse_latex(&source)
        .map(|expr| to_latex(&simplify(&expr)))
        .unwrap_or_else(|err| format!("error: {err}"));
    let mut boxed = output.into_bytes().into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    std::mem::forget(boxed);
    ptr
}
