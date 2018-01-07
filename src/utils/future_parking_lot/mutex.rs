use parking_lot;
use futures::{Future, Async, Poll};
use std::ops::{Deref};

pub struct Mutex<T> {
    inner: parking_lot::Mutex<T>
}

pub struct AsyncMutexGuard<'a, T: 'a> {
    outer: &'a Mutex<T>
}

impl <'a, T> Future for AsyncMutexGuard <'a, T> {
    type Item = parking_lot::MutexGuard<'a, T>;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.outer.inner.try_lock() {
            Some(guard) => Ok(Async::Ready(guard)),
            None => Ok(Async::NotReady)
        }
    }
}

impl <T> Mutex <T> {
    pub fn new(val: T) -> Mutex<T> {
        Mutex {
            inner: parking_lot::Mutex::new(val)
        }
    }
    pub fn lock_async(&self) -> AsyncMutexGuard<T> {
        AsyncMutexGuard {
            outer: self
        }
    }
}

impl <T> Deref for Mutex<T> {
    type Target = parking_lot::Mutex<T>;
    #[inline]
    fn deref(&self) -> &parking_lot::Mutex<T> {
        &self.inner
    }
}