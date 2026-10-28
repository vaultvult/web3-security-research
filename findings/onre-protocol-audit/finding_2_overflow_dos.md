# Finding 2: Arithmetic Overflow DoS for High-Decimal Tokens

## Severity: Low
## Impact: Denial of Service (transaction reverts) for large swaps

## Description
In `calculate_token_out_amount` (used by `buy.rs`) and `process_redemption_core` (used by `sell.rs`), the code calculates:
`numerator = token_in_amount * 10^(decimals + 9)`

If a user attempts to swap a massive amount of a token with high decimals (e.g., 18 decimals like wETH), the intermediate `u128` calculation will overflow.

While the code uses `checked_mul` and safely returns an error instead of panicking, it creates a Denial of Service for legitimate whale-sized swaps, as the transaction will revert.

## Proof of Concept
1. Assume `token_out_decimals` is 18.
2. `10^(18 + 9)` = `10^27`.
3. If `token_in_amount` is greater than ~340,000,000,000,000,000,000 units (achievable with 18-decimal tokens), the `checked_mul` overflows `u128::MAX`.
4. The transaction reverts with `MathOverflow`.

## Recommendation
While the transaction reverting is safe (no funds are lost), it limits the maximum swap size. If large swaps are expected, consider using a custom `mul_div` function that handles larger numbers, or ensure that the UI properly limits the maximum input based on the token's decimals.

