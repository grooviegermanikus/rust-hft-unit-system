use fixed::types::I80F48;
use rust_hft_unit_system::hft_units::Mint;

struct MarketConf {
    /// Number of quote native in a quote lot. Must be a power of 10.
    ///
    /// Primarily useful for increasing the tick size on the market: A lot price
    /// of 1 becomes a native price of quote_lot_size/base_lot_size becomes a
    /// ui price of quote_lot_size*base_decimals/base_lot_size/quote_decimals.
    pub quote_lot_size: i64,

    /// Number of base native in a base lot. Must be a power of 10.
    ///
    /// Example: If base decimals for the underlying asset is 6, base lot size
    /// is 100 and and base position lots is 10_000 then base position native is
    /// 1_000_000 and base position ui is 1.
    pub base_lot_size: i64,
}


#[test]
fn price_lot() {

    let price_lots = {
        let perp_market = MarketConf {
            quote_lot_size: 10,
            base_lot_size: 10000,
        };
        // let perp_market = solana.get_account::<PerpMarket>(perp_market).await;
        // perp_market.native_price_to_lot(I80F48::ONE)
        native_price_to_lot(&perp_market, I80F48::ONE)
    };
    assert_eq!(price_lots, 1000);

}

/// Convert from the price stored on the book to the price used in value calculations
//
fn lot_to_native_price(market_conf: &MarketConf, price: i64) -> I80F48 {
    I80F48::from_num(price) * I80F48::from_num(market_conf.quote_lot_size)
        / I80F48::from_num(market_conf.base_lot_size)
}

fn native_price_to_lot(market: &MarketConf, price: I80F48) -> i64 {
    (price * I80F48::from_num(market.base_lot_size) / I80F48::from_num(market.quote_lot_size))
        // TODO consider rounding
        // .ceil() - see rust-hacking -> rounding()
        .to_num()
}


