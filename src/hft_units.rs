#[warn(unused_imports)]

use core::fmt;

use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::cmp::PartialEq;
use std::borrow::Borrow;
use std::marker::PhantomData;
use fixed::types::I80F48;

// TODO derive symbol from S
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Mint<S> {
    symbol: &'static str,
    decimals: u8, // e9
    _marker: PhantomData<S>,
}

impl<U> Mint<U> {
    pub fn new(symbol: &'static str, decimals: u8) -> Self {
        assert!(decimals <= 19, "decimals must be <= 19");
        Mint {
            symbol: symbol,
            decimals: decimals,
            _marker: PhantomData,
        }
    }

    pub fn symbol(&self) -> &'static str {
        self.symbol
    }
    pub fn decimals(&self) -> u8 {
        self.decimals
    }
    // TODO rename .. maybe one_in_native, units
    pub fn inverse_unit(&self) -> u64 {
        10u64.pow(self.decimals as u32)
    }

    pub fn unit(&self) -> I80F48 {
        I80F48::ONE / I80F48::from_num(self.inverse_unit())
    }

}

// TODO define ...
pub struct Market<B, Q>
{
    base_mint: Mint<B>,
    quote_mint: Mint<Q>,
    base_lot_size: i64,
    // Number of quote native in a quote lot. Must be a power of 10.
    quote_lot_size: i64,
}

impl<B, Q> Market<B, Q>
where
    B: PartialEq + Clone,
    Q: PartialEq + Clone,
{
    pub fn new(
        base_mint: &Mint<B>, quote_mint: &Mint<Q>,
        base_lot_size: i64, quote_lot_size: i64,
    ) -> Self {
        // TODO assert quote_lot_size is power of 10
        Market {
            base_mint: base_mint.clone(),
            quote_mint: quote_mint.clone(),
            base_lot_size,
            quote_lot_size,
        }
    }
    pub fn base_mint(&self) -> &Mint<B> {
        &self.base_mint
    }
    pub fn quote_mint(&self) -> &Mint<Q> {
        &self.quote_mint
    }

    // pub fn quote_lot_to_native(&self, quote_lots: i64) -> NativeSize {
    //     NativeSize {
    //         amount: I80F48::from_num(quote_lots) * I80F48::from_num(self.quote_lot_size),
    //     }
    // }

    // quote lots to quote native
    pub fn quote_lots_to_native(&self, quote_lots: &LotAmount<Q>) -> NativeAmount<Q> {
        let native = I80F48::from_num(quote_lots.amount) * I80F48::from_num(self.quote_lot_size);
            // / I80F48::from_num(self.base_lot_size);
        NativeAmount::from_raw(&self.quote_mint, native)
    }

    // quote_native * base_lot_size = native_price * quote_lot_size
    // quote_native / quote_lot_size = native_price / base_lot_size
    // 19710000 / 100 = native_price / 10_000_000
    // => native_price = 19710000 * 100_000 = 1971e9

    // calculate price-native to price-lots
    pub fn native_price_to_lot(&self, quote_native: I80F48) -> i64 {
        (quote_native * I80F48::from_num(self.base_lot_size) / I80F48::from_num(self.quote_lot_size))
            .to_num()
    }


    // amount == lots
    pub fn from_raw_as_base(&self, amount: i64) -> LotAmount<B> {

        LotAmount {
            amount: amount,
            _marker: PhantomData,
        }
    }

    pub fn from_raw_as_quote(&self, amount: i64) -> LotAmount<Q> {
        LotAmount {
            amount: amount,
            _marker: PhantomData,
        }
    }


}


/// TODO
#[derive(PartialEq, Debug, Clone, Copy)]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct NativeAmount<S>
where S: PartialEq {
    amount: I80F48, // TODO this could be u64/i64
    _marker: PhantomData<S>,
}



// note: S maybe should be replaced by Mint<S>; ATM this is only guaranteed by the factory method
impl<S> NativeAmount<S>
where S: PartialEq {

    pub fn unit_symbol() -> &'static str { "nsz" }

    pub fn unit_name() -> &'static str { "native size" }

    // TODO find better name ('raw' might be misleading)
    pub fn from_raw(mint: &Mint<S>, amount: I80F48) -> Self {
        NativeAmount {
            amount: amount,
            _marker: PhantomData,
        }
    }

    pub fn to_ui(&self, mint: &Mint<S>) -> I80F48 {
        self.amount.mul(I80F48::from_num(mint.unit()))
    }

}

pub fn convert_native_to_ui<S: PartialEq>(native_amount: NativeAmount<S>, mint: &Mint<S>) -> I80F48 {
    native_amount.to_ui(mint)
}

impl<S> fmt::Display for NativeAmount<S>
where S: PartialEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO fix fmt of I80F48
        write!(f, "{:?} {}", &self.amount, Self::unit_symbol())
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct LotAmount<S>  {
    // in lots
    amount: i64,
    _marker: PhantomData<S>,
}

impl<S> LotAmount<S> {

    pub fn unit_symbol() -> &'static str { "lots" }

    pub fn unit_name() -> &'static str { "lots" }

    // pub fn to_native(&self, market: &Market<B, Q>) -> I80F48 {
    //     I80F48::from_num(amount) * I80F48::from_num(market.quote_lot_size)
    //         / I80F48::from_num(market.base_lot_size)
    // }

    // pub fn native_price_to_lot(&self, quote_native: I80F48) -> i64 {
    //     (quote_native * I80F48::from_num(self.base_lot_size) / I80F48::from_num(self.quote_lot_size))
    //         .to_num()
    // }

}


// how to model
// TODO
// we need LotPrice and NativePrice
//
// Amt<B> * Price<B,Q> = Amt<Q>
// native_price_to_lot : LotPrice<B,Q>
struct Price {

}


// pub fn lot_to_native_price(&self, price: i64) -> I80F48 {
//     I80F48::from_num(price) * I80F48::from_num(self.quote_lot_size)
//         / I80F48::from_num(self.base_lot_size)
// }