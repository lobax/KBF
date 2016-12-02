use std::io;
use std::collections::VecDeque;

macro_rules! scanline { 
    ($x:expr) => { 
        io::stdin().read_line(&mut $x).unwrap(); 
    };
}

fn main() { 
    let mut input = String::new(); 
    scanline!(input); 
    let tokens = tokenize(&input); 
    interpret(tokens);
}


fn interpret(tokens: VecDeque<Token>) { 
    
    let mut mem: [u8;256] = [0; 256];
    let mut stack: Vec<usize> =  Vec::new();
    let mut ip: usize = 0; 
    let mut ptr: u8 =  0;
    while ip < tokens.len() { 
        let token = tokens.get(ip); 
        match token { 
            Some(&Token::IncrementPtr) => { 
                if ptr == 255 { 
                    ptr = 0; 
                } else { 
                    ptr += 1;
                }
            }, 
            Some(&Token::DecrementPtr) => { 
                if ptr == 0 { 
                    ptr = 255
                } else { 
                    ptr -= 1;
                }
            },
            Some(&Token::LeftBracket) => { 
                stack.push(ip); 
            },
            Some(&Token::RightBracket) => { 
                if mem[ptr as usize] > 0 {
                    ip = stack.pop().expect("Unmatching brackets!") -1; 
                }
            },
            Some(&Token::IncrementVal) => { 
                mem[ptr as usize] = mem[ptr as usize] + 1;
            },
            Some(&Token::DecrementVal) => { 
                if mem[ptr as usize] == 0 {
                    mem[ptr as usize] = 255;
                } else { 
                    mem[ptr as usize] = mem[ptr as usize] - 1;
                }
            },
            Some(&Token::InputVal)=> { 
                println!("InputVal");
            },
            Some(&Token::OutputVal) => { 
                print!("{}", mem[ptr as usize] as char);
            },
            None => {},
        }
        ip += 1; 
    }
    println!("");
}

enum Token { 
    IncrementPtr,
    DecrementPtr,
    IncrementVal, 
    DecrementVal, 
    OutputVal, 
    InputVal,
    LeftBracket, 
    RightBracket
}

fn tokenize(input: &String) -> VecDeque<Token> { 
    let mut tkns: VecDeque<Token> = VecDeque::new(); 
    for s in input.chars() { 
        match s { 
            '[' => { 
                tkns.push_back(Token::LeftBracket); 
            },
            ']' => { 
                tkns.push_back(Token::RightBracket); 
            },
            '+' => { 
                tkns.push_back(Token::IncrementVal); 
            },
            '-' => { 
                tkns.push_back(Token::DecrementVal); 
            },
            '>' => {
                tkns.push_back(Token::IncrementPtr); 
            },
            '<' => {
                tkns.push_back(Token::DecrementPtr); 
            },
            '.' => {
                tkns.push_back(Token::OutputVal); 
            },
            ',' => {
                tkns.push_back(Token::InputVal); 
            },
            '\n' => {},
            ' ' => {},
            _ => println!("Error"),
        }
    }
    tkns
}
