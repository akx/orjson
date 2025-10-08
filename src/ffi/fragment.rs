// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[allow(unused_imports)]
use crate::ffi::pytypes_common::{create_type_object, pymutex_new};
use core::ffi::c_char;
use core::ptr::null_mut;
use pyo3_ffi::{
    PyErr_SetObject, PyExc_TypeError, PyObject, PyTypeObject, PyType_Ready,
    PyUnicode_FromStringAndSize, Py_DECREF, Py_INCREF, Py_SIZE,
};

#[allow(unused_imports)]
use core::sync::atomic::{AtomicIsize, AtomicU32};

#[repr(C)]
pub(crate) struct Fragment {
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
    pub contents: *mut PyObject,
}

#[cold]
#[inline(never)]
#[cfg_attr(feature = "optimize", optimize(size))]
unsafe fn raise_args_exception() {
    let msg = "orjson.Fragment() takes exactly 1 positional argument";
    let err_msg = PyUnicode_FromStringAndSize(msg.as_ptr().cast::<c_char>(), msg.len() as isize);
    PyErr_SetObject(PyExc_TypeError, err_msg);
    Py_DECREF(err_msg);
}

#[unsafe(no_mangle)]
#[cold]
#[cfg_attr(feature = "optimize", optimize(size))]
unsafe extern "C" fn orjson_fragment_tp_new(
    _subtype: *mut PyTypeObject,
    args: *mut PyObject,
    kwds: *mut PyObject,
) -> *mut PyObject {
    if Py_SIZE(args) != 1 || !kwds.is_null() {
        raise_args_exception();
        return null_mut();
    }
    let contents = crate::ffi::PyTuple_GET_ITEM(args, 0);
    Py_INCREF(contents);
    let obj = Box::new(Fragment {
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
        ob_type: crate::typeref::FRAGMENT_TYPE,
        contents,
    });
    Box::into_raw(obj).cast::<PyObject>()
}

#[unsafe(no_mangle)]
#[cold]
#[cfg_attr(feature = "optimize", optimize(size))]
unsafe extern "C" fn orjson_fragment_dealloc(object: *mut PyObject) {
    Py_DECREF((*object.cast::<Fragment>()).contents);
    std::alloc::dealloc(object.cast::<u8>(), core::alloc::Layout::new::<Fragment>());
}

#[unsafe(no_mangle)]
#[cold]
#[cfg_attr(feature = "optimize", optimize(size))]
pub(crate) unsafe extern "C" fn orjson_fragmenttype_new() -> *mut PyTypeObject {
    let ob = create_type_object(
        c"orjson.Fragment".as_ptr(),
        size_of::<Fragment>() as isize,
        Some(orjson_fragment_dealloc),
        Some(orjson_fragment_tp_new),
        None,
        None,
    );
    let ob_ptr = Box::into_raw(ob);
    PyType_Ready(ob_ptr);
    ob_ptr
}
