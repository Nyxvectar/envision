use crate::dirman::echo_yan::echo_yan;

/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/13/2025
 */

mod dirman {
    pub mod echo_yan;
}

enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
}

fn main() {
    let score = 85;
    let msg = Message::Move { x:30, y:40 };
    
    print_name();
    
    {
        match score {
            90..=100 => println!("优秀"), // 90 到 100（包含）
            80..=89 => println!("良好"),  // 80 到 89
            60..=79 => println!("及格"),
            0..=59 => println!("不及格"),
            _ => println!("分数无效"), // 处理负数或超过 100 的情况
        }
    }
    
    match msg { 
        Message::Quit => {
            println!("Quit")
        }
        Message::Move { x, y} => {
            println!("Move to {}, {}", x,y)
        }
        Message::Write(text) => {
            println!("Write {}", text)
        }
    }
}

fn print_name() {
    let mut name = "Nyxvectar";
    name = "Nyxvectar Yan";
    println!("{}",echo_yan());
    println!("{}", name);
}
