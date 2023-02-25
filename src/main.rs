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
    // Thus far, our ability to share information between threads has had to navigate the borrowing
    // rules... all so we know who calls drop AND to ensure that the lifetime of what we borrow
    // lives as long as the thread.
    // 1. Static value created before main
    // 2. Arc created in main
    // 3. Value that leaks memory (never calls drop)
    // 4. Scoped threads
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
