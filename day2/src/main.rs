#![allow(non_snake_case)]
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use itertools::Itertools;

enum Direction
{
    Forward,
    Down,
    Up,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str()
        {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(format!("'{}' is not a valid value for Direction", s)),
        }
    }
}

struct Position
{
    horizontal: isize,
    depth: isize,
    aim: isize,
}

struct Command
{
    direction: Direction,
    movement: isize
}

fn parseCommand(line: String) -> Command
{
    let (command, amount) = line.splitn(2, ' ').collect_tuple().unwrap();

    return Command
    {
        direction: Direction::from_str(command).unwrap(),
        movement: amount.parse::<isize>().unwrap(),
    }
}

fn processCommand(currentPos: Position, cmds: &[Command]) -> Position
{
    if cmds.is_empty()
    {
        return currentPos;
    }

    let cmd = cmds.first().unwrap();

    let newPos : Position = match cmd.direction
    {
        Direction::Up => Position
        {
            horizontal: currentPos.horizontal,
            depth: currentPos.depth - cmd.movement,
            aim: 0
        },
        Direction::Down => Position
        {
            horizontal: currentPos.horizontal,
            depth: currentPos.depth + cmd.movement,
            aim: 0,
        },
        Direction::Forward => Position
        {
            horizontal: currentPos.horizontal + cmd.movement,
            depth: currentPos.depth,
            aim: 0
        },
    };

    return processCommand(newPos, &cmds[1..]);
}

fn processCommandPart2(currentPos: Position, cmds: &[Command]) -> Position
{
    if cmds.is_empty()
    {
        return currentPos;
    }

    let cmd = cmds.first().unwrap();

    let newPos : Position = match cmd.direction
    {
        Direction::Up => Position
        {
            horizontal: currentPos.horizontal,
            depth: currentPos.depth,
            aim: currentPos.aim - cmd.movement,
        },
        Direction::Down => Position
        {
            horizontal: currentPos.horizontal,
            depth: currentPos.depth,
            aim: currentPos.aim + cmd.movement,
        },
        Direction::Forward => Position
        {
            horizontal: currentPos.horizontal + cmd.movement,
            depth: currentPos.depth + currentPos.aim * cmd.movement,
            aim: currentPos.aim,
        },
    };

    return processCommandPart2(newPos, &cmds[1..]);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || !Path::new(&args[1]).exists()
    {
        println!("Unknown input file: usage {} <input_file>", args.first().unwrap());
        return;
    }

    let file = File::open(&args[1]).expect("Unable to open file");
    let f = BufReader::new(file);

    let input: Vec<Command> = f.lines()
        .map(|l| parseCommand(l.unwrap()))
        .collect();

    let newPosition = processCommand(Position { depth: 0, horizontal: 0, aim: 0 }, &input);
    println!("Part1: Horizontal {}, Depth {}, Multiplied {}", newPosition.horizontal, newPosition.depth, newPosition.horizontal * newPosition.depth);

    let newPosition2 = processCommandPart2(Position { depth: 0, horizontal: 0, aim: 0 }, &input);
    println!("Part2: Horizontal {}, Depth {}, Multiplied {}", newPosition2.horizontal, newPosition2.depth, newPosition2.horizontal * newPosition2.depth);
}

