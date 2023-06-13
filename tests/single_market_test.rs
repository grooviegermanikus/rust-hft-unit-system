extern crate dimensioned as dim;

use rust_hft_unit_system::hft_units::u64consts::*;

#[test]
fn test_ff() {

    let size = 50 * SIZE;
    // let price = 1400 * PRICE;
    let notional = 10000 * NOTIONAL;

    let _zzz = notional / size;


    println!("size = {}", size);
    // println!("price = {}", price);
    println!("notional = {}", notional);

    let sum = size + size;
    println!("sum = {}", sum);

    // binary_unit_change
    // let _doh = size + price;
    // let _doh = size * size;
}



