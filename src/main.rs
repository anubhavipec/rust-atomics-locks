use std::os::unix::raw::time_t;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::time::{sleep,Duration};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut l = Mutex::new(3);

    let counter = Arc::new(Mutex::new(3));
    let mut handles = vec![];

    for _ in 0..10 {

        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move  {
            increment_and_do_stuff(&counter_clone).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }



    println!("{:?}", *counter.lock().unwrap())
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


// fn f() {
//     println!("Hello from another thread!");
//     let id = thread::current().id();
//     println!("This is my thread id: {id:?}");
// }


