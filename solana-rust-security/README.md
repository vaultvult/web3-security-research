# Solana (Rust) Security Research Snippets

This repository contains isolated Rust Proof-of-Concepts (PoCs) demonstrating common smart contract vulnerabilities found in Solana programs (Anchor framework).

## 1. Integer Underflow in `buy_exact_sol_in`
**Target:** Pump.fun (Solana)
**Severity:** Medium (Denial of Service)
**Vulnerability:** Passing a dust amount (1 lamport) causes the net SOL calculation to floor to `0`. The subsequent math `(net_sol - 1)` causes a Rust underflow panic, reverting the transaction.

## 2. Missing Creator Check (Access Control)
**Target:** Pump.fun (Solana)
**Severity:** Medium / High
**Vulnerability:** The `create` instruction lacks a check to ensure the `signer` matches the `creator` argument. An attacker can launch tokens under a victim's identity to spoof them or redirect/grief creator fees.
