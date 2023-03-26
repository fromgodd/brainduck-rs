use std::io;


fn main() {

        const TOTAL_STACK: usize = 30000;
        let mut stack = [0u8; TOTAL_STACK];
        let mut currentIndex = 0;
        let plus = b'+';
        let minus = b'-';
        let next = b'>';
        let back = b'<';
        let prt = b'.';
        let inp = b',';
        let lp = b'[';
        let llp = b']';
        let mut rc = 0;
        let endl = b"\n";
        let ex = b"@"; //exit from program
    stack.iter_mut().for_each(|x| *x = 0);


        println!("brainfuck shell written in Rust");
        let mut command = String::new();
        let mut i = 1;
        loop {
            println!("$ ");
            io::stdin().read_line(&mut command).unwrap();
            for c in command.chars() {
                match c {
                    '+' => stack[currentIndex] += 1,
                    '-' => stack[currentIndex] -= 1,
                    '>' => currentIndex += 1,
                    '<' => currentIndex -= 1,
                    '.' => print!("{}", stack[currentIndex] as char),
                    ',' => {
                        let mut rdln = String::new();
                        io::stdin().read_line(&mut rdln).unwrap();
                        stack[currentIndex] = rdln.as_bytes()[0];
                    }
                    '\n' => i = 1,
                    '@' => i = 0,
                    '[' => {
                        if stack[currentIndex] == 0 {
                            let mut rc = 1;
                            while rc > 0 {
                                let c = command.chars().nth(i).unwrap();
                                if c == '[' { rc += 1; } else if c == ']' { rc -= 1; }
                                i += 1;
                            }
                        }
                    }
                    ']' => {
                        if stack[currentIndex] != 0 {
                            let mut rc = 1;
                            while rc > 0 {
                                i -= 1;
                                let c = command.chars().nth(i).unwrap();
                                if c == '[' { rc -= 1; } else if c == ']' { rc += 1; }
                            }
                            i -= 1;
                        }
                    }
                    _ => {}
                }
                if i == 0 {
                    break
                }
            }
            command.clear();
        }
    }