use std::str::Lines;

pub struct Mark<'a> {
    line: &'a str,
}

pub trait Parse {
    fn whack(&self) -> Option<Mark>;
}

impl Parse for &str {
    fn whack(&self) -> Option<Mark> {
        Some(Mark { line: &self[0..4] })
    }
}

pub trait ParseLines<'a> {
    fn parse(&mut self) -> Vec<Mark>;
}

impl<'a> ParseLines<'a> for Lines<'a> {
    fn parse(&mut self) -> Vec<Mark> {
        let mut marks = vec![];
        for (lnum, line) in self.enumerate() {
            if let Some(m) = line.whack() {
                marks.push(m);
            }
        }
        marks
    }
}

fn parse() {}

fn main() {
    println!("Hello, world!");
}
