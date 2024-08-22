use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn progress_repo() {

    let num_done = &AtomicUsize::new(0);

    /*
    As a move closure, it moves (or copies) its captures rather than borrowing them, giving it a copy of t.
    Because it also captures num_done, we’ve changed that variable to be a reference,
    since we do still want to borrow that same AtomicUsize.
    Note that the atomic types do not implement the Copy trait,
    so we’d have gotten an error if we tried to move one into more than one thread.
     */

    let main_thread = &thread::current();
    /*
    We’ve obtained a handle to the main thread through thread::current(),
    that’s now used by the background thread to unpark the main thread after every status update.
    The main thread now uses park_timeout rather than sleep, such that it can be interrupted.
    Now, any status updates are immediately reported to the user,
    while still repeating the last update every second to show the program is still running.
     */
    thread::scope(|s| {
        for t in 0..4{
            s.spawn(move||{
                for i in 0..25 {
                    process_item();
                    let _ = t*1;
                    num_done.fetch_add(1,Relaxed);
                    sleep(Duration::from_millis(50));
                    main_thread.unpark();
                }
            });

        }

        loop {
            let n = num_done.load(Relaxed);
            if n == 100{break;}
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });
}

fn process_item(){
    let x = 524;
    let y= 29487;
    // sleep(Duration::from_secs(3));
    println!("{}", {x*y});
}