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
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from main thread");
}

fn f() {
    println!("Hello from another thread");
    let id = thread::current().id();
    println!("My id: {id:?}");
}
