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
    // Mutex
    //
    // Here we let 10 threads mutate a value.  Each one gets to augment the value by 100.
    // The total edits add up to 1000.
    //
    #[derive(Debug, PartialEq, Eq)]
    struct Acc {
        count: i32,
        log: String,
    }
    impl Acc {
        fn new() -> Self {
            Acc {
                count: 0,
                log: "".into(),
            }
        }
        fn add_one(&mut self, entry: char) {
            self.count += 1;
            self.log.push(entry);
        }
    }
    let n = Mutex::new(Acc::new());
    // creating a separate, single parent scope allows us to use "scoped" threads
    // where the child threads can each share a value.
    use std::time::Duration;
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    guard.add_one('0');
                }
                drop(guard); // change to parallel waiting
                thread::sleep(Duration::from_secs(1));
            });
        }
    });

    println!("Hello from main thread");

    dbg!(&n);
    assert_eq!(n.into_inner().unwrap().count, 1000);
}
