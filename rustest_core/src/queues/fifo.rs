use std::collections::VecDeque;

pub struct FIFOQueue<T> {
    list: VecDeque<T>
}

impl<T> FIFOQueue<T> {
    /**-
        Instantiates a new instance of FIFOQueue
     */
    pub fn new() -> Self {
        FIFOQueue {
            list: VecDeque::new()
        }
    }

    /**
    Gets the next element from the queue to be attended.

    The element will be removed from the queue as its fetched.
     */
    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    /**
    Adds a new element to the end of the queue.
     */
    pub fn enqueue(&mut self, item: T) -> () {
        self.list.push_back(item);
    }

    /**
    Gets the next element from the queue without removing it.
     */
    pub fn next(&self) -> Option<&T> {
        self.list.front()
    }

    /**
    Gets the size of the queue
     */
    pub fn size(&self) -> usize {
        self.list.len()
    }
}

impl<T> From<Vec<T>> for FIFOQueue<T> {
    fn from(v: Vec<T>) -> Self {
        FIFOQueue {
            list: VecDeque::from(v)
        }
    }
}

impl<T> From<VecDeque<T>> for FIFOQueue<T> {
    fn from(v: VecDeque<T>) -> Self {
        FIFOQueue {
            list: v
        }
    }
}