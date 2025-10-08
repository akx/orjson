// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::deserialize::utf8::read_input_to_buf;
use crate::deserialize::DeserializeError;
use crate::typeref::EMPTY_UNICODE;
use core::ptr::NonNull;

pub(crate) struct DeserializeResult {
    pub(crate) obj: NonNull<pyo3_ffi::PyObject>,
    pub(crate) bytes_read: usize,
}

pub(crate) fn deserialize(
    ptr: *mut pyo3_ffi::PyObject,
    must_read_all: bool,
) -> Result<DeserializeResult, DeserializeError<'static>> {
    debug_assert!(ffi!(Py_REFCNT(ptr)) >= 1);
    let buffer = read_input_to_buf(ptr, true)?;
    deserialize_buffer(buffer, must_read_all)
}

pub(crate) fn deserialize_buffer(
    buffer: &'static [u8],
    must_read_all: bool,
) -> Result<DeserializeResult, DeserializeError<'static>> {
    debug_assert!(!buffer.is_empty());

    if unlikely!(buffer.len() == 2) {
        if buffer == b"[]" {
            return Ok(DeserializeResult {
                obj: nonnull!(ffi!(PyList_New(0))),
                bytes_read: 2,
            });
        } else if buffer == b"{}" {
            return Ok(DeserializeResult {
                obj: nonnull!(ffi!(PyDict_New())),
                bytes_read: 2,
            });
        } else if buffer == b"\"\"" {
            unsafe {
                return Ok(DeserializeResult {
                    obj: nonnull!(use_immortal!(EMPTY_UNICODE)),
                    bytes_read: 2,
                });
            }
        }
    }
    crate::deserialize::backend::deserialize(buffer, must_read_all)
}
