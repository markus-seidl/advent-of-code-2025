use crate::adc01::adc01;

mod adc01;
mod adc02;

fn main() {
    adc02::adc02().unwrap();
}
