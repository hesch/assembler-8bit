use crate::output_datastructures::ControlWord;

pub trait GenMicrocode {
    fn test();
    fn microcode() -> [ControlWord];
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
