pub use self::f64consts::*;

// pub trait Price: Dimensioned {}
// pub trait Size: Dimensioned {}
// pub trait Notional: Dimensioned {}

// dimensions
// pub trait Quantity: Dimensioned {}
// pub trait Money: Dimensioned {}


make_units! {
    HFT; // name of unit system
    ONE: Unitless;
    base {
        // PRICE: Price, "price", Money;
        SIZE: Size, "size";
        NOTIONAL: Notional, "notional";
        // M: Meter, "m", Length;
        // S: Second, "s", Time;
    }
    derived {
        PRICE: Price = (Notional / Size);
        // MPS: MeterPerSecond = (Meter / Second), Velocity;
        // HZ: Hertz = (Unitless / Second), Frequency;
        // M3: Meter3 = (Meter * Meter * Meter), Volume;
        // M5: Meter5 = (Meter3 * Meter * Meter);
    }
    constants {
        // FT: Meter = 0.3048;
        // CM: Meter = CENTI * M.value_unsafe;
        // MIN: Second = 60.0;
        // HR: Second = 60.0 * MIN.value_unsafe;
        // PI: Unitless = consts::PI;
    }
    fmt = true;
}
