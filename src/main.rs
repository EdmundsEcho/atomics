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
    let numbers = vec![1, 2, 3];

    // join a single thread to main that counts to 3
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    println!("Hello from main thread");
}
