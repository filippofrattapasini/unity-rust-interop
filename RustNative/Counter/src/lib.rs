#![allow(non_snake_case)]

pub mod bind;
mod counter;

use interoptopus::ffi_function;

use crate::counter::{CounterData, Vector2};
use counter::{Args, Counter};

#[ffi_function]
#[no_mangle]
pub extern "C" fn createCounter(args: Args) -> *mut Counter {
    let _counter = Box::into_raw(Box::new(Counter::new(args)));
    _counter
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn destroyCounter(ptr: *mut Counter) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn getCounterData(ptr: *const Counter) -> CounterData {
    if ptr.is_null() {
        return CounterData { val: 0, by: 0 };
    }

    let _counter = unsafe { &*ptr };
    CounterData {
        val: _counter.get(),
        by: _counter.get_by(),
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn getCounterValue(ptr: *mut Counter) -> u32 {
    if ptr.is_null() {
        return 0;
    }

    let mut _counter = unsafe { &mut *ptr };
    _counter.get()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn incrementCounter(ptr: *mut Counter) -> u32 {
    if ptr.is_null() {
        return 0;
    }

    let mut _counter = unsafe { &mut *ptr };
    _counter.incr()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn decrementCounter(ptr: *mut Counter) -> u32 {
    if ptr.is_null() {
        return 0;
    }

    let mut _counter = unsafe { &mut *ptr };
    _counter.decr()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn incrementCounterBy(ptr: *mut Counter, by: u32) -> u32 {
    if ptr.is_null() {
        return 0;
    }

    let mut _counter = unsafe { &mut *ptr };
    _counter.incr_by(by)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn decrementCounterBy(ptr: *mut Counter, by: u32) -> u32 {
    if ptr.is_null() {
        return 0;
    }

    let mut _counter = unsafe { &mut *ptr };
    _counter.decr_by(by)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn incrementCounterByMany(
    ptr: *mut Counter,
    bys_ptr: *const u32,
    bys_len: u32,
) -> u32 {
    if ptr.is_null() {
        return 0;
    }

    let mut _counter = unsafe { &mut *ptr };
    let bys = unsafe { std::slice::from_raw_parts(bys_ptr, bys_len as usize) };

    _counter.incr_by_many(bys)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn decrementCounterByMany(
    ptr: *mut Counter,
    bys_ptr: *const u32,
    bys_len: u32,
) -> u32 {
    if ptr.is_null() {
        return 0;
    }

    let mut _counter = unsafe { &mut *ptr };
    let bys = unsafe { std::slice::from_raw_parts(bys_ptr, bys_len as usize) };
    _counter.decr_by_many(bys)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn getCounterPositions(
    counter: *const Counter,
    out_len: &mut u32,
) -> *const Vector2 {
    if counter.is_null() {
        return std::ptr::null();
    }

    let counter = unsafe { &*counter };
    let positions = counter.get_positions();
    *out_len = positions.len() as u32;
    positions.as_ptr()
}

// Using Rust References is more idiomatic but less safe
// because we can't guarantee that the counter is not null.
// #[ffi_function]
// #[no_mangle]
// pub extern "C" fn getCounterPositions(counter: &Counter, out_len: &mut u32) -> *const Vector2 {
//     let positions = counter.get_positions();
//     *out_len = positions.len() as u32;
//     positions.as_ptr()
// }
