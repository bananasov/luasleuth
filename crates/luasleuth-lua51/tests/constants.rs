use luasleuth_lua51::types::constants::Constant;

#[test]
fn test_is_constant_tag_valid() {
    assert_eq!(Constant::Nil.get_type(), 0);
    assert_eq!(Constant::Boolean(false).get_type(), 1);
    assert_eq!(Constant::Number(3.14).get_type(), 3);
    assert_eq!(Constant::String("hello".into()).get_type(), 4);
}
