#![allow(non_snake_case)]
use std::fs::File;
use std::io::{BufRead, BufReader};

fn countIncreases(count: usize, tail: &[isize]) -> usize
{
    let (head, tail) = tail.split_at(1);

    if head.is_empty() || tail.is_empty()
    {
        return count;
    }

    let newCount = if head[0] < tail[0] { count + 1 } else { count };

    return countIncreases(newCount, tail );
}

fn sumAdjacentSamples(input: &[isize]) -> Vec<isize>
{
    if input.len() < 3
    {
        return Vec::new();
    }

    let head = &input[0..3];
    let sum: isize = head.iter().sum();

    let mut sums : Vec<isize> = vec!(sum);
    sums.extend(sumAdjacentSamples(&input[1..]));

    return sums;
}

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2
    {
        println!("Unknown input file: usage {} <input_file>", args.first().unwrap());
    }

    let file = File::open(&args[1]).expect("Unable to open file");
    let f = BufReader::new(file);

    let input: Vec<isize> = f.lines()
        .map(|l| l.unwrap().parse::<isize>().unwrap())
        .collect();

    println!("{}", countIncreases(0, &input));

    let sums : Vec<isize> = sumAdjacentSamples(&input);
    println!("{}", countIncreases(0, &sums));
}
