#[test]
fn test_password_hashing() {
    let password = "super-secret";
    let hash = super::hash_password(password).unwrap();
    assert!(super::verify_password(password, &hash).unwrap());
    assert!(!super::verify_password("wrong", &hash).unwrap());
}
