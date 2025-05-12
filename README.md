# Notes
## 1. [Complex types with Rust’s FFI](https://medium.com/jim-fleming/complex-types-with-rust-s-ffi-315d14619479)
This article is a bit old, but it was a good starting point. We start by creating a simple Counter class, which we'll try to use on C#.

```rust
pub struct Counter {
    val: u32
}

impl Counter {
    pub fn new(val: u32) -> Counter {
        Counter{val: val}
    }

    pub fn get(&self) -> u32 {
        self.val
    }

    pub fn incr(&mut self, by: u32) -> u32 {
        self.val += by;
        self.val
    }

    pub fn decr(&mut self, by: u32) -> u32 {
        self.val -= by;
        self.val
    }
}
```

> **From the article**
> We utilize Rust for memory allocation to create our counter on the heap, using [**_Box_**](https://doc.rust-lang.org/std/boxed/struct.Box.html) ([book](https://doc.rust-lang.org/book/ch15-01-box.html)) then [**_transmute_**](https://doc.rust-lang.org/std/mem/fn.transmute.html) this box into a raw pointer. This trickery avoids having to manually allocate the memory and seems to be [the most canonical way](http://www.reddit.com/r/rust/comments/2fmvcy/rust_ffi_and_opaque_pointer_idiom/) to allocate the counter. Our destructor works similarly by transmuting the counter’s pointer back into a **_Box_** then letting it automatically [drop](https://doc.rust-lang.org/std/ops/trait.Drop.html).

**Transmute has been deprecated, but I'll leave it just to keep track of the process**
What I've found to be safer, instead, is using:

`Box::into_raw(Box::new(Counter::new(args)))`

```rust
#![allow(non_snake_case)]

mod counter;

use counter::Counter;
use std::mem::transmute;

#[no_mangle]  
pub extern "C" fn createCounter(args: Args) -> *mut Counter {  
    let _counter = Box::into_raw(Box::new(Counter::new(args)));  
    _counter  
}

#[no_mangle]
pub extern fn getCounterValue(ptr: *mut Counter) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.get()
}

#[no_mangle]
pub extern fn incrementCounterBy(ptr: *mut Counter, by: u32) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.incr(by)
}

#[no_mangle]
pub extern fn decrementCounterBy(ptr: *mut Counter, by: u32) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.decr(by)
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
```
#### ❓ Why `decrementCounterBy` cannot be fully safe:
```rust
pub extern fn decrementCounterBy(ptr: *mut Counter, by: u32) -> u32 {`
    let mut _counter = unsafe { &mut *ptr };`
    _counter.decr(by)`
}
```

The use of `unsafe { &mut *ptr }` is **unavoidable** in safe Rust because:
#### 🚨 Raw pointers (`*mut T`) are inherently unsafe: (ChatGPT)
- Rust can't verify at compile time whether:
    
    - `ptr` is non-null.
        
    - `ptr` is aligned correctly.
        
    - `ptr` points to a valid `Counter` (i.e., it was originally created from a `Box<Counter>`).
        
    - There are no aliasing violations (e.g., two mutable references to the same object).
        
All of these are **safety guarantees** required to make `&mut T` valid in Rust.
#### 👾 Why not using Rust References &?
Using Rust References is more idiomatic but less safe because we can't guarantee that the reference is not null. 
```rust 
#[ffi_function]
#[no_mangle]
pub extern "C" fn getCounterPositions(counter: &Counter, out_len: &mut u32) -> *const Vector2 {  
	let positions = counter.get_positions();  
	*out_len = positions.len() as u32;  
	positions.as_ptr()  
}
```
#### 😱 Error Handling
```rust
static mut LAST_ERROR: Option<String> = None;

#[no_mangle]
pub extern "C" fn createCounter(args: Args) -> *mut Counter {
    match Counter::try_new(args) {
        Ok(counter) => Box::into_raw(Box::new(counter)),
        Err(e) => {
            unsafe {
                LAST_ERROR = Some(e.to_string());
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn get_last_error() -> *const c_char {
    unsafe {
        match &LAST_ERROR {
            Some(msg) => msg.as_ptr() as *const c_char,
            None => std::ptr::null(),
        }
    }
}
```

## 2. Auto-generate bindings using [Interoptopus](https://github.com/ralfbiedert/interoptopus)


[UnityRustNative](https://github.com/rhedgeco/UnityRustNative)

I wanted a hybrid approach between opaque pointers and transparent structures.
**Counter** is tagged *opaque*, so that it can be instantiated in C#, and its method called by passing a reference to the instance registered at that pointer.

If I want to access the inner data of Counter I can use a different structure, which acts as a snapshot of the original Counter, i.e.: **CounterData**.

**Interoptopus** is going to generate bindings for structs marked as **opaque** 
```rust
[ffi_type(opaque)]
```
as raw pointers (IntPtr) in C#. While for structs not marked as opaque it will try to marshal the structure, but only if the struct is passed by value and not by reference.
Read this [ChatGPT Research](https://chatgpt.com/s/dr_681b367604d4819180c09154ca4daefb) research for a deeper explanation. I hope that ChatGPT is not hallucinating, it looks quite reasonable though.

If I try to write something like:

```rust
pub extern "C" fn getCounterData(ptr: *const Counter) -> *mut CounterData {..}
```

Interoptopus will generate this binding:

```csharp
[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "getCounterData")]
public static extern IntPtr getCounterData(IntPtr ptr);
```
#### 🤖 Code Example
*counter.rs*

```rust
use interoptopus::ffi_type;  
  
#[ffi_type()]  
#[repr(C)]  
pub struct Args {  
    init: u32,  
    by: u32,  
}  
  
// Transparent copyable struct  
#[ffi_type]  
#[repr(C)]  
pub struct CounterData {  
    pub val: u32,  
    pub by: u32,  
}  
  
#[ffi_type(opaque)]  
#[repr(C)]  
#[derive(Copy, Clone)]  
pub struct Counter {  
    val: u32,  
    by: u32,  
}

[...]
```

*lib.rs*

```rust
[...]

#[ffi_function]  
#[no_mangle]  
pub extern "C" fn createCounter(args: Args) -> *mut Counter {  
    let _counter = Box::into_raw(Box::new(Counter::new(args)));  
    _counter  
}
  
#[ffi_function]  
#[no_mangle]  
pub extern "C" fn getCounterData(ptr: *const Counter) -> CounterData {  
	if ptr.is_null() {  
	    return 0;  
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

[...]
```

*Counter.cs*

```csharp
[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "createCounter")]  
public static extern IntPtr createCounter(Args args);  
  
[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "getCounterData")]  
public static extern CounterData getCounterData(IntPtr ptr);  
  
[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "getCounterValue")]  
public static extern uint getCounterValue(IntPtr ptr);
```
