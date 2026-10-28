# Onre Protocol - Smart Contract Audit (Solana)

## Overview
A security review of the Onre Protocol (Prop AMM) written in Rust/Anchor on Solana. This audit focused on the core pricing logic, dynamic liquidity walls, and token transfer utilities.

## Summary of Findings

| ID | Title | Severity | Status |
|---|---|---|---|
| 1 | Uninitialized Permissionless Fee | Medium | Unresolved |
| 2 | Arithmetic Overflow DoS for High-Decimal Tokens | Low | Unresolved |

## Scope
- `programs/onreapp/src/instructions/prop_amm/sell.rs`
- `programs/onreapp/src/instructions/prop_amm/buy.rs`
- `programs/onreapp/src/instructions/offer/offer_utils.rs`
- `programs/onreapp/src/instructions/offer/make_offer.rs`
- `programs/onreapp/src/utils/token_utils.rs`
