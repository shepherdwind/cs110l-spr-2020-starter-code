
use crossbeam_channel::bounded;
use threadpool::ThreadPool;
use std::{thread, time, fmt::Display};

fn parallel_map<T, U, F>(input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + Display + Copy + 'static,
    U: Send + 'static + Default,
{
    let (s, r) = bounded(0);
    let pool = ThreadPool::new(num_threads);
    let len = input_vec.len();

    for item in input_vec {
        let sender = s.clone();
        pool.execute(move || {
            println!("task {}: is start", &item);
            let ret = f(item);
            sender.send(ret).unwrap();
        });
    }

    r.iter().take(len).collect()
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
