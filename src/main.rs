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
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from main thread");

    // introduces a forced sequence with main
    // t1, then main where main, then does t2
    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread");
    let id = thread::current().id();
    println!("My id: {id:?}");
}
