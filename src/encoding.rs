use core::slice;

use windows::{
    core::{Error, Result},
    Win32::Globalization::{
        MultiByteToWideChar, WideCharToMultiByte, CP_UTF8, MB_ERR_INVALID_CHARS, MB_PRECOMPOSED,
        WC_ERR_INVALID_CHARS,
    },
};

pub(crate) fn to_utf16(input: impl AsRef<[u8]>, code_page: u32) -> Result<Vec<u16>> {
    let required = unsafe {
        MultiByteToWideChar(
            code_page,
            MB_ERR_INVALID_CHARS | MB_PRECOMPOSED,
            input.as_ref(),
            None,
        )
    };
    if required < 1 {
        return Err(Error::from_win32());
    }

    let mut buffer = Vec::with_capacity(required as usize);
    let buffer_view = unsafe { slice::from_raw_parts_mut(buffer.as_mut_ptr(), required as usize) };
    let written = unsafe {
        MultiByteToWideChar(
            code_page,
            MB_ERR_INVALID_CHARS | MB_PRECOMPOSED,
            input.as_ref(),
            Some(buffer_view),
        )
    };
    if written < 1 {
        return Err(Error::from_win32());
    }

    unsafe { buffer.set_len(written as usize) };

    Ok(buffer)
}

pub(crate) fn to_utf8(input: impl AsRef<[u16]>) -> Result<Vec<u8>> {
    let required = unsafe {
        WideCharToMultiByte(
            CP_UTF8,
            WC_ERR_INVALID_CHARS,
            input.as_ref(),
            None,
            None,
            None,
        )
    };
    if required < 1 {
        return Err(Error::from_win32());
    }

    let mut buffer = Vec::with_capacity(required as usize);
    let buffer_view = unsafe { slice::from_raw_parts_mut(buffer.as_mut_ptr(), required as usize) };
    let written = unsafe {
        WideCharToMultiByte(
            CP_UTF8,
            WC_ERR_INVALID_CHARS,
            input.as_ref(),
            Some(buffer_view),
            None,
            None,
        )
    };
    if written < 1 {
        return Err(Error::from_win32());
    }

    unsafe { buffer.set_len(written as usize) };

    Ok(buffer)
}
