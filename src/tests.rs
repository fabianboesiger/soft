use crate::Error;

#[test]
fn assert() {
    assert_eq!(crate::assert!(true), Ok(()));
    assert_eq!(crate::assert!(false), Err(Error::without_message()));
    assert_eq!(
        crate::assert!(false, "Test Message"),
        Err(Error::with_message("Test Message".to_owned()))
    );
}

#[test]
fn assert_eq() {
    assert_eq!(crate::assert_eq!(2, 2), Ok(()));
    assert_eq!(crate::assert_eq!(2, 3), Err(Error::without_message()));
    assert_eq!(
        crate::assert_eq!(2, 3, "Test Message"),
        Err(Error::with_message("Test Message".to_owned()))
    );
}

#[test]
fn assert_ne() {
    assert_eq!(crate::assert_ne!(2, 3), Ok(()));
    assert_eq!(crate::assert_ne!(2, 2), Err(Error::without_message()));
    assert_eq!(
        crate::assert_ne!(2, 2, "Test Message"),
        Err(Error::with_message("Test Message".to_owned()))
    );
}
