#![allow(non_snake_case)]
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::isize;

fn countNthBit(digit: isize, zeros: isize, ones: isize, values: &[isize]) -> (isize, isize)
{
    if values.is_empty()
    {
        return (zeros, ones);
    }

    let mask = 1 << digit;

    if values[0] & mask == 0
    {
        return countNthBit(digit, zeros + 1, ones, &values[1..]);
    }
    return countNthBit(digit, zeros, ones + 1, &values[1..]);
}

fn CO2Filter(digit: isize, zeros: isize, ones: isize, value: isize) -> bool
{
    let bit = (value & (1 << digit)) >> digit;

    return if ones < zeros
    {
        bit == 1
    }
    else
    {
        bit == 0
    }
}

fn O2Filter(digit: isize, zeros: isize, ones: isize, value: isize) -> bool
{
    let bit = (value & (1 << digit)) >> digit;

    return if zeros > ones
    {
        bit == 0
    }
    else
    {
        bit == 1
    }
}

fn gasRating(digits: isize, values: &[isize], filter: fn(isize, isize, isize, isize) -> bool) -> isize
{
    let mut input : Vec<isize> = values.to_vec();

    for digit in (0..digits).rev()
    {
        let (zeros, ones) = countNthBit(digit, 0, 0, &input);

        input.retain(|val| filter(digit, zeros, ones, *val));

        if input.len() == 1
        {
            return input[0];
        }
    }

    return -1;
}

fn getNthBit(n: isize, values: &[isize]) -> isize
{
    let (zeros, ones) = countNthBit(n, 0, 0, values);

    return if zeros > ones { 0 } else { 1 };
}

fn gamma(digits: isize, values: &[isize]) -> isize
{
    return (0..digits).rev()
        .map(|n| getNthBit(n, values) << n)
        .fold(0, |acc: isize, i: isize| acc | i);
}

fn epsilon(digits: isize, gamma: isize) -> isize
{
    let mask = (1 << digits) - 1;

    return gamma ^ mask;
}


fn getInputLength(f: &mut BufReader<File>) -> isize {
    let mut firstLine: String = String::new();

    f.read_line(&mut firstLine).ok();
    f.seek(SeekFrom::Start(0)).ok();

    return firstLine.trim().len() as isize
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || !Path::new(&args[1]).exists()
    {
        println!("Unknown input file: usage {} <input_file>", args.first().unwrap());
        return;
    }

    let file = File::open(&args[1]).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let digits = getInputLength(&mut reader);
    println!("{}", digits);

    let input: Vec<isize> = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| isize::from_str_radix(&s, 2).unwrap())
        .collect();

    let γ = gamma(digits, &input);
    let ε = epsilon( digits,γ);
    println!("γ {} ε {}, power {}", γ, ε, γ * ε);

    let o2 = gasRating(digits, &input,O2Filter);
    let co2 = gasRating(digits, &input,CO2Filter);

    println!("O_2 {}, CO_2 {} multiplied {}", o2, co2, o2 * co2);
}

