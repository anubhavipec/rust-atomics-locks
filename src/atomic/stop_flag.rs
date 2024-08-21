use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn stop_flag_use(){

    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed){
            sleep(Duration::from_secs_f32(2 as f32));
            println!("hello");
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("command: helps, stop"),
            "stop" => break,
            cmd=> println!("unknown command :{cmd:?}"),
        }
    }

    STOP.store(true,Relaxed);
    background_thread.join().unwrap();
}