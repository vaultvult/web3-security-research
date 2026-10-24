# Web3 Security Research Portfolio

Welcome to my security research portfolio. This repository contains Proof-of-Concept (PoC) exploits and write-ups for vulnerabilities I have discovered in both EVM (Solidity) and Solana (Rust) smart contracts.

## 📁 Repository Structure

### 1. `tenor-labs-solidity`
- **Language:** Solidity (Foundry)
- **Vulnerability:** Rounding Error / LTV Inflation
- **Impact:** High
- **Summary:** A "dust fill" attack during a Morpho Midnight position migration causes collateral to round down to zero, allowing an attacker to liquidate unbacked debt.

### 2. `solana-rust-security`
- **Language:** Rust (Anchor Framework)
- **Vulnerabilities:** Integer Underflow (DoS) & Missing Access Control
- **Impact:** Medium / High
- **Summary:** Rust PoCs demonstrating a math underflow in `buy_exact_sol_in` and an identity spoofing flaw in the `create` instruction on a Solana launchpad contract.
