# Finding 1: Uninitialized Permissionless Fee

## Severity: Medium
## Impact: Loss of Protocol Revenue

## Description
When an Offer is created via the `make_offer` instruction, the `fee_basis_points_permissionless` field is not explicitly initialized. Because the `Offer` struct uses `#[account(zero_copy)]`, this field defaults to `0`.

The `buy.rs` (permissionless buy) instruction relies on `offer.permissionless_fee_basis_points()` to calculate the fee. Because it defaults to 0, all permissionless buys will incur zero fees unless the admin manually calls a separate update instruction immediately after creation. If the admin forgets, the protocol loses all fee revenue from permissionless buys.

## Proof of Concept
1. Admin calls `make_offer(fee_basis_points=100, needs_approval=false, allow_permissionless=true)`.
2. `Offer.fee_basis_points` is set to 100.
3. `Offer.fee_basis_points_permissionless` remains 0.
4. A user calls `open_swap_buy` (permissionless path).
5. `process_offer_core` uses `offer.permissionless_fee_basis_points()`, which returns 0.
6. User pays 0 fees.

## Recommendation
In `make_offer.rs`, explicitly initialize the permissionless fee to a default value (e.g., the same as `fee_basis_points`):
```rust
offer.set_permissionless_fee_basis_points(fee_basis_points);
```
