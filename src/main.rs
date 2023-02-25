#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, Thread},
};

fn main() {
    //
    // Ref counting is a way to share ownership.  The value of sharing ownership in a multithreaded
    // context is that multiple threads can share the same value (avoiding new memory allocations
    // for each thread). However, it turns out that counting references to the value is not
    // thread-safe.  So we use `Arc` instead of `Rc`.
    //
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    let t1 = thread::spawn(move || dbg!(a));
    let t2 = thread::spawn(move || dbg!(b));

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Hello from main thread");
}
