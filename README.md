


## Vague ideas

### constant generic types
* provide universal definitions and constants at compile-time - e.g. number of decimals
  * instead of ... `Mint::new("USDC", 6)`
  * do this ... `type Mint_USDC = Mint_typed<"USDC", 6>` (uses constant generic types)
  * we should provide both and implement a shared trait
* collect all types and combinations (similar to _simple_units_)
