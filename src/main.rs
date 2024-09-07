mod suit_context;

use std::io::Error;
use rustest_core::queue::FIFOQueue;
use crate::suit_context::SuitContext;

struct FakeTest {
    cb: Box<dyn Fn()>
}

impl FakeTest {
    pub fn run(&self) -> Result<(), Error> {
       Ok((self.cb)())
    }
}

pub fn main() -> () {
    // main
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
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::{execute_suit, FakeTest};
    use crate::suit_context::SuitContext;

    #[test]
    fn engine_flow() {
        let mut suit: SuitContext<FakeTest> = SuitContext::new();
        let outputs: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

        // register tests
        let clone = Rc::clone(&outputs);
        suit.tests_queue.enqueue(FakeTest {
            cb: Box::new(move || {
                clone.borrow_mut().push("Ran call back n° 1!".into());
            })
        });

        let clone = Rc::clone(&outputs);
        suit.tests_queue.enqueue(FakeTest {
            cb: Box::new(move || {
                clone.borrow_mut().push("Ran call back n° 2!".into());
            })
        });

        // registers before all and after all
        let clone = Rc::clone(&outputs);
        suit.before_all_queue.enqueue(Box::new(move || {
            clone.borrow_mut().push("Before all ran!!!".into());
        }));

        let clone = Rc::clone(&outputs);
        suit.after_all_queue.enqueue(Box::new(move || {
            clone.borrow_mut().push("After all ran!!!".into());
        }));

        // registers before and after each

        let clone = Rc::clone(&outputs);
        suit.before_each_list.push(Rc::new(move || {
            clone.borrow_mut().push("Before each 1 just ran.".into());
        }));

        let clone = Rc::clone(&outputs);
        suit.before_each_list.push(Rc::new(move || {
            clone.borrow_mut().push("Before each 1 just ran.".into());
        }));

        let clone = Rc::clone(&outputs);
        suit.after_each_list.push(Rc::new(move || {
            clone.borrow_mut().push("After each 1 just ran.".into());
        }));

        let clone = Rc::clone(&outputs);
        suit.after_each_list.push(Rc::new(move || {
            clone.borrow_mut().push("After each 1 just ran.".into());
        }));

        let result = execute_suit(suit);
        result.unwrap();

        let expected_output: Vec<String> = vec![
            "Before all ran!!!".into(),
            "Before each 1 just ran.".into(),
            "Before each 1 just ran.".into(),
            "Ran call back n° 1!".into(),
            "After each 1 just ran.".into(),
            "After each 1 just ran.".into(),
            "Before each 1 just ran.".into(),
            "Before each 1 just ran.".into(),
            "Ran call back n° 2!".into(),
            "After each 1 just ran.".into(),
            "After each 1 just ran.".into(),
            "After all ran!!!".into(),
        ];

        assert_eq!(expected_output, outputs.take());
    }
}
