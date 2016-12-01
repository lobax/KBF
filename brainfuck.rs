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
    parse(&tokens);
}


fn parse(tokens: &VecDeque<Token>) { 
    let mut mem: [u8; 256] = [0; 256]; 
    let mut ptr: u8 = 0; 
    for token in tokens.iter() { 
        match token { 
            &Token::IncrementPtr => { 
                ptr += 1;
            }, 
            &Token::DecrementPtr => { 
                ptr -= 1;
            },
            &Token::LeftBracket => { 
                println!("LeftBracket");
            },
            &Token::RightBracket => { 
                println!("RightBracket");
            },
            &Token::IncrementVal => { 
                mem[ptr as usize] = mem[ptr as usize] + 1;
            },
            &Token::DecrementVal => { 
                mem[ptr as usize] = mem[ptr as usize] - 1;
            },
            &Token::InputVal => { 
                println!("InputVal");
            },
            &Token::OutputVal => { 
                println!("{}", mem[ptr as usize]);
            },
        }
    }

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
            '\n' => println!("NewLine"),
            _ => println!("Error"),
        }
    }
    println!("END"); 
    tkns
}
