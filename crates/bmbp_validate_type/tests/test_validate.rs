use std::collections::HashMap;
use serde_json::Value;
use bmbp_validate_type::{BmbpValidateRule, BmbpValidator, MaxLengthRule, MinLengthRule, RequireRule};

#[test]
fn test_validate() {
    let mut user = HashMap::new();
    user.insert("name", "A");

    let mut validator = BmbpValidator::new();
    let mut validate_rule: Vec<Box<dyn BmbpValidateRule>> = vec![];
    let require = RequireRule::new("姓名不能为空".to_string());
    validate_rule.push(Box::new(require));
    let max_length = MaxLengthRule::new(10, "不能超过10".to_string());
    validate_rule.push(Box::new(max_length));
    let min_length = MinLengthRule::new(5, "不能少于5".to_string());
    validate_rule.push(Box::new(min_length));
    validator.field_rule("name", validate_rule);
    let (result, msg) = validator.run_data(user);
    assert_eq!(result, false);
    assert_eq!(msg.len(), 1);
    assert_eq!(msg.get(0).unwrap(), &"姓名不能为空".to_string());
}

#[test]
fn test_require() {
    let v = "".to_string();
    let require = RequireRule::new("不能为空".to_string());
    let (result, msg) = require.run_field_field(&"".to_string(), &"".to_string(), Some(&Value::String(v.to_string())));
    assert_eq!(result, false)
}