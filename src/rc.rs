use std::rc::Rc;

pub fn same_ref() {
    let a: Rc<[i32; 3]> = Rc::new([1, 2, 3]);
    let b: Rc<[i32; 3]> = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
}

#[cfg(test)]
mod rc_tests {
    use super::*;

    #[test]
    fn test_same_ref() {
        same_ref();
    }
}
