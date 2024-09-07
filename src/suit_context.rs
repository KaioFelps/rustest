use std::rc::Rc;
use rustest_core::queue::FIFOQueue;

type Procedure = Rc<dyn Fn()>;

/**
# Suit Context
A Suit Context represents a situation being tested. It can be declared with the `describe()`
function.

A SuitContext instance is composed by the following elements:
- **Before All Queue**

    A FIFO queue holding procedures (functions) that must run
    once at the start of the suit execution (before every test unit).
    Every item of this list will me **moved** to the Execution Stack
    before the first, or better, the first *before list *procedure
    even comes to be executed.

- **After All Queue**

    Just like the *Before All Queue*, but will run at the very end
    of the suit execution.

- **Before Each List**

    A read-only list of procedures to run before each suit's tests.
    Every item of this list will be **copied** to the Execution Stack
    and ran before each test's execution at the Execution Stack.

- **After Each List**

    Just like the *Before Each List*, but holds procedures (functions)
    to be executed after each test ends running.

- **Testes Queue**

    Finally, this (FIFO) queue holds every test to be run in this Suit
    Context. Each item of the queue will be `dequeue`d (moved) to the
    Execution Stack.
*/
pub struct SuitContext<T: Clone> {
    pub before_all_queue: FIFOQueue<Procedure>,
    pub after_all_queue: FIFOQueue<Procedure>,
    pub before_each_list: Vec<Procedure>,
    pub after_each_list: Vec<Procedure>,
    pub tests_queue: FIFOQueue<T>
}

impl<T: Clone> From<DraftSuitContext<T>> for SuitContext<T> {
    /**
    Converts this DraftSuitContext into a SuitContext.

    It's an alias for `SuitContext::seal(self)` (note that it's a consuming method).
     */
    fn from(value: DraftSuitContext<T>) -> Self {
        value.seal()
    }
}

/**
# Draft Suit Context
A draft Suit Context is the very first stage of a Suit Context.

A DraftSuitContext instance allows to manipulate every queue and lists as vectors.
After sealing it (thus getting a SuitContext), queues will behave like queues,
and lists will be read-only.
*/
pub struct DraftSuitContext<T: Clone> {
    pub before_all_queue: Vec<Procedure>,
    pub after_all_queue: Vec<Procedure>,
    pub before_each_list: Vec<Procedure>,
    pub after_each_list: Vec<Procedure>,
    pub tests_queue: Vec<T>
}

impl<T: Clone> DraftSuitContext<T> {
    /**
    Instantiates a new `DraftSuitContext`

    **Type Parameters:**

    F:  Type representing the types (that implements Fn trait) of items to
        be stored in the queues.

    T:  Type of `Test`s that will be stored in the Tests Queue.
    */
    pub fn new() -> Self {
        Self {
            before_all_queue: Vec::new(),
            after_all_queue: Vec::new(),
            before_each_list: Vec::new(),
            after_each_list: Vec::new(),
            tests_queue: Vec::new(),
        }
    }

    /**
    Seals a `DraftSuitContext`, turning it into a `SuitContext`.
    */
    pub fn seal(self) -> SuitContext<T> {
        return SuitContext {
            before_all_queue: FIFOQueue::from(self.before_all_queue),
            after_all_queue: FIFOQueue::from(self.after_all_queue),
            before_each_list: self.before_each_list,
            after_each_list: self.after_each_list,
            tests_queue: FIFOQueue::from(self.tests_queue),
        }
    }
}