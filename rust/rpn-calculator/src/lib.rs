#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut s = vec![];

    for token in inputs {
        match token {
            CalculatorInput::Value(number) => {
                s.push(*number);
            }
            operator => {
                if s.len() < 2 {
                    return None;
                }

                let b = s.pop().unwrap();
                let a = s.pop().unwrap();

                match operator {
                    CalculatorInput::Add => s.push(a + b),
                    CalculatorInput::Subtract => s.push(a - b),
                    CalculatorInput::Multiply => s.push(a * b),
                    CalculatorInput::Divide => s.push(a / b),
                    _ => return None,
                }
            }
        }
    }

    if s.len() != 1 {
        return None;
    }

    s.pop()
}
