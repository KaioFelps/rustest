use crate::queue::FIFOQueue;
use crate::suit_context::SuitContext;
use crate::test_error::TestError;

pub fn suit_processor(mut suit: SuitContext) -> Vec<TestError> {
    let mut errors: Vec<TestError> = Vec::new();

    // runs every before-all-procedure
    while let Some(bap) = suit.before_all_queue.dequeue() {
        bap()
    }

    // runs tests
    while let Some(test) = suit.tests_queue.dequeue() {
        let mut before_each_procedures = FIFOQueue::from(suit.before_each_list.clone());
        let mut after_each_procedures = FIFOQueue::from(suit.after_each_list.clone());

        // runs every before-each-procedure
        while let Some(bep) = before_each_procedures.dequeue() {
            bep()
        }

        if let Err(error) = test.run() {
            errors.push(error);
        }

        // runs every after-each-procedure
        while let Some(aep) = after_each_procedures.dequeue() {
            aep()
        }
    }

    // runs every after-all-procedure
    while let Some(aap) = suit.after_all_queue.dequeue() {
        aap()
    }

    errors
}