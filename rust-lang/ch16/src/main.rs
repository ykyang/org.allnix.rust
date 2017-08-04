use std::thread;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move||{
        println!("Here is a vector: {:?}", v);
    });

//    drop(v);
    handle.join();

    let handle = thread::spawn(||{
       for i in 1..10 {
           println!("Hi number {} from the spawned thread!", i);
       }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
    }

    handle.join();
}