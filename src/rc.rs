use std::{rc::Rc, sync::Arc, thread};

pub fn same_ref() {
    let a: Rc<[i32; 3]> = Rc::new([1, 2, 3]);
    let b: Rc<[i32; 3]> = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
}

pub fn same_ref_not_threadsafe() {
    let a: Rc<[i32; 3]> = Rc::new([1, 2, 3]);
    let _b: Rc<[i32; 3]> = a.clone();

    // won't compile, not threadsafe
    // thread::spawn(move || dbg!(a));
    // thread::spawn(move || dbg!(_b));
}

pub fn same_ref_threadsafe() {
    let a: Arc<[i32; 3]> = Arc::new([1, 2, 3]);
    let b: Arc<[i32; 3]> = a.clone();

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}

#[cfg(test)]
mod rc_tests {
    use super::*;

    #[test]
    fn test_same_ref() {
        same_ref();
    }

    #[test]
    fn test_same_ref_threadsafe() {
        same_ref_threadsafe();
    }
}
