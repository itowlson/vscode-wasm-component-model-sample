wit_bindgen::generate!({
    path: "../wit",
	world: "calculator",
});

use std::cell::RefCell;
use exports::vscode::example::types::Operation;

struct CalcEngine {
    stack: RefCell<Vec<u32>>,
}

impl exports::vscode::example::types::GuestEngine for CalcEngine {
    fn new() -> Self {
        CalcEngine {
            stack: RefCell::new(vec![])
        }
    }

    fn push_operand(&self, operand: u32) {
        self.stack.borrow_mut().push(operand);
    }

    fn push_operation(&self, operation:Operation) {
        let mut stack = self.stack.borrow_mut();
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();
        let result = match operation {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        };
        stack.push(result);
    }

    fn execute(&self) -> u32 {
        return self.stack.borrow_mut().pop().unwrap();
    }
}

struct Implementation;
impl exports::vscode::example::types::Guest for Implementation {
    type Engine = CalcEngine;
}

export!(Implementation);
