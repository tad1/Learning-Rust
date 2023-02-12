// As mentioned in rustling
// We can have shared memory, and messeage channels

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn simple_test(){
    let (tx, rx) = mpsc::channel();

    // We start a new thread
    thread::spawn(move || {
        let val = String::from("hi");
        thread::sleep(Duration::from_secs(2));
        tx.send(val).unwrap();
    });

    // And we wait until it ends
    let recived = rx.recv().unwrap();

    // But can we create threads in threads?
    println!("Got: {}", recived);
}


fn threads_in_threads(){

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi from thread lv1");
        let tx2 = tx.clone();
        tx.send(val).unwrap();

        thread::spawn(move || {
            let val = String::from("hello from thread lv2");
            tx2.send(val).unwrap();
        })
    });

    let recived_1 = rx.recv().unwrap();
    let recived_2 = rx.recv().unwrap();

    println!("rc1: {}", recived_1);
    println!("rc2: {}", recived_2);
}

fn ping_pong(tx: Sender<String>, rx: Receiver<String>){
    
    loop {
        let recived = rx.recv().unwrap();
        if recived == String::from("Stop") {
            return;
        } else if recived == String::from("Ping") {
            println!("Pong");
            let val = String::from("Pong");
            tx.send(val).unwrap();
        } else if recived == String::from("Pong") {
            println!("Ping");
            let val = String::from("Ping");
            tx.send(val).unwrap();
        }
    }
}

fn make_ping_pong(){
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let txc = tx.clone();
    let tx2c = tx2.clone();

    thread::spawn(move || {
        ping_pong(tx, rx2);
    });

    thread::spawn(move || {
        ping_pong(tx2, rx);
    });

    txc.send(String::from("Ping")).unwrap();
    //tx2c.send(String::from("Ping")).unwrap();
    thread::sleep(Duration::from_nanos(1));
    txc.send(String::from("Stop")).unwrap();  

}

fn main(){
    threads_in_threads();
    // That's a lot!
    make_ping_pong();
}