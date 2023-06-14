// https://twitter.com/chameleon_jeff/status/1652778368708882434

#[test]
fn foo() {
    // Price * Size = Notional
    // Size + Size = Size
    // But Size * Size? Nonsensical, and now you get a compiler error.

    // It doesn't make sense to add 1 BTC to 2 ETH.
    // So sizes should be Size(2, ETH), and you can check that the units agree in each of your arithmetic implementations.

    // To continue this type extension, prices would be Price(2000, USDT/BTC). That is, the price of one BTC is in units of USDT *per BTC*.

    // The units work out like:
    // Price(2000, USDT/BTC) * Size(2, BTC) = Notional(4000, USDT).

    //  it's helpful to include the exchange as part of the symbol too, so that you can't directly compare BTC on Binance with BTC on Coinbase.
}