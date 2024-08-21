use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn progress_repo() {

    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread to process all 100 items
        s.spawn(||{
            for i in 0..100 {
                process_item();
                num_done.store(i+1,Relaxed);
                sleep(Duration::from_millis(50));
            }
        });
        loop {
            let n = num_done.load(Relaxed);
            if n == 100{break;}
            println!("Working.. {n}/100 done");
            sleep(Duration::from_secs(1));
        }
    });
}

fn process_item(){
    let x = 524;
    let y= 29487;
    // sleep(Duration::from_secs(3));
    println!("{}", {x*y});
}