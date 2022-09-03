use std::fmt::Display;

use num_bigint::BigUint;

#[derive(Debug, Default)]
pub struct ValueHandler {
    operation_stack: Vec<Operation>,
    current_operation: Operation,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(ValueWrap),
}

impl Default for Operation {
    fn default() -> Self {
        Operation::Value(ValueWrap::default())
    }
}

impl ValueHandler {
    pub fn main_display(&self) -> String {
        // TODO cache or store last operation that is Value
        if let Operation::Value(value) = &self.current_operation {
            value.to_string()
        } else {
            let last_value_op = self.operation_stack.iter().rev().find_map(|op| {
                if let Operation::Value(value) = op {
                    Some(value.to_string())
                } else {
                    None
                }
            });

            last_value_op.unwrap_or_else(|| "0".to_string())
        }
    }

    pub fn confirm_operation(&mut self) {
        self.operation_stack.push(self.current_operation.clone());
        self.current_operation = Operation::default();
    }

    pub fn add(&mut self) {
        if let Operation::Value(_) = &self.current_operation {
            self.confirm_operation();
        }

        self.current_operation = Operation::Add;
    }

    pub fn subtract(&mut self) {
        if let Operation::Value(_) = &self.current_operation {
            self.confirm_operation();
        }

        self.current_operation = Operation::Subtract;
    }

    pub fn multiply(&mut self) {
        if let Operation::Value(_) = &self.current_operation {
            self.confirm_operation();
        }

        self.current_operation = Operation::Multiply;
    }

    pub fn divide(&mut self) {
        if let Operation::Value(_) = &self.current_operation {
            self.confirm_operation();
        }

        self.current_operation = Operation::Divide;
    }

    pub fn append_digit(&mut self, digit: u8) {
        if let Operation::Value(value) = &mut self.current_operation {
            value.append_digit(digit);
        } else {
            self.confirm_operation();

            let mut value = ValueWrap::new();
            value.set_value(digit.into(), false, None);
            self.current_operation = Operation::Value(value);
        }
    }

    pub fn clear_all(&mut self) {
        *self = ValueHandler::default();
    }
}

#[derive(Debug, Clone)]
pub struct ValueWrap {
    main_value: BigUint,
    sub_value: Option<Vec<u8>>,
    is_negative: bool,
}

impl Default for ValueWrap {
    fn default() -> Self {
        Self {
            main_value: BigUint::from(0u8),
            sub_value: None,
            is_negative: false,
        }
    }
}

impl ValueWrap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_value(&mut self, value: BigUint, is_negative: bool, sub_value: Option<Vec<u8>>) {
        self.main_value = value;
        self.is_negative = is_negative;
        self.sub_value = sub_value;
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn append_digit(&mut self, digit: u8) {
        match &mut self.sub_value {
            Some(value) => value.push(digit),
            None => {
                self.main_value *= BigUint::from(10u8);
                self.main_value += BigUint::from(digit);
            }
        }
    }

    pub fn append_to_sub_value(&mut self) {
        self.sub_value = Some(Vec::new());
    }

    pub fn set_negative(&mut self) {
        self.is_negative = true;
    }
}

impl Display for ValueWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value = String::new();

        if self.is_negative {
            value.push('-');
        }
        value.push_str(&self.main_value.to_string());
        if let Some(sub_value) = &self.sub_value {
            value.push_str(&format!(
                ".{}",
                sub_value.iter().map(|v| v.to_string()).collect::<String>()
            ));
        }

        write!(f, "{}", value)
    }
}
