
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
    let total = input_vec.len();

    let mut tasks = Vec::new();
    for item in input_vec {
        let sender = s.clone();
        let task = thread::spawn(move || {
            println!("task {}: run", &item);
            let ret = f(item);
            println!("task {}: run end", item);
            sender.send(ret).unwrap();
        });
        tasks.push(task);
    }

    for _ in 0..total {
        output_vec.push(r.recv().unwrap());
    }
    for child in tasks {
        child.join().expect("oops! the child thread panicked");
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];

    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
