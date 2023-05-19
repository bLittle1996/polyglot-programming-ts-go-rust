struct Coordinate {
    x: i32,
    y: i32,
}

fn main() {
    let final_coord = get_input().lines().map(|line| parse_line(line)).fold(
        Coordinate { x: 0, y: 0 },
        |mut final_coord, coord| {
            final_coord.x += coord.x;
            final_coord.y += coord.y;

            return final_coord;
        },
    );
    let answer = final_coord.x * final_coord.y;

    println!("Coordinate: {}, {}", final_coord.x, final_coord.y);
    println!("Answer: {}", answer);
}

fn parse_line(line: &str) -> Coordinate {
    let (direction, amount) = line.split_once(" ").unwrap_or(("down", "0"));
    let amount: i32 = amount.parse::<i32>().unwrap_or(0);

    return match direction {
        "up" => Coordinate {
            x: 0,
            y: amount * -1,
        },
        "down" => Coordinate { x: 0, y: amount },
        "forward" => Coordinate { x: amount, y: 0 },
        "backward" => Coordinate {
            x: amount * -1,
            y: 0,
        },
        _ => Coordinate { x: 0, y: 0 },
    };
}

fn get_input() -> &'static str {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2";
}
