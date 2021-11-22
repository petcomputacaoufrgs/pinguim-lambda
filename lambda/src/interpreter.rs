use crate::value::Value;

/// Reduz o termo até a sua forma normal, se existir. Se não existir, entra em loop infinito.
pub fn run_once(input: Value) -> Value {
    let mut interpreter = Interpreter::new(input);
    interpreter.run_all();
    interpreter.finish()
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    steps: u64,
    input: Value,
    current: Value,
}

impl Interpreter {
    pub fn new(input: Value) -> Self {
        Self { current: input.clone(), input, steps: 0 }
    }

    pub fn reset(&mut self) {
        self.steps = 0;
        self.current = self.input.clone();
    }

    pub fn run_step(&mut self) -> bool {
        if self.current.reduce_one() {
            self.steps += 1;
            true
        } else {
            false
        }
    }

    pub fn run_steps(&mut self, max_steps: u32) -> bool {
        for _ in 0..max_steps {
            if !self.run_step() {
                return false;
            }
        }
        true
    }

    pub fn run_all(&mut self) {
        while self.run_step() {}
    }

    pub fn steps(&self) -> u64 {
        self.steps
    }

    pub fn input(&self) -> &Value {
        &self.input
    }

    pub fn output(&self) -> &Value {
        &self.current
    }

    pub fn finish(self) -> Value {
        self.current
    }
}
