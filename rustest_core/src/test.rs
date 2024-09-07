use crate::test_error::TestError;

pub struct TestContext {
    test: Box<dyn Fn() -> Result<(), TestError>>
}

impl TestContext {
    pub fn new(test: Box<dyn Fn() -> Result<(), TestError>>) -> Self {
        Self {
            test,
        }
    }

    pub fn run(&self) -> Result<(), TestError>{
        (self.test)()?;
        Ok(())
    }
}