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
    type OpFn = (fn(&mut IntCodeComputer, [i32; 3]), usize);

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
                let op = self.get_current_op();
                op.0(self, [
                    self.memory[*self.get(1).unwrap_or(&0) as usize],
                    self.memory[*self.get(2).unwrap_or(&0) as usize],
                    *self.get(3).unwrap_or(&0)
                ]);
                self.pointer += op.1 + 1;
            }
        }

        pub fn get(&self, offset: usize) -> Option<&i32> {
            self.memory.get(self.pointer + offset)
        }

        fn get_current_op(&self) -> OpFn {
            match self.memory[self.pointer] {
                1 => (Self::op_1, 3),
                2 => (Self::op_2, 3),
                99 => (Self::op_99, 0),
                op => panic!("Unknown opcode: {}", op)
            }
        }

        fn op_1(&mut self, args: [i32; 3]) {
            self.memory[args[2] as usize] = args[0] + args[1];
        }

        fn op_2(&mut self, args: [i32; 3]) {
            self.memory[args[2] as usize] = args[0] * args[1];
        }

        fn op_99(&mut self, _args: [i32; 3]) {
            self.running = false;
            self.pointer = 0;
        }
    }
}
