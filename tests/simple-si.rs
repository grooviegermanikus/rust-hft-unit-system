use simple_si_units::base::{Distance, Time};
use simple_si_units::mechanical::AreaDensity;

#[test]
fn test_si() {

    let box_width = Distance::from_cm(33.5);
    let box_length = Distance::from_cm(45.0);
    let box_height = Distance::from_cm(23.5);
    let carboard_density = AreaDensity::from_grams_per_square_meter(300.);
    let t = Time::from_ms(10.);

    let foo = t + t;


    println!("box_width = {}", box_width);

}


#[test]
fn size_andPrice() {
    // let size = NativeSize::from(50);
    // let price = Price::from(1400);
    //
    // let notional: Notional = price * size;
    //
    // println!("notional = {}", notional);
    //
    // // TODO
    // // let _notional_deref = &price * &size;
    //
    //
    // // let _zzz = size * size;
    //
    //
    // assert_eq!(size * price, price * size, "commutative property");

}

