use gen_microcode_macro::gen_microcode;
use gen_microcode::GenMicrocode;

#[derive(gen_microcode)]
enum NoFieldsEnum {
    Variant1,
    Variant2,
    Variant3,
    Variant4,
    Variant5,
}

enum TestEnum {
    V0 = 0,
    V1
}

#[derive(gen_microcode)]
enum FieldsEnum {
    Variant1(TestEnum),
/*    Variant2(NoFieldsEnum, NoFieldsEnum),
    Variant4(u8),
    Variant5(u8, i32),*/
}

#[test]
fn returns_zero_on_first_variant() {
   assert_eq!(0u8, NoFieldsEnum::Variant1.into()); 
}

#[test]
fn returns_four_on_fith_variant() {
   assert_eq!(4u8, NoFieldsEnum::Variant5.into()); 
}
