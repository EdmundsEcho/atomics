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
    let numbers = Vec::from_iter(0..=1000);

    //
    // Return a value from the thread...
    // Note: spawn is sugar for:
    // `thread::Builder::new().spawn().unwrap()`
    //
    let builder = thread::Builder::new().name("Average".into());
    let result = builder
        .spawn(move || {
            let len = numbers.len();
            let sum = numbers.iter().sum::<usize>();
            panic!("Something went wrong");
            sum / len
        })
        .unwrap()
        .join()
        .unwrap();

    println!("Hello from main thread");
    println!("Result: {result}");
}
