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

    // Tips and tricks - avoid needing to rename the clone
    let a = Arc::new([1, 2, 3]);
    let t1 = thread::spawn({
        let a = a.clone(); // shadow a in a separate scope so that...
                           // clone before move
        move || {
            dbg!(a);
        }
    });
    t1.join().unwrap();
    dbg!(a); // ... we can use a again, but pointing to the original ref created using Arc

    //
    // Reminder: Ownership usually decides who called `drop`.  The Arc tracks when there are
    // zero references at which point call drop. Ownership "emulation" is a reference to something
    // in the thread that will never call drop if the thread keeps going "forever".
    // Arc is a way to make sure the lifetime of the borrow is less than or equal to that of the
    // main function... where the Arc was instantiated.
    //
    println!("Hello from main thread");
}
