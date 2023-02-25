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
    // Prevent any one thread from owning a value using Static
    // This value exists even before main is called.
    //
    // 🔑 Normally we cannot borrow values because a thread can live as long as "forever" (from
    // main's perpsective).  Static ensure what is being borrowed will in fact live forever.
    //
    // Note: borrowing is the only way to share a resource.
    //
    static X: [i32; 3] = [1, 2, 3];

    let t1 = thread::spawn(|| dbg!(&X));
    let t2 = thread::spawn(|| dbg!(&X));

    t1.join().unwrap();
    t2.join().unwrap();

    //
    // Another way to accomplish the same, memory leaking.
    //
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    let t1 = thread::spawn(move || dbg!(x)); // copy a borrow
    let t2 = thread::spawn(move || dbg!(x)); // copy a borrow

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Hello from main thread");
}
