fn main() {
    soft::assert!(true).unwrap();
    soft::assert!(false).unwrap_err();

    soft::assert_eq!(2, 2).unwrap();
    soft::assert_eq!(2, 3).unwrap_err();

    soft::assert_ne!(2, 3).unwrap();
    soft::assert_ne!(2, 2).unwrap_err();
}
