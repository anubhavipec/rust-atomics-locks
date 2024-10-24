use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};



pub fn threading1(){

    let v = vec![5,2,4];

    thread::spawn(move || {
        for i in 1..10{
            println!("number {i} from spawned thread");
            let  size = v.len();
            println!("first of vec is {size}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for j in 1..10 {
        println!("number {j} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn channels(){

    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("HEllo");
        tx.send(val).unwrap();
    });

    let received = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    println!("Got {received}");

}

pub fn mutex_usage(){

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result : {} ",*counter.lock().unwrap());
}