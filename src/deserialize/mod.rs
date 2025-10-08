// SPDX-License-Identifier: (Apache-2.0 OR MIT)

mod backend;
#[cfg(not(Py_GIL_DISABLED))]
mod cache;
mod deserializer;
mod error;
mod pyobject;
mod utf8;

#[cfg(not(Py_GIL_DISABLED))]
pub(crate) use cache::{KeyMap, KEY_MAP};
pub(crate) use deserializer::{deserialize, deserialize_buffer, DeserializeResult};
pub(crate) use error::DeserializeError;
pub(crate) use utf8::read_input_to_buf;
