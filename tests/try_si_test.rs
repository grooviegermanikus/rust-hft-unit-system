extern crate dimensioned as dim;

use dim::si;

#[test]
fn test_si() {
    let si_x = 6.0 * si::M;
    let si_t = 3.0 * si::S;
    let si_v = 2.0 * si::M / si::S;

    let notallowed = si::S * si::S;


    println!("si_x = {}", si_x);
    println!("si_v = {}", si_v);


    // let p: ms::PRICE;

}
