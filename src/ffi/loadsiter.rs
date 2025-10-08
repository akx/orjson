// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::deserialize::DeserializeResult;
#[allow(unused_imports)]
use crate::ffi::pytypes_common::{create_type_object, pymutex_new};
use crate::raise_loads_exception;
use core::ffi::c_char;
use core::ptr::null_mut;
use pyo3_ffi::{
    PyErr_SetObject, PyExc_TypeError, PyObject, PyTypeObject, PyType_Ready,
    PyUnicode_FromStringAndSize, Py_DECREF, Py_INCREF, Py_SIZE,
};

#[allow(unused_imports)]
use core::sync::atomic::{AtomicIsize, AtomicU32};

#[repr(C)]
pub(crate) struct LoadsIterator {
    #[cfg(Py_GIL_DISABLED)]
    pub ob_tid: usize,
    #[cfg(all(Py_GIL_DISABLED, Py_3_14))]
    pub ob_flags: u16,
    #[cfg(all(Py_GIL_DISABLED, not(Py_3_14)))]
    pub _padding: u16,
    #[cfg(Py_GIL_DISABLED)]
    pub ob_mutex: pyo3_ffi::PyMutex,
    #[cfg(Py_GIL_DISABLED)]
    pub ob_gc_bits: u8,
    #[cfg(Py_GIL_DISABLED)]
    pub ob_ref_local: AtomicU32,
    #[cfg(Py_GIL_DISABLED)]
    pub ob_ref_shared: AtomicIsize,
    #[cfg(not(Py_GIL_DISABLED))]
    pub ob_refcnt: pyo3_ffi::Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub data: *mut PyObject,
    pub offset: usize,
    pub buffer_ptr: *const u8,
    pub buffer_len: usize,
}
#[cold]
#[inline(never)]
#[cfg_attr(feature = "optimize", optimize(size))]
unsafe fn raise_args_exception() {
    let msg = "orjson.LoadsIterator() takes exactly 1 positional argument";
    let err_msg = PyUnicode_FromStringAndSize(msg.as_ptr().cast::<c_char>(), msg.len() as isize);
    PyErr_SetObject(PyExc_TypeError, err_msg);
    Py_DECREF(err_msg);
}

#[unsafe(no_mangle)]
#[cold]
#[cfg_attr(feature = "optimize", optimize(size))]
unsafe extern "C" fn orjson_loadsiterator_tp_new(
    _subtype: *mut PyTypeObject,
    args: *mut PyObject,
    kwds: *mut PyObject,
) -> *mut PyObject {
    if Py_SIZE(args) != 1 || !kwds.is_null() {
        raise_args_exception();
        return null_mut();
    }
    let data = crate::ffi::PyTuple_GET_ITEM(args, 0);
    Py_INCREF(data);
    let obj = Box::new(LoadsIterator {
        #[cfg(Py_GIL_DISABLED)]
        ob_tid: 0,
        #[cfg(all(Py_GIL_DISABLED, Py_3_14))]
        ob_flags: 0,
        #[cfg(all(Py_GIL_DISABLED, not(Py_3_14)))]
        _padding: 0,
        #[cfg(Py_GIL_DISABLED)]
        ob_mutex: pymutex_new!(),
        #[cfg(Py_GIL_DISABLED)]
        ob_gc_bits: 0,
        #[cfg(Py_GIL_DISABLED)]
        ob_ref_local: AtomicU32::new(0),
        #[cfg(Py_GIL_DISABLED)]
        ob_ref_shared: AtomicIsize::new(0),
        #[cfg(not(Py_GIL_DISABLED))]
        ob_refcnt: 1,
        ob_type: crate::typeref::LOADSITERATOR_TYPE,
        data,
        offset: 0,
        buffer_ptr: core::ptr::null(),
        buffer_len: 0,
    });
    Box::into_raw(obj).cast::<PyObject>()
}

#[unsafe(no_mangle)]
#[cold]
#[cfg_attr(feature = "optimize", optimize(size))]
unsafe extern "C" fn orjson_loadsiterator_dealloc(object: *mut PyObject) {
    Py_DECREF((*object.cast::<LoadsIterator>()).data);
    std::alloc::dealloc(
        object.cast::<u8>(),
        core::alloc::Layout::new::<LoadsIterator>(),
    );
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "optimize", optimize(size))]
unsafe extern "C" fn orjson_loadsiterator_iter(object: *mut PyObject) -> *mut PyObject {
    Py_INCREF(object);
    object
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "optimize", optimize(size))]
unsafe extern "C" fn orjson_loadsiterator_iternext(object: *mut PyObject) -> *mut PyObject {
    let iter = object.cast::<LoadsIterator>();
    let mut offset = (*iter).offset;

    // Initialize buffer on first iteration
    if (*iter).buffer_ptr.is_null() {
        let data_obj = (*iter).data;
        let buffer = match crate::deserialize::read_input_to_buf(data_obj, false) {
            Ok(buf) => buf,
            Err(err) => return raise_loads_exception(err),
        };
        (*iter).buffer_ptr = buffer.as_ptr();
        (*iter).buffer_len = buffer.len();
    }

    let buffer = core::slice::from_raw_parts((*iter).buffer_ptr, (*iter).buffer_len);

    // Skip whitespace
    while offset < buffer.len() {
        let byte = buffer[offset];
        if !byte.is_ascii_whitespace() {
            break;
        }
        offset += 1;
    }
    let next = &buffer[offset..];

    if next.is_empty() {
        return null_mut(); // StopIteration
    }

    match crate::deserialize::deserialize_buffer(next, false) {
        Ok(DeserializeResult {
            obj, bytes_read, ..
        }) => {
            (*iter).offset = offset + bytes_read;
            obj.as_ptr()
        }
        Err(err) => raise_loads_exception(err),
    }
}

#[unsafe(no_mangle)]
#[cold]
#[cfg_attr(feature = "optimize", optimize(size))]
pub(crate) unsafe extern "C" fn orjson_loadsiteratortype_new() -> *mut PyTypeObject {
    let ob = create_type_object(
        c"orjson.LoadsIterator".as_ptr(),
        size_of::<LoadsIterator>() as isize,
        Some(orjson_loadsiterator_dealloc),
        Some(orjson_loadsiterator_tp_new),
        Some(orjson_loadsiterator_iter),
        Some(orjson_loadsiterator_iternext),
    );
    let ob_ptr = Box::into_raw(ob);
    PyType_Ready(ob_ptr);
    ob_ptr
}
