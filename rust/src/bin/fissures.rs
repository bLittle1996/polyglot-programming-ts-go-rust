use std::str::FromStr;

use anyhow::{anyhow, Context};

fn get_input() -> &'static str {
    return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.split_once(",");

        if result.is_none() {
            return Err(anyhow!("expected Point to contain a comma"));
        }

        let (x, y) = result.unwrap();

        let x = str::parse::<i32>(x).context("Unable to parse x value of the provided point");
        let y = str::parse::<i32>(y).context("Unable to parse y value of the provided point");

        return match (x, y) {
            (Ok(x), Ok(y)) => Ok(Point { x, y }),
            (Err(x), _) => Err(x),
            (_, Err(y)) => Err(y),
        };
    }
}
impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.split_once(" -> ");

        if result.is_none() {
            return Err(anyhow!("expected Line to contain a ' -> ' separator"));
        }

        let (left, right) = result.unwrap();

        let p1 = Point::from_str(left);
        let p2 = Point::from_str(right);

        return match (p1, p2) {
            (Ok(p1), Ok(p2)) => Ok(Line { p1, p2 }),
            (Err(p1_err), _) => Err(p1_err),
            (_, Err(p2_err)) => Err(p2_err),
        };
    }
}

impl Line {
    fn is_hori_or_verti(&self) -> bool {
        return self.p1.x == self.p2.x || self.p1.y == self.p2.y;
    }
}

fn main() {
    get_input()
        .lines()
        .map(|line| Line::from_str(line))
        .for_each(|line| match line {
            Ok(line) => {
                if line.is_hori_or_verti() {
                    println!("{:?} is hori or verti", line);
                } else {
                    println!("{:?} is hori or verti", line);
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        });
}
