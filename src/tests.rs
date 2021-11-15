#[test]
fn panic() {
    assert!(crate::panic!().is_err());
    assert!(crate::panic!("Test Message").is_err());
}

#[test]
fn assert() {
    assert!(crate::assert!(true).is_ok());
    assert!(crate::assert!(false).is_err());
    assert!(crate::assert!(false, "Test Message").is_err());
}

#[test]
fn assert_eq() {
    assert!(crate::assert_eq!(2, 2).is_ok());
    assert!(crate::assert_eq!(2, 3).is_err());
    assert!(crate::assert_eq!(2, 3, "Test Message").is_err());
}

#[test]
fn assert_ne() {
    assert!(crate::assert_ne!(2, 3).is_ok());
    assert!(crate::assert_ne!(2, 2).is_err());
    assert!(crate::assert_ne!(2, 2, "Test Message").is_err());
}
