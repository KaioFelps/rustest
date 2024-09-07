pub trait UnshiftTrait<T> {
    /**
    Returns the first element of the vector, or None if it is empty.
    The element will be removed from the slice.

    # Examples
    ```
    // v must be mutable, as this mutates the vector
    let mut v = vec![10, 40, 30];
    assert_eq!(Some(10), v.unshift());
    assert_eq!(vec![40, 30], v);

    let w: Vec<i32> = vec![];
    assert_eq!(None, w.unshift());
    ```
     */
    fn unshift(&mut self) -> Option<T>;
}

impl<T: Clone> UnshiftTrait<T> for Vec<T> {
    fn unshift(&mut self) -> Option<T> {
        let first: Option<T> = match self.first() {
            Some(value) => Some(value.clone().into()),
            None => None
        };

        if first.is_some() {
            self.remove(0);
        }

        return first;
    }
}