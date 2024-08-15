use chrono::{NaiveDate, NaiveTime};

use crate::BmbpValidateRule;

// require
pub struct RequireRule {
    msg: String,
}
impl RequireRule {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl BmbpValidateRule for RequireRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}

// max length
pub struct MaxLengthRule {
    max_length: u32,
    msg: String,
}
impl BmbpValidateRule for MaxLengthRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}

// min length
pub struct MinLengthRule {
    min_length: u32,
    msg: String,
}
impl BmbpValidateRule for MinLengthRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// minValue
pub struct MinValueRule {
    min_value: u32,
    msg: String,
}
impl BmbpValidateRule for MinValueRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// maxValue
pub struct MaxValueRule {
    min_value: u32,
    msg: String,
}
impl BmbpValidateRule for MaxValueRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// number range
pub struct NumberRangeRule {
    min_value: u32,
    max_value: u32,
    msg: String,
}
impl BmbpValidateRule for NumberRangeRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// time range
pub struct TimeRangeRule {
    min_value: NaiveTime,
    max_value: NaiveTime,
    msg: String,
}
impl BmbpValidateRule for TimeRangeRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// date range
pub struct DateRangeRule {
    min_value: NaiveDate,
    max_value: NaiveDate,
    msg: String,
}
impl BmbpValidateRule for DateRangeRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}

// enum in set
pub struct EnumValueRule<T>
where
    T: ToString,
{
    enum_value: Vec<T>,
    msg: String,
}
impl<T> BmbpValidateRule for EnumValueRule<T>
where
    T: ToString,
{
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// regex
pub struct RegexRule {
    regex: String,
    msg: String,
}
impl BmbpValidateRule for RegexRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}

// 常用校验规则
// email
pub struct EmailRule {
    msg: String,
}
impl BmbpValidateRule for EmailRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// phone
pub struct TelphoneRule {
    msg: String,
}
impl BmbpValidateRule for TelphoneRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// region
pub struct RegionRule {
    msg: String,
}
impl BmbpValidateRule for RegionRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// id card no
pub struct IDNumRule {
    msg: String,
}
impl BmbpValidateRule for IDNumRule {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// IPV4
pub struct IPV4 {
    msg: String,
}
impl BmbpValidateRule for IPV4 {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// IPV6
pub struct IPV6 {
    msg: String,
}
impl BmbpValidateRule for IPV6 {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
// MAC
pub struct MAC {
    msg: String,
}
impl BmbpValidateRule for MAC {
    fn run_rule(
        &self,
        field_namme: String,
        field_desc: String,
        value: serde_json::Value,
    ) -> (bool, String) {
        (true, "".to_string())
    }
}
