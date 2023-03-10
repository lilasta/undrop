#![no_std]

use core::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Undroppable<T: ?Sized>(T);

impl<T> Undroppable<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    pub fn drop(this: Self) {
        drop(Self::into_inner(this))
    }

    pub const fn forget(this: Self) {
        core::mem::forget(this);
    }

    pub fn into_inner(mut this: Self) -> T {
        let inner = unsafe { Self::take(&mut this) };
        Self::forget(this);
        inner
    }

    pub unsafe fn take(this: &mut Self) -> T {
        core::ptr::read(&this.0)
    }
}

impl<T: ?Sized> Deref for Undroppable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for Undroppable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Panic<T: ?Sized>(T);

impl<T: ?Sized> Panic<T> {
    const PANIC: () = panic!("Undroppable!");
}

impl<T: ?Sized> Drop for Undroppable<T> {
    fn drop(&mut self) {
        Panic::<T>::PANIC
    }
}
