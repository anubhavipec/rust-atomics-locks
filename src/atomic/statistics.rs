use std::sync::atomic::{AtomicU64, AtomicUsize,Ordering};
use std::thread;
use tokio::time::Duration;
use tokio::time::Instant;

use crate::atomic::progress_reporting::process_item;
pub fn statistics_report(){

    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);


    thread::scope(|s| {
        for t in 0..4{
            s.spawn(move || {
                for i in 0..25{
                    let start = Instant::now();
                    process_item();
                    println!("{}", i*t);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1.try_into().unwrap(), Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken,Ordering::Relaxed);

                }
            });
        }

        loop{
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {break;}
            if n == 0 {
                println!("Working ... nothing done yet");
            } else {
                println!("Working ... {n}/100 done {:?} average, {:?} peak", total_time/n.try_into().unwrap(), max_time);

            }
            thread::sleep(Duration::from_secs(1));
        }
        
    });
    println!("Done")
}
