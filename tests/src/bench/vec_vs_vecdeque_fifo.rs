#[cfg(test)]
mod benchmarks {
    use std::time::Duration;
    use rustest_core::queues::{FIFOQueue, VFIFOQueue};
    use std::time::Instant;
    use std::thread;

    #[test]
    fn benchmark() {
        const ITERATIONS: i32 = 100000000;

        let a = thread::spawn(|| {
            let deque = get_fifo_elapsed_time(ITERATIONS);
            let vec = get_vec_fifo_elapsed_time(ITERATIONS);

            println!("VecDeque: {:?}\nVec: {:?}\n", deque, vec);
        });
        let b = thread::spawn(|| {
            let vec = get_vec_fifo_elapsed_time(ITERATIONS);
            let deque = get_fifo_elapsed_time(ITERATIONS);

            println!("Vec: {:?}\nVecDeque: {:?}\n", vec, deque);
        });

        let _ = a.join();
        let _ = b.join();
    }

    fn get_fifo_elapsed_time(range: i32) -> Duration {
        let mut fifo: FIFOQueue<i32> = FIFOQueue::new();
        let now = Instant::now();

        for i in 0..range {
            fifo.enqueue(i);
        }

        while let Some(_i) = fifo.dequeue() {
            ()
        }

        let elapsed = now.elapsed();
        return elapsed
    }

    fn get_vec_fifo_elapsed_time(range: i32) -> Duration {
        let mut fifo: VFIFOQueue<i32> = VFIFOQueue::new();
        let now = Instant::now();

        for i in 0..range {
            fifo.enqueue(i);
        }

        while let Some(_i) = fifo.dequeue() {
            ()
        }

        let elapsed = now.elapsed();
        return elapsed
    }
}