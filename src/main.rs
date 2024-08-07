pub struct Mark<'a> {
    line: &'a str,
}

pub trait Parse<'a> {
    fn _parse(self) -> Mark<'a>;
}

impl<'a> Parse<'a> for &'a str {
    fn _parse(self) -> Mark<'a> {
        Mark { line: &self }
    }
}

fn test<'b>(x: &'b str) {
    let m: Mark<'b> = Mark { line: &x };
    let m: Mark<'b> = Mark { line: x };
    let m: Mark<'b> = x._parse(); // compile error here
}

fn main() {}
