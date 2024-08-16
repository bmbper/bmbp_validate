use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;

use crate::{BmbpValidateRule};


pub struct BmbpValidator
{
    field: HashMap<String, Vec<Box<dyn BmbpValidateRule>>>,
}


impl BmbpValidator
{
    pub fn new() -> Self {
        BmbpValidator {
            field: HashMap::new(),
        }
    }
    pub fn field_rule(&mut self, field: &str, rule: Vec<Box<dyn BmbpValidateRule>>) -> &mut Self {
        self.field.insert(field.to_string(), rule);
        self
    }
}

impl BmbpValidator
{
    pub fn run_data<'a, D>(&self, data: D) -> (bool, Vec<String>)
        where
            D: Serialize,
    {
        match serde_json::to_value(data) {
            Ok(json_data) => {
                match json_data {
                    Value::Object(map) => {
                        let mut result_msg = vec![];
                        for (name, rules) in self.field.iter() {
                            let field_name_value = map.get(name);
                            for rule in rules {
                                let (result, msg) = rule.run_field_field(name, &"".to_string(), field_name_value);
                                if !result {
                                    result_msg.push(msg);
                                }
                            }
                        }
                        if result_msg.is_empty() {
                            (true, vec![])
                        } else {
                            (false, result_msg)
                        }
                    }
                    _ => {
                        (false, vec!["数据格式错误".to_string()])
                    }
                }
            }
            Err(err) => {
                (false, vec![err.to_string()])
            }
        }
    }
}
