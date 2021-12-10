use std::fs;

struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}


fn main() {
    let raw  = fs::read_to_string("input.txt").expect("File not found?");
    let input = raw.lines();

    let mut sub = Submarine {x: 0, y:0, aim: 0};
      

    for line in input {
        let mut iter = line.split_ascii_whitespace();
        let direction = iter.next().expect("No direction?");
        let delta = iter.next().expect("Expected distance?").parse::<i32>().unwrap();

        match direction {
            
            "forward" => {
                sub.x += delta;
                sub.y += sub.aim * delta;
            },
            "up" => sub.aim -= delta,
            "down" => sub.aim += delta,
            _ => println!("Not a valid direction"),
        }

    }
    println!("{}", sub.x * sub.y);
    


    println!("Hello, world!");
}
