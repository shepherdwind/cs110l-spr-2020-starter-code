
use crossbeam_channel::bounded;
use std::{thread, time, fmt::Display};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + Display + Copy + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::new();
    let (s, r) = bounded(0);
    let (s2, r2) = bounded(0);
    let total = input_vec.len();
    let task_num = if total > num_threads { num_threads } else { total };

    let mut tasks = Vec::new();
    for i in 0..task_num {
        let sender = s.clone();
        let receiver = r2.clone();
        let task = thread::spawn(move || {
            loop {
                // when parent thread is stop to send, will drop sender
                // then thread will resume, and get an error, thread should be stop
                // after thread stoped, parent join can receiver message
                let data = receiver.recv();
                if data.is_err() {
                    break;
                }

                println!("task {}: is start", i);
                let ret = f(data.unwrap());
                sender.send(ret).unwrap();
            }
        });
        tasks.push(task);
    }

    for _ in 0..task_num {
        s2.send(input_vec.pop().unwrap()).unwrap();
    }

    while output_vec.len() < total {
        output_vec.push(r.recv().unwrap());
        // if task is not finished, after recv data from channels,
        // then send an task to thread pool
        if let Some(item) = input_vec.pop() {
            s2.send(item).unwrap();
        }
    }

    drop(s2);
    for child in tasks {
        child.join().expect("oops! the child thread panicked");
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];

    let start = time::Instant::now();
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });

    println!("squares: {:?}, time: {:.2?}", squares, start.elapsed());
}
