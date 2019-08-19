mod output_datastructures;

use crate::output_datastructures::ControlWord;
use nom::branch::alt;
use nom::bytes::complete:: {
    tag,
    take_while_m_n,
    take_while1,
};
use nom::character::complete::digit1;
use nom::sequence::preceded;
use nom::character::complete::one_of;
use nom::character::is_digit;
use nom::character::is_space;
use nom::IResult;
use nom::error::ErrorKind;
use nom::Err;

fn identifier(input: &str) -> IResult<&str, &str> {
    one_of("_abcdefghijklmnopqrstuvwxyz")(input)?;
   take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_')(input)
}

fn label(input: &str) -> IResult<&str, &str> {
    let (remaining, identifier) = identifier(input)?;
    let (remaining, _) = tag(":")(remaining)?;
    Ok((remaining, identifier))
}

fn hlt(input: &str) -> IResult<&str, &str> {
    tag("hlt")(input)
}

fn memory_location(input: &str) -> IResult<&str, u8> {
    let (remaining, number) = digit1(input)?;
    let number: u8 = match number.parse() {
        Ok(i) => i,
        _ => return Err(Err::Error((input, ErrorKind::Digit))),
    };
    Ok((remaining, number))
}
fn main() {
    /*let a = tag("a");
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
    let cmp = tag("cmp");

    let shl = tag("shl");
    let shr = tag("shr");

    let jmp = tag("jmp");
    let jc = tag("jc");
    let jz = tag("jz");

    let hlt = tag("hlt");
    */

    let test = ControlWord::empty();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err;


    #[test]
    fn memory_location_matches_8bit_number() {
        let input = "123";
        assert_eq!(memory_location(input), Ok(("", 123)));
        let input = "sda";
        assert_eq!(memory_location(input), Err(Err::Error((input, ErrorKind::Digit))));
        let input = "256";
        assert_eq!(memory_location(input), Err(Err::Error((input, ErrorKind::Digit))));
        let input = "12lakfsdj";
        assert_eq!(memory_location(input), Err(Err::Error((input, ErrorKind::Digit))));
        let input = "12 lakfsdj";
        assert_eq!(memory_location(input), Ok(("lakfsdj", 12)));
    }

    #[test]
    fn hlt_matches_hlt() {
        let input = "hlt";
        assert_eq!(hlt(input), Ok(("", input)));
        let input = "sda";
        assert_eq!(hlt(input), Err(Err::Error((input, ErrorKind::Tag))));
        let input = "hlt asdf";
        assert_eq!(hlt(input), Ok((" asdf", "hlt")));
    }

    #[test]
    fn identifier_allows_alphabetic() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(identifier(input), Ok(("", input)));
    }

    #[test]
    fn identifier_allows__() {
        let input = "_ ";
        assert_eq!(identifier(input), Ok((" ", "_")));
    }

    #[test]
    fn identifier_allows_numeric_after_one_other() {
        let input = "_0123456789 ";
        assert_eq!(identifier(input), Ok((" ", "_0123456789")));
    }

    #[test]
    fn identifier_denies_numeric_beginning() {
        let input = "012345asdfdf6789 ";
        assert_eq!(identifier(input), Err(Err::Error((input, ErrorKind::OneOf))));
    }

    #[test]
    fn label_parses_stuff() {
        let input = "test: ";
        assert_eq!(label(input), Ok((" ", "test")));
    }
}
