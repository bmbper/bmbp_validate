use serde_json::Value;

pub trait BmbpValidateRule {
    fn run_rule(&self, value: Option<&Value>) -> (bool, String) {
        self.run_field_field(&"".to_string(), &"".to_string(), value)
    }
    fn run_field_field(&self, field_name: &String, field_desc: &String, value: Option<&Value>) -> (bool, String);
}
