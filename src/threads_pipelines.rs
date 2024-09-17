extern crate crossbeam;
extern crate crossbeam_channel;

use std::{path::Path, thread,
sync::Mutex, time::Duration};
use std::fs::File;
use std::io::{BufReader, Read, Error, ErrorKind};
use std::sync::{mpsc::channel};
use crossbeam_channel::{bounded, unbounded};
use lazy_static::lazy_static;
use ring::digest::{Context, Digest, SHA256};
use threadpool::ThreadPool;
use walkdir::WalkDir;


// Global variable
lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn create_parallel_pipeline() {
    let (sender_1, receiver_1) = bounded::<usize>(1);
    let (sender_2, receiver_2) = bounded::<usize>(1);
    let n_messages: usize = 4;
    let n_workers: usize = 2;

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            for i in 0..n_messages {
                sender_1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // Close the channel - necessary for exit the for loop in the worker
            drop(sender_1);
        });

        // Parallel processing by 2 threads
        for _ in 0..n_workers {
            // Send to sink, receive from source
            let (sender, receiver) = (sender_2.clone(), receiver_1.clone());
            // Spawn workers in separated threads
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                // Receive until channel closes
                for msg in receiver.iter() {
                    println!("Worker {:?} received {}", thread::current().id(), msg);
                    sender.send(msg * 2).unwrap();
                }
            });
        }
        // Close the channel
        drop(sender_2);

        // Sink
        for msg in receiver_2.iter() {
            println!("sink received {}", msg);
        }
    }).unwrap();
}

pub fn spawn_a_short_live_thread() {
    let arr:&[i32;4] = &[1, 25, -4, 10];
    let max: Option<i32> = find_max(arr);
    assert_eq!(max, Some(25));
}

pub fn pass_data_between_threads() {
    let (sender, receiver) = unbounded();
    let n_messages: usize = 5;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_messages {
                sender.send(i).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }).unwrap();
    for _ in 0..n_messages {
        let msg: usize = receiver.recv().unwrap();
        println!("Received {}", msg);
    }
}

pub fn maintain_global_mutable_state() -> Result<(), Error> {
    insert_fruit("apple")?;
    insert_fruit("orange")?;
    insert_fruit("banana")?;
    {
        let db = FRUIT.lock()
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to acquire MutexGuard"))?;
        db.iter().enumerate().for_each(|(i, item)|
        println!("{}, {}", i, item));
    }
    insert_fruit("kiwi")?;
    Ok(())
}

pub fn calculate_sha256(path: &str, extension: &str) -> Result<(), Error> {
    let pool: ThreadPool = ThreadPool::new(num_cpus::get());
    let (sender, receiver) = channel();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(
            |ext| ext.to_str()) == Some(extension)
        &&  !e.path().is_dir()) {
            let path = entry.path().to_owned();
            let sender = sender.clone();
            pool.execute(move || {
                let digest = compute_digest(path);
                sender.send(digest).expect("Could not send data!");
        });
    }
    drop(sender);
    for t in receiver.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}

// auxiliar functions
fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }
    let mid: usize = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_left = s.spawn(|_| find_max(left));
        let thread_right = s.spawn(|_| find_max(right));

        let max_left: i32 = thread_left.join().unwrap()?;
        let max_rigth: i32 = thread_right.join().unwrap()?;

        Some(max_left.max(max_rigth))
    }).unwrap()
}

fn insert_fruit(fruit: &str) -> Result<(), Error> {
    let mut db = FRUIT.lock()
        .map_err(|_| Error::new(ErrorKind::Other,"Failed to acquire MutexGuad"))?;
    db.push(String::from(fruit));
    Ok(())
}

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader: BufReader<File> = BufReader::new(File::open(&filepath)?);
    let mut context: Context = Context::new(&SHA256);
    let mut buffer:[u8; 1024] = [0; 1024];

    loop {
        let count: usize = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok((context.finish(), filepath))

}