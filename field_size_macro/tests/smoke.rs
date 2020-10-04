#[macro_use]
extern crate field_size_macro;
use field_size::FieldSize;

#[derive(FieldSize)]
enum TestEnumEmpty {
}

#[allow(dead_code)]
#[derive(FieldSize)]
enum TestEnumOneVariant {
    Variant,
}

#[allow(dead_code)]
#[derive(FieldSize)]
enum TestEnumManyVariants {
    Variant1,
    Variant2,
    Variant3,
    Variant4,
    Variant5,
}

#[test]
fn returns_zero_on_empty() {
   assert_eq!(0, TestEnumEmpty::field_size()); 
}

#[test]
fn returns_one_on_one_variant() {
   assert_eq!(1, TestEnumOneVariant::field_size()); 
}

#[test]
fn counts_variants_correctly() {
   assert_eq!(5, TestEnumManyVariants::field_size()); 
}
