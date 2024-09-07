use crate::extension_traits::vec::UnshiftTrait;

pub struct VFIFOQueue<T> {
    list: Vec<T>
}

impl<T> VFIFOQueue<T> {
    /**-
        Instantiates a new instance of VFIFOQueue
     */
    pub fn new() -> Self {
        VFIFOQueue {
            list: Vec::new()
        }
    }

    /**
    Gets the next element from the queue to be attended.

    The element will be removed from the queue as its fetched.
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