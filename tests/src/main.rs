mod bench;

pub fn main() -> () {
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use rustest_core::suit_context::SuitContext;
    use rustest_core::suit_processor::suit_processor;
    use rustest_core::test::TestContext;

    #[test]
    fn engine_flow() {
        let mut suit: SuitContext = SuitContext::new();
        let outputs: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

        // register tests
        let clone = Rc::clone(&outputs);
        suit.tests_queue.enqueue(TestContext::new(Box::new(move || {
            clone.borrow_mut().push("Ran call back n° 1!".into());
            Ok(())
        })));

        let clone = Rc::clone(&outputs);
        suit.tests_queue.enqueue(TestContext::new(Box::new(move || {
            clone.borrow_mut().push("Ran call back n° 2!".into());
            Ok(())
        })));

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

        let result = suit_processor(suit);

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
        assert_eq!(0, result.len());
    }
}
