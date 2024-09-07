mod suit_context;

use std::io::Error;
use std::rc::Rc;
use rustest_core::queue::FIFOQueue;
use crate::suit_context::{DraftSuitContext, SuitContext};

#[derive(Clone)]
struct FakeTest {
    cb: Rc<dyn Fn()>
}

impl FakeTest {
    pub fn run(&self) -> Result<(), Error> {
       Ok(self.cb.clone()())
    }
}

pub fn main() -> () {
    let mut suit_draft: DraftSuitContext<FakeTest> = DraftSuitContext::new();
    suit_draft.tests_queue.push(FakeTest {
        cb: Rc::new(|| {
            println!("Ran call back n° 1!");
        })
    });

    let mut suit = SuitContext::from(suit_draft);

    while let Some(test) = suit.tests_queue.dequeue() {
        let _ = test.run();
    }
}

fn execute_suit(mut suit: SuitContext<FakeTest>) -> Result<(), Error> {
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

        test.run()?;

        // runs every after-each-procedure
        while let Some(aep) = after_each_procedures.dequeue() {
            aep()
        }
    }

    // runs every after-all-procedure
    while let Some(aap) = suit.after_all_queue.dequeue() {
        aap()
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::{execute_suit, FakeTest};
    use crate::suit_context::DraftSuitContext;

    #[test]
    fn engine_flow() {
        let mut suit_draft: DraftSuitContext<FakeTest> = DraftSuitContext::new();

        // register tests
        suit_draft.tests_queue.push(FakeTest {
            cb: Rc::new(|| {
                println!("Ran call back n° 1!");
            })
        });

        suit_draft.tests_queue.push(FakeTest {
            cb: Rc::new(|| {
                println!("Ran call back n° 2!");
            })
        });

        // registers before all and after all
        suit_draft.before_all_queue.push(Rc::new(|| {
            println!("Before all ran!!!");
        }));
        suit_draft.after_all_queue.push(Rc::new(|| {
            println!("After all ran!!!");
        }));

        // registers before and after each
        suit_draft.before_each_list.push(Rc::new(|| {
            println!("Before each 1 just ran.");
        }));
        suit_draft.before_each_list.push(Rc::new(|| {
            println!("Before each 1 just ran.");
        }));
        suit_draft.after_each_list.push(Rc::new(|| {
            println!("After each 1 just ran.");
        }));
        suit_draft.after_each_list.push(Rc::new(|| {
            println!("After each 1 just ran.");
        }));

        let suit = suit_draft.seal();

        let result = execute_suit(suit);
        result.unwrap();
    }
}
