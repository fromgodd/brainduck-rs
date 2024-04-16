use std::io::{self, Read};

struct Stack {
    segments: Vec<Vec<u8>>,
    current_segment: usize,
    pointer: usize,
}

impl Stack {
    fn new() -> Self {
        Stack {
            segments: vec![vec![0; 2]],
            current_segment: 0,
            pointer: 0,
        }
    }

    fn current_cell(&mut self) -> &mut u8 {
        &mut self.segments[self.current_segment][self.pointer]
    }

    fn move_right(&mut self) {
        self.pointer += 1;
        if self.pointer >= self.segments[self.current_segment].len() {
            self.segments[self.current_segment].push(0);
        }
    }

    fn move_left(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        }
    }

    fn create_segment(&mut self) {
        self.current_segment += 1;
        self.segments.push(vec![0; 2]);
        self.pointer = 0;
    }

    fn switch_segment(&mut self, direction: isize) {
        let new_segment = (self.current_segment as isize + direction + self.segments.len() as isize) as usize % self.segments.len();
        self.current_segment = new_segment;
    }
}

fn interpret(code: &str) {
    let mut stack = Stack::new();
    let mut loop_stack = Vec::new();
    let mut index = 0;

    while index < code.len() {
        match code.chars().nth(index).unwrap() {
            '>' => stack.move_right(),
            '<' => stack.move_left(),
            '^' => stack.create_segment(),
            'v' => stack.switch_segment(1),
            '^' => stack.switch_segment(-1),
            '+' => *stack.current_cell() = stack.current_cell().wrapping_add(1),
            '-' => *stack.current_cell() = stack.current_cell().wrapping_sub(1),
            '*' => *stack.current_cell() = stack.current_cell().wrapping_mul(stack.segments[stack.current_segment][(stack.pointer + 1) % stack.segments[stack.current_segment].len()]),
            '[' => {
                if *stack.current_cell() == 0 {
                    let mut depth = 1;
                    while depth != 0 {
                        index += 1;
                        match code.chars().nth(index).unwrap() {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => (),
                        }
                    }
                } else {
                    loop_stack.push(index);
                }
            }
            ']' => {
                if *stack.current_cell() != 0 {
                    index = *loop_stack.last().unwrap();
                } else {
                    loop_stack.pop();
                }
            }
            ',' => {
                let mut buffer = [0; 1];
                io::stdin().read_exact(&mut buffer).unwrap();
                *stack.current_cell() = buffer[0];
            }
            '.' => print!("{}", *stack.current_cell() as char),
            _ => (),
        }
        index += 1;
    }
}

fn main() {
    let code = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
    +.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
    ]<+.".to_string();
    interpret(&code);
}
