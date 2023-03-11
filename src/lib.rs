#![no_std]

use core::ops::{Deref, DerefMut};

/// A wrapper to prevent the compiler from calling T’s destructor. This wrapper is transparent (zero-cost).
/// It can be thought of as a variant of `core::mem::ManuallyDrop` with compile-time checking.
///
/// If you drop a value wrapped by it, the compilation will fail. To drop the value, you must explicitly call `Undroppable::drop`.
/// Or to drop without calling the destructor, you can use `Undroppable::forget`.
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Undroppable<T: ?Sized>(T);

impl<T: ?Sized> Undroppable<T> {
    const PANIC: () = panic!("Undroppable!");
}

impl<T> Undroppable<T> {
    /// Wrap a value in `Undroppable`.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Drop the inner value explicitly.
    #[inline(always)]
    pub fn drop(mut this: Self) {
        let inner_ptr = core::ptr::addr_of_mut!(this.0);
        core::mem::forget(this);
        unsafe { core::ptr::drop_in_place(inner_ptr) };
    }

    /// Forget the inner value explicitly.
    /// This is equivalent to calling `core::mem::forget`.
    #[inline(always)]
    pub const fn forget(this: Self) {
        core::mem::forget(this);
    }

    /// Extracts the inner value from the `Undroppable` container.
    #[inline(always)]
    pub fn into_inner(mut this: Self) -> T {
        let inner = unsafe { Self::take(&mut this) };
        Self::forget(this);
        inner
    }

    /// Extracts the inner value from the `Undroppable` container without dropping the container.
    #[inline(always)]
    pub unsafe fn take(this: &mut Self) -> T {
        core::ptr::read(&this.0)
    }
}

impl<T: ?Sized> Deref for Undroppable<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for Undroppable<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ?Sized> Drop for Undroppable<T> {
    fn drop(&mut self) {
        Self::PANIC
    }
}
