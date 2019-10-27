#![no_std]

use shared_bus::BusMutex;

pub struct NoopBusMutex<T> {
	data: T
}

impl<T> BusMutex<T> for NoopBusMutex<T> {
	#[inline(always)]
	fn create(v: T) -> NoopBusMutex<T> {
		NoopBusMutex { data: v }
	}

	#[inline(always)]
	fn lock<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
		f(&self.data)
	}
}
