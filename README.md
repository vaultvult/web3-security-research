# Web3 Security Research Portfolio

Welcome to my security research portfolio. This repository contains Proof-of-Concept (PoC) exploits and write-ups for vulnerabilities I have discovered in both EVM (Solidity) and Solana (Rust) smart contracts.

## 📁 Repository Structure

### 1. `solana-rust-security`
- **Language:** Rust (Anchor Framework)
- **Vulnerabilities:** Integer Underflow (DoS) & Missing Access Control
- **Impact:** Medium / High
- **Summary:** Rust PoCs demonstrating a math underflow in `buy_exact_sol_in` and an identity spoofing flaw in the `create` instruction on a Solana launchpad contract.

*(For the Solidity/Foundry PoCs, please see my pinned repository: `tenor-labs-solidity`)*
