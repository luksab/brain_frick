#![feature(test)]
pub mod bf_parse {
    use std::{
        cmp::{max, min},
        fmt::Debug,
    };
    use std::{
        fmt::Display,
        io::{self, Read},
    };

    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    use termion::color;

    type Pointer = u16;

    #[derive(FromPrimitive)]
    pub enum OpCode {
        Right = 0,
        Left = 1,
        Plus = 2,
        Minus = 3,
        Put = 4,
        Get = 5,
        LoopStart = 6,
        LoopEnd = 7,

        SwapData = 8,    // Swaps Data under Pointers
        SwapPointer = 9, // Swaps the two Pointers
        JumpForwardByte = 10,
        JumpForwardWord = 11,
        JumpAbsoluteByte = 12,
        JumpAbsoluteWord = 13,
        Return = 14,
    }

    impl Display for OpCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                OpCode::Right => write!(f, ">"),
                OpCode::Left => write!(f, "<"),
                OpCode::Plus => write!(f, "+"),
                OpCode::Minus => write!(f, "-"),
                OpCode::Put => write!(f, "."),
                OpCode::Get => write!(f, ","),
                OpCode::LoopStart => write!(f, "["),
                OpCode::LoopEnd => write!(f, "]"),
                OpCode::SwapData => write!(f, "$"),
                OpCode::SwapPointer => write!(f, "!"),
                OpCode::JumpForwardByte => write!(f, ">"),
                OpCode::JumpForwardWord => write!(f, ">"),
                OpCode::JumpAbsoluteByte => write!(f, ">"),
                OpCode::JumpAbsoluteWord => write!(f, ">"),
                OpCode::Return => write!(f, "?"),
            }
        }
    }

    pub struct BfInterpret {
        data: [u8; 30_000],
        ip: Pointer,
        jump_back: Vec<Pointer>,
        dp: Pointer,
    }

    impl Debug for BfInterpret {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "ip: {}, dp: {}, jb: {:?} \n{}",
                self.ip, self.dp, self.jump_back, self
            )
        }
    }

    impl Display for BfInterpret {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let size = 20;
            let mut ret = String::new();
            ret += &format!("{:0>5}", self.ip);
            for i in max(self.ip, size) - size..min(self.ip + size, 30_000) {
                match OpCode::from_u8(self.data[i as usize] & 15) {
                    Some(inst) => {
                        if i == self.ip {
                            ret += &format!(
                                "{}{}{}",
                                color::Fg(color::Red),
                                &inst,
                                color::Fg(color::Reset)
                            );
                        } else {
                            ret += &format!("{}", &inst);
                        }
                    }
                    None => {
                        ret += "";
                    }
                }
            }
            //ret += &format!("\n{:0>6}", self.dp);
            ret += &format!("\n");
            for i in max(self.dp, size) - size..min(self.dp + size, 30_000) {
                if i == self.dp {
                    ret += &format!("{}", color::Fg(color::Blue));
                }
                if false {
                    match <OpCode as FromPrimitive>::from_u8(self.data[i as usize] & 15) {
                        Some(inst) => {
                            ret += &format!("{}", &inst);
                        }
                        None => {
                            ret += &format!("{}", self.data[i as usize]);
                        }
                    }
                } else {
                    ret += &format!("{:0>3},", self.data[i as usize]);
                }
                if i == self.dp {
                    ret += &format!("{}", color::Fg(color::Reset));
                }
            }
            write!(f, "{}", ret)
        }
    }

    pub enum Operation {
        Right(),
        Left(),
        Plus(),
        Minus(),
        Put(u8),
        Get(),
        LoopStart(),
        LoopEnd(),

        SwapData(),    // Swaps Data under Pointers
        SwapPointer(), // Swaps the two Pointers
        JumpForwardByte(),
        JumpForwardWord(),
        JumpAbsoluteByte(),
        JumpAbsoluteWord(),
        Return(),
    }

    impl Display for Operation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Operation::Right() => write!(f, ">"),
                Operation::Left() => write!(f, "<"),
                Operation::Plus() => write!(f, "+"),
                Operation::Minus() => write!(f, "-"),
                Operation::Put(char) => write!(f, "{}", *char as char),
                Operation::Get() => write!(f, ","),
                Operation::LoopStart() => write!(f, "["),
                Operation::LoopEnd() => write!(f, "]"),
                Operation::SwapData() => write!(f, "!"),
                Operation::SwapPointer() => write!(f, "$"),
                Operation::JumpForwardByte() => write!(f, ">"),
                Operation::JumpForwardWord() => write!(f, ">"),
                Operation::JumpAbsoluteByte() => write!(f, ">"),
                Operation::JumpAbsoluteWord() => write!(f, ">"),
                Operation::Return() => write!(f, "?"),
            }
        }
    }

    impl BfInterpret {
        pub fn new(code: String) -> Result<BfInterpret, ()> {
            let mut data = [0; 30_000];
            for (i, char) in code.chars().enumerate() {
                data[i] = match char {
                    '>' => OpCode::Right,
                    '<' => OpCode::Left,
                    '+' => OpCode::Plus,
                    '-' => OpCode::Minus,
                    '.' => OpCode::Put,
                    ',' => OpCode::Get,
                    '[' => OpCode::LoopStart,
                    ']' => OpCode::LoopEnd,
                    '?' => OpCode::Return,

                    _ => return Err(()),
                } as u8;
            }
            //println!("code len: {}", code.len());
            Ok(BfInterpret {
                data,
                ip: 0,
                jump_back: Vec::new(),
                dp: (code.len()) as Pointer,
            })
        }

        pub fn runAll(&mut self) -> Operation {
            Operation::Return()
        }

        pub fn step(&mut self) -> Operation {
            match FromPrimitive::from_u8(self.data[self.ip as usize] & 15)
                .expect("unknown Instruction")
            {
                OpCode::Right => {
                    self.dp += 1;
                    self.ip += 1;
                    Operation::Right()
                }
                OpCode::Left => {
                    self.dp -= 1;
                    self.ip += 1;
                    Operation::Left()
                }
                OpCode::Plus => {
                    self.data[self.dp as usize] = self.data[self.dp as usize].wrapping_add(1);
                    self.ip += 1;
                    Operation::Plus()
                }
                OpCode::Minus => {
                    self.data[self.dp as usize] = self.data[self.dp as usize].wrapping_sub(1);
                    self.ip += 1;
                    Operation::Minus()
                }
                OpCode::Put => {
                    let data = self.data[self.dp as usize];
                    //print!("{}", data as char);
                    //print!("{} ", self.data[self.dp as usize]);
                    self.ip += 1;
                    Operation::Put(data)
                }
                OpCode::Get => {
                    let mut buffer = [0];
                    io::stdin()
                        .lock()
                        .read(&mut buffer)
                        .expect("non-ascii character on stdin");
                    self.data[self.dp as usize] = buffer[0];
                    self.ip += 1;
                    Operation::Get()
                }
                OpCode::LoopStart => {
                    if self.data[self.dp as usize] == 0 {
                        let mut brack_count = 1;
                        while brack_count > 0 {
                            self.ip += 1;
                            match FromPrimitive::from_u8(self.data[self.ip as usize] & 15)
                                .expect("unknown Instruction")
                            {
                                OpCode::LoopStart => brack_count += 1,
                                OpCode::LoopEnd => brack_count -= 1,
                                _ => {}
                            }
                        }
                        self.ip += 1;
                    } else {
                        self.jump_back.push(self.ip);
                        self.ip += 1;
                    }
                    Operation::LoopStart()
                }
                OpCode::LoopEnd => {
                    let address = self.jump_back.last();
                    let address = match address {
                        Some(a) => a,
                        None => {
                            println!("\n{:?}", self);
                            panic!("jump back empty");
                        }
                    };

                    //println!("data: {}", self.data[self.dp as usize]);
                    if self.data[self.dp as usize] != 0 {
                        self.ip = *address + 1;
                    } else {
                        self.jump_back.pop();
                        self.ip += 1;
                    }
                    Operation::LoopEnd()
                }
                OpCode::SwapData => {
                    let data = self.data[self.dp as usize];
                    self.data[self.dp as usize] = self.data[self.ip as usize];
                    self.data[self.ip as usize] = data;
                    Operation::SwapData()
                }
                OpCode::SwapPointer => {
                    let dp = self.dp;
                    self.dp = self.ip;
                    self.ip = dp;
                    Operation::SwapPointer()
                }
                OpCode::JumpForwardByte => Operation::JumpForwardByte(),
                OpCode::JumpForwardWord => Operation::JumpForwardWord(),
                OpCode::JumpAbsoluteByte => Operation::JumpAbsoluteByte(),
                OpCode::JumpAbsoluteWord => Operation::JumpAbsoluteWord(),
                OpCode::Return => {
                    return Operation::Return();
                }
            }
        }
    }

    impl Iterator for BfInterpret {
        type Item = Operation;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            let step = self.step();
            match step {
                Operation::Return() => None,
                step => Some(step),
            }
        }
    }
}

pub use bf_parse::BfInterpret;


mod tests;