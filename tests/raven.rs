

#[test]
fn base_to_native() {

    let perp_base_lots = 10 as u64;
    let perp_ask_price_lots = 20_000 as u64;
    // perp market option:
    let market_quote_lot_size = 100 as u64;

    let perp_ask_quote_lots = perp_base_lots
        .checked_mul(perp_ask_price_lots)
        .unwrap();
    let perp_ask_quote_native = i64::try_from(
        perp_ask_quote_lots // price
            .checked_mul(market_quote_lot_size)
            .unwrap(),
    ).unwrap();
    assert_eq!(perp_ask_quote_native, 20000000);

}

#[test]
fn sol_trace() {
    // example sell 1 SOL (e9,l7), best bid is 19.71 USDC (e6, l2)
    // SOL-PERP: lot size = 1e7; decimals = 1e9
    // base_ui = 1
    // base_native  = 1e9
    // base_lots (SOL) = 1e9 / 1l7 = 1e2
    // price_lots (USDC) = 1971
    // quote_lots (USDC) = base_lots * price_lots = 1971e2
    // quote_native(USDC) = 1971e2 * 1l2 = 1971e4
    // quote_ui = 19.71

    // untyped
    {
        // base_native  = 1e9
        let base_native = 1_000_000_000; // =e9  smallest unit
        // base_lots (SOL) = 1e9 / 1l7 = 1e2
        let base_lot_size = 10_000_000; // =l7, number of native units in a lot
        let base_lots = 1 * 100; // qty of lots to trade -> 100 lots = 1 SOL
        let base_ui = base_lots * base_lot / base_native;
        assert_eq!(base_ui, 1);

        // price_lots (USDC) = 1971
        let price_lots = 1971;
        // quote_lots (USDC) = base_lots * price_lots = 1971e2
        let quote_lots = base_lots * price_lots;
        // quote_native(USDC) = 1971e2 * 1l2 = 1971e4
        let quote_native = quote_lots * 100;
        // quote_ui = 19.71
        let quote_ui = quote_native as f64 / 1_000_000.;
        assert_eq!(quote_ui, 19.71);
    }

    // typed
    {
        let base_native = NativeSize::from(1_000_000_000);
        let base_lot_size = NativeSizePerLot::from_native_per_lots(10_000_000);
        let base_lots = Lots::from(100);
        // let base_base_ui = base_lots * base_lot / base_native;
        // assert_eq!(base_ui, 1);
    }

}
