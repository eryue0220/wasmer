//! Data types, functions and traits for `sys` runtime's `Tag` implementation.

use super::store::StoreHandle;
use crate::{
    js::vm::{VMException, VMExceptionRef},
    AsStoreMut, AsStoreRef, Tag, Value,
};
use std::any::Any;
use wasmer_types::{TagType, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A WebAssembly `tag` in the `v8` runtime.
pub(crate) struct Exception {
    pub(crate) handle: VMException,
}

unsafe impl Send for Exception {}
unsafe impl Sync for Exception {}

impl Exception {
    /// Create a new [`Exception`].
    pub fn new(store: &mut impl AsStoreMut, tag: Tag, payload: &[Value]) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
/// A WebAssembly `exnref` in `js`.
pub(crate) struct ExceptionRef;

impl ExceptionRef {
    pub fn new<T>(_store: &mut impl AsStoreMut, _value: T) -> Self
    where
        T: Any + Send + Sync + 'static + Sized,
    {
        unimplemented!("ExceptionRef is not yet supported in js");
    }

    pub fn downcast<'a, T>(&self, _store: &'a impl AsStoreRef) -> Option<&'a T>
    where
        T: Any + Send + Sync + 'static + Sized,
    {
        unimplemented!("ExceptionRef is not yet supported in js");
    }

    pub(crate) fn vm_exceptionref(&self) -> VMExceptionRef {
        unimplemented!("ExceptionRef is not yet supported in js");
    }

    pub(crate) unsafe fn from_vm_exceptionref(
        _store: &mut impl AsStoreMut,
        _vm_exceptionref: VMExceptionRef,
    ) -> Self {
        unimplemented!("ExceptionRef is not yet supported in js");
    }

    pub fn is_from_store(&self, _store: &impl AsStoreRef) -> bool {
        true
    }
}
