use std::rc::Rc;
use crate::queues::FIFOQueue;
use crate::test::TestContext;

/// Type of procedures to go inside the Lists of the SuitContext struct
// Will be cloned lots of times (and thus, called lots of times),
// so it must be a Reference Counter in order of it not going out of scope before being fully used
// nor prevented to be called more than once.
type ListProcedure = Rc<dyn Fn()>;

/// Type of procedures to go inside the Queues of the SuitContext struct
// Will be freed as used, so no need to reference-count it, opposed to the ListProcedure.
// Also, it can be FnOnce (opposed to ListProcedure) as it will be called only once
// and then dequeued.
type QueueProcedure = Box<dyn FnOnce()>;

/// # Suit Context
/// A Suit Context represents a situation being tested.
///
/// A SuitContext instance is composed by the following elements:
/// - **Before All Queue**
///
///     A FIFO queue holding procedures (functions) that must run
///     once at the start of the suit execution (before every test unit).
///     Every item of this list will me **moved** to the Execution Stack
///     before the first, or better, the first *before list *procedure
///     even comes to be executed.
///
/// - **After All Queue**
///
///     Just like the *Before All Queue*, but will run at the very end
///     of the suit execution.
///
/// - **Before Each List**
///
///     A read-only list of procedures to run before each suit's tests.
///     Every item of this list will be **copied** to the Execution Stack
///     and ran before each test's execution at the Execution Stack.
///
/// - **After Each List**
///
///     Just like the *Before Each List*, but holds procedures (functions)
///     to be executed after each test ends running.
///
/// - **Testes Queue**
///
///     Finally, this (FIFO) queue holds every test (of type [`rustest_core::test::TestContext`])
///     that must be run during this Suit Context execution.
///     Each item of the queue will be dequeued (moved) to the Execution Stack.
///
/// [rustest_core::test::Test]: crate::test::TestContext
pub struct SuitContext {
    pub before_all_queue: FIFOQueue<QueueProcedure>,
    pub after_all_queue: FIFOQueue<QueueProcedure>,
    pub before_each_list: Vec<ListProcedure>,
    pub after_each_list: Vec<ListProcedure>,
    pub tests_queue: FIFOQueue<TestContext>
}

impl SuitContext {
    pub fn new() -> Self {
        SuitContext {
            before_all_queue: FIFOQueue::new(),
            after_all_queue: FIFOQueue::new(),
            before_each_list: Vec::new(),
            after_each_list: Vec::new(),
            tests_queue: FIFOQueue::new()
        }
    }
}

impl From<Vec<TestContext>> for SuitContext {
    /**
    Instantiates a SuitContext with the given `value` as it's initial `tests_queue`.
    */
    fn from(value: Vec<TestContext>) -> Self {
        let mut suit = SuitContext::new();
        suit.tests_queue = FIFOQueue::from(value);
        suit
    }
}
