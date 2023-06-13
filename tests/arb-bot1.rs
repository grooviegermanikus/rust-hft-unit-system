
#[test]
fn router_exactout() {
    // 1,753066 USDC
    // 6 decimals
    let in_amount = 1753066;
    // 0,00100167 ETH
    // 8 decimals
    let out_amount = 100000;

    // https://api.mngo.cloud/router/v1/swap
    // ?inputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    // &outputMint=7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs
    // &amount=100000
    // &slippage=0.005&feeBps=0&mode=ExactOut
    // &wallet=11111111111111111111111111111111
}