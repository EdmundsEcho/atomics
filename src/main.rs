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
    // Scoped threads
    //
    let numbers = vec![1, 2, 3];
    // creating a separate, single parent scope allows us to use "scoped" threads
    // where the child threads can each share a value.
    thread::scope(|s| {
        s.spawn(|| {
            println!("Length: {}", numbers.len()); // borrowed here
        });
        s.spawn(|| {
            // borrowed here
            for n in &numbers {
                println!("number: {n}");
            }
        });
    });

    //
    // Recall: Threads must abide by the usual lifetime hierarchy that comes to be by
    // defintion of how things get instantiated (e.g., values to parent struct get instantiated
    // first) { last: { value: first } }... such that if first is a borrow, given that first must
    // outlive last, last's lifetime **cannot exceed** life of first borrow.
    //
    // With threads we actually want to work-around this lifecycle... A. it's ok for the parent
    // to fully consume the threads (unlike parent struct that points to void values), B. the
    // parent wants to live last to benefit from all the work done by the threads.  To do this,
    // we need to work around the borrow checker.  We can do that by
    // 1. making parent value live forever (to match the potential forever of the the thread)
    //    * leak
    //    * static
    // 2. give each thread "fake" ownership of each value using Arc (ties lifetime of parent to
    //    thread; value won't get dropped until all threads have died)
    // 3. use a scoped thread where the lifetime of thread is tied to its parent AND its child
    //    threads are tied to the scoped thread (by way of auto joining).  Here, the child borrows
    //    are allowed to reference anything that outlives the scoped thread (parent)
    //
    println!("Hello from main thread");
}
