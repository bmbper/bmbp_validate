use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::BmbpValidateRule;

pub struct BmbpValidator<T>
where
    T: BmbpValidateRule,
{
    rule: Vec<Box<T>>,
    field: HashMap<String, String>,
}
impl<T> BmbpValidator<T>
where
    T: BmbpValidateRule,
{
    pub fn new() -> Self {
        BmbpValidator {
            rule: vec![],
            field: HashMap::new(),
        }
    }
    pub fn with_rule(rule: Vec<Box<T>>) -> Self {
        BmbpValidator {
            rule,
            field: HashMap::new(),
        }
    }
}

impl<T> BmbpValidator<T>
where
    T: BmbpValidateRule,
{
    pub fn run_data<'a, D>(&self, data: D) -> (bool, Vec<String>)
    where
        D: Serialize,
    {
        (true, vec![])
    }
}
