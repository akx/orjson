// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[cfg(Py_GIL_DISABLED)]
use crate::ffi::compat::_Py_IMMORTAL_REFCNT_LOCAL;
#[cfg(Py_GIL_DISABLED)]
use core::sync::atomic::{AtomicIsize, AtomicU32};

#[cfg(all(Py_GIL_DISABLED, feature = "c_ulong_32"))]
pub(crate) type AtomicCULong = std::sync::atomic::AtomicU32;

#[cfg(all(Py_GIL_DISABLED, not(feature = "c_ulong_32")))]
pub(crate) type AtomicCULong = core::sync::atomic::AtomicU64;

#[cfg(Py_GIL_DISABLED)]
type MaybeAtomicCULong = AtomicCULong;

#[cfg(not(Py_GIL_DISABLED))]
type MaybeAtomicCULong = core::ffi::c_ulong;

use core::ptr::null_mut;

use pyo3_ffi::{
    PyObject, PyTypeObject, PyType_Type, PyVarObject, Py_TPFLAGS_DEFAULT, Py_TPFLAGS_IMMUTABLETYPE,
};

#[cfg(not(Py_GIL_DISABLED))]
use pyo3_ffi::PyObjectObRefcnt;

// https://docs.python.org/3/c-api/typeobj.html#typedef-examples

#[allow(unused_macros)]
macro_rules! pymutex_new {
    () => {
        unsafe { core::mem::zeroed() }
    };
}

pub(crate) use pymutex_new;

#[cfg(Py_GIL_DISABLED)]
const DEFAULT_TP_FLAGS: MaybeAtomicCULong =
    MaybeAtomicCULong::new(Py_TPFLAGS_DEFAULT | Py_TPFLAGS_IMMUTABLETYPE);

#[cfg(all(Py_3_10, not(Py_GIL_DISABLED)))]
const DEFAULT_TP_FLAGS: MaybeAtomicCULong = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_IMMUTABLETYPE;

#[cfg(not(Py_3_10))]
const DEFAULT_TP_FLAGS: MaybeAtomicCULong = Py_TPFLAGS_DEFAULT;

pub(crate) unsafe fn create_type_object(
    name: *const core::ffi::c_char,
    basicsize: isize,
    tp_dealloc: Option<unsafe extern "C" fn(*mut PyObject)>,
    tp_new: Option<
        unsafe extern "C" fn(*mut PyTypeObject, *mut PyObject, *mut PyObject) -> *mut PyObject,
    >,
    tp_iter: Option<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>,
    tp_iternext: Option<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>,
) -> Box<PyTypeObject> {
    Box::new(PyTypeObject {
        ob_base: PyVarObject {
            ob_base: PyObject {
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
                ob_ref_local: AtomicU32::new(_Py_IMMORTAL_REFCNT_LOCAL),
                #[cfg(Py_GIL_DISABLED)]
                ob_ref_shared: AtomicIsize::new(0),
                #[cfg(all(Py_3_12, not(Py_GIL_DISABLED)))]
                ob_refcnt: PyObjectObRefcnt { ob_refcnt: 0 },
                #[cfg(not(Py_3_12))]
                ob_refcnt: 0,
                ob_type: &raw mut PyType_Type,
            },
            ob_size: 0,
        },
        tp_name: name,
        tp_basicsize: basicsize,
        tp_itemsize: 0,
        tp_dealloc,
        tp_init: None,
        tp_new,
        tp_flags: DEFAULT_TP_FLAGS,
        tp_iter,
        tp_iternext,
        tp_bases: null_mut(),
        tp_cache: null_mut(),
        tp_del: None,
        tp_finalize: None,
        tp_free: None,
        tp_is_gc: None,
        tp_mro: null_mut(),
        tp_subclasses: null_mut(),
        tp_vectorcall: None,
        tp_version_tag: 0,
        tp_weaklist: null_mut(),
        tp_vectorcall_offset: 0,
        tp_getattr: None,
        tp_setattr: None,
        tp_as_async: null_mut(),
        tp_repr: None,
        tp_as_number: null_mut(),
        tp_as_sequence: null_mut(),
        tp_as_mapping: null_mut(),
        tp_hash: None,
        tp_call: None,
        tp_str: None,
        tp_getattro: None,
        tp_setattro: None,
        tp_as_buffer: null_mut(),
        tp_doc: null_mut(),
        tp_traverse: None,
        tp_clear: None,
        tp_richcompare: None,
        tp_weaklistoffset: 0,
        tp_methods: null_mut(),
        tp_members: null_mut(),
        tp_getset: null_mut(),
        tp_base: null_mut(),
        tp_dict: null_mut(),
        tp_descr_get: None,
        tp_descr_set: None,
        tp_dictoffset: 0,
        tp_alloc: None,
        #[cfg(Py_3_12)]
        tp_watched: 0,
    })
}
