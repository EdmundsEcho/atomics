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
    time::Duration,
};

fn main() {
    //
    // Mutex & thread parking
    //
    // Producer adds a new item every second.
    // Consumer consumes items, but waits (parked) when the Vec is empty
    //

    let queue = Mutex::new(VecDeque::new());

    // This means that in our example above it’s important that we only park the thread if we’ve
    // seen the queue is empty, rather than park it after every processed item.

    thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park(); // critical that only happens when empty
            }
        });
        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Hello from main thread - use Condvar instead");
}
