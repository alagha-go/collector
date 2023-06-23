use super::*;

#[test]
fn test_des_data() {
    let json = "\"Tv\"";
    let data: Data = serde_json::from_str(json).unwrap();
    assert_eq!(data, Data::Type(Type::Tv))
}