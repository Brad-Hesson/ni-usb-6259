use std::{ffi::c_char, rc::Rc};

use crate::{
    daqmx,
    error_enums::{ErrorCode, WarningCode},
};

pub trait ErrorCoded {
    fn check_code(self) -> Result<(), Error>;
}
impl ErrorCoded for i32 {
    fn check_code(self) -> Result<(), Error> {
        match self.cmp(&0) {
            std::cmp::Ordering::Less => return Err(Error::from_code(self)),
            std::cmp::Ordering::Greater => {
                tracing::warn!("{}", Warning::from_code(self))
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{}", self.ext)]
pub struct Error {
    code: ErrorCode,
    ext: Box<str>,
}
impl Error {
    fn from_code(code: i32) -> Self {
        Self {
            code: ErrorCode::try_from(code).unwrap(),
            ext: get_last_error_info(),
        }
    }
}

pub struct Warning {
    code: WarningCode,
    desc: Box<str>,
}
impl Warning {
    fn from_code(code: i32) -> Self {
        Self {
            code: WarningCode::try_from(code).unwrap(),
            desc: get_last_error_info(),
        }
    }
}
impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}

fn get_error_string(code: i32) -> Box<str> {
    let buf_size = unsafe { daqmx::bindings::DAQmxGetErrorString(code, std::ptr::null_mut(), 0) };
    let mut buf = vec![0u8; buf_size as usize];
    unsafe { daqmx::bindings::DAQmxGetErrorString(code, buf.as_mut_ptr() as _, buf_size as u32) };
    unsafe { std::str::from_boxed_utf8_unchecked(buf.into_boxed_slice()) }
}

fn get_last_error_info() -> Box<str> {
    let buf_size = unsafe { daqmx::bindings::DAQmxGetExtendedErrorInfo(std::ptr::null_mut(), 0) };
    let mut buf = vec![0u8; buf_size as usize];
    unsafe { daqmx::bindings::DAQmxGetExtendedErrorInfo(buf.as_mut_ptr() as _, buf_size as u32) };
    unsafe { std::str::from_boxed_utf8_unchecked(buf.into_boxed_slice()) }
}
