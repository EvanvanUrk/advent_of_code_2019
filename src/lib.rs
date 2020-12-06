use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_input(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = File::open(&path).expect("Error opening input file!");

    let mut input = String::new();
    file.read_to_string(&mut input).expect("Error reading input file!");

    input
}

pub mod int_code {
    pub struct IntCodeComputer {
        memory: Vec<i32>,
        pointer: usize,
        running: bool,
    }

    impl IntCodeComputer {
        pub fn new(code: Vec<i32>) -> IntCodeComputer {
            IntCodeComputer {
                memory: code,
                pointer: 0,
                running: false,
            }
        }

        pub fn run(&mut self) {
            self.running = true;
            while self.running {
                self.pointer += self.do_current_op();
            }
        }

        pub fn get(&self, offset: usize) -> i32 {
            self.memory[self.pointer + offset]
        }

        fn do_current_op(&mut self) -> usize {
            match self.memory[self.pointer] {
                1 => self.op_1(
                    self.get(1) as usize,
                    self.get(2) as usize,
                    self.get(3) as usize
                ),
                2 => self.op_2(
                    self.get(1) as usize,
                    self.get(2) as usize,
                    self.get(3) as usize
                ),
                99 => self.op_99(),
                op => panic!("Unknown opcode: {}", op),
            }
        }

        fn op_1(&mut self, a: usize, b: usize, store: usize) -> usize {
            self.memory[store] = self.memory[a] + self.memory[b];
            4
        }

        fn op_2(&mut self, a: usize, b: usize, store: usize) -> usize {
            self.memory[store] = self.memory[a] * self.memory[b];
            4
        }

        fn op_99(&mut self) -> usize {
            self.running = false;
            self.pointer = 0;
            0
        }
    }
}
