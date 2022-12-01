use std::fs;

#[derive(Debug)]
struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

fn main() {
    let mut submarine_position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");

   for line in contents.lines() {
       let mut splitted = line.split_whitespace();
       let (direction, n) = (
           splitted.next().expect("invalid"),
           splitted.next().expect("invalid")
           .parse::<usize>().expect("invalid"));
       match direction {
           "forward" => {
               submarine_position.horizontal += n;
               submarine_position.depth += submarine_position.aim * n;
           }
           "up" => {
               submarine_position.aim -= n;
           }
           "down" => {
               submarine_position.aim += n;
           }
           _ => { panic!("wtf did you put in your input"); }
       }
   }
   println!("sub is {}",
       submarine_position.horizontal * submarine_position.depth);
}
