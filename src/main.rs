mod output_datastructures;

use crate::output_datastructures::ControlWord;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::is_alphanumeric;
use nom::IResult;

fn indentifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_')(input)
}

fn label(input: &str) -> IResult<&str, &str> {
    let (remaining, identifier) = identifier(input)?;
    let (remaining, _) = tag(":")?;
    let (remaining, _) = take_while(is_space)?;
}

fn main() {
    let a = tag("a");
    let b = tag("b");
    let c = tag("c");
    let d = tag("d");
    let out = tag("out");
    let bs = tag("bs");

    let gpr = alt((a, b, c, d));
    let params = alt((gpr, out, bs));

    let mov = tag("mov");
    let sub = tag("sub");
    let add = tag("add");
    let and = tag("and");
    let or = tag("or");
    let xor = tag("xor");
    let shl = tag("shl");
    let shr = tag("shr");
    let cmp = tag("cmp");
    let jmp = tag("jmp");
    let hlt = tag("hlt");
    let jc = tag("jc");
    let jz = tag("jz");

    let test = ControlWord::empty();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn label_parses_stuff() {
        let input = "test: ";
        assert_eq!(label(input), Ok((" ", "test")));
    }
}
