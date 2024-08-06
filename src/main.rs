pub struct Mark<'a> {
    line: &'a str,
}

pub trait Parse {
    fn _parse<'a>(&'a self) -> Mark<'a>;
}

impl Parse for &str {
    fn _parse<'a>(&'a self) -> Mark<'a> {
        Mark { line: &self }
    }
}

fn test<'b>(x: &'b str) {
    let m: Mark<'b> = x._parse();
}

fn main() {}
