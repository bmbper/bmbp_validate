use serde_json::Value;

pub trait BmbpValidateRule {
    fn run_rule(&self, field_namme: String, field_desc: String, value: Value) -> (bool, String);
}
