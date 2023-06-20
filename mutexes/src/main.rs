use std::sync::{Mutex,Arc};
use std::{thread,time};
use rand::distributions::{Alphanumeric, DistString};

fn main() {

    let mut queue: Vec<String> = Vec::new();
    let mutex = Arc::new(Mutex::new(queue));

    let mut handles = vec![];

    {
        println!("Seeding");
        let mut q = mutex.lock().unwrap();
        for _ in 1..1000 {
            let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
            q.push(string);
        }
    }


    for thread_num in 0..10 {
        let cloned_mutex = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let ten_millis = time::Duration::from_millis(10);
            loop {
                {
                    println!("Waiting in thread {}", thread_num);
                    let mut q = cloned_mutex.lock().unwrap();
                    
                    if q.len() > 0 {
                        let item = q.pop().unwrap();
                        println!("Got {} in thread {}", item, thread_num);
                    } else {
                        //no more work to do
                        println!("No more work, thread {} exiting", thread_num);
                        break;
                    }
                }
                thread::sleep(ten_millis);
            }

        });
        handles.push(handle);
    }



    for handle in handles {
        handle.join().unwrap();
    }
    println!("All done!");


}
