use crate::extension_traits::vec::UnshiftTrait;

pub struct FIFOQueue<T: Clone> {
    list: Vec<T>
}

impl<T: Clone> FIFOQueue<T> {
    /**-
    Instantiates a new instance of FIFOQueue
    */
    pub fn new() -> Self {
        FIFOQueue {
            list: Vec::new()
        }
    }

    /**
    Gets the next element from the queue to be attended.

    The element will be removed from the queue as its fetched.

    Note that `T` must implement `Clone` trait (so that the element can
    be successfully removed from the queue).
     */
    pub fn dequeue(&mut self) -> Option<T> {
        self.list.unshift()
    }

    /**
    Adds a new element to the end of the queue.
    */
    pub fn enqueue(&mut self, item: T) -> () {
        self.list.push(item);
    }

    /**
    Gets the next element from the queue without removing it.
    */
    pub fn next(&self) -> Option<&T> {
        self.list.first()
    }

    /**
    Gets the size of the queue
    */
    pub fn size(&self) -> usize {
        self.list.len()
    }
}

impl<T: Clone> From<Vec<T>> for FIFOQueue<T> {
    fn from(v: Vec<T>) -> Self {
        FIFOQueue {
            list: v
        }
    }
}