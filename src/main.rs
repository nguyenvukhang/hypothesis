use std::str::Lines;

pub struct Mark<'a> {
    line: &'a str,
}

pub trait Parse {
    fn whack<'a>(&'a self) -> Option<Mark<'a>>;
    fn identity(&self) -> Self
    where
        Self: Sized;
}

impl Parse for &str {
    fn whack<'a>(&'a self) -> Option<Mark<'a>> {
        Some(Mark { line: &self[0..4] })
    }
    fn identity(&self) -> Self {
        &self[0..1]
    }
}

pub trait ParseLines<'a> {
    fn parse(&mut self) -> Vec<Mark<'a>>;
    fn parse_str(&mut self) -> Vec<&str>;
}

impl<'a> ParseLines<'a> for Lines<'a> {
    fn parse(&mut self) -> Vec<Mark<'a>> {
        let mut lines = vec![];
        for line in self {
            lines.push(Mark { line })
        }
        lines
        // for line in self {
        // let x: Option<Mark<'a>> = line.whack();
        // if let Some(m) = line.whack() {
        //     marks.push(m);
        // }
        // }
        // vec![]
    }

    fn parse_str(&mut self) -> Vec<&str> {
        let mut lines = vec![];
        for line in self {
            lines.push(line.identity());
        }
        lines
    }
}

fn parse() {}

fn main() {
    println!("Hello, world!");
}
