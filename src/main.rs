mod atomic;

use std::cell::Cell;
use std::os::unix::raw::time_t;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicI32;
use std::thread;
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::time::{sleep,Duration};
use crate::atomic::{progress_repo, stop_flag_use};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    //let (tx, mut rx) = mpsc::channel(32);
    let a = 3;
    let mut b = 5;
    f(&a,&mut b);
    //stop_flag_use();
    progress_repo();

    // let mut l = Mutex::new(3);

    // let counter = Arc::new(Mutex::new(3));
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //
    //     let counter_clone = Arc::clone(&counter);
    //     let handle = tokio::spawn(async move  {
    //         increment_and_do_stuff(&counter_clone).await;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.await.unwrap();
    // }



    // println!("{:?}", *counter.lock().unwrap())
}

fn f(a:&i32,b:&mut i32) {
    let before = *a;
    *b +=1;
    let after = *a;
    println!("a is {}, b is{}",*a,*b);
    if before!=after{
        println!("Check happend");
    }
}

fn f2(a:&Cell<i32>,b: &Cell<i32>) {
    let r = AtomicI32::new(2);
    let before = a.get();
    b.set(b.get()+1);
    let after = a.get();
    if before!= after {
        println!("Checked the in f2");
    }

}


async fn do_something_async() {
    sleep(Duration::from_secs(1)).await;
}
async fn increment_and_do_stuff(mutex : &Mutex<i32>) {
    {
        let mut lock = mutex.lock().unwrap();
        *lock += 1;
    }

    do_something_async().await;
}





#[derive(Debug)]
enum Command {
    Get{
        key: String,
    },
    SET {
        key: String,
        value: Bytes,
    }
}

