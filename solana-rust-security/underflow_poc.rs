fn main() {
    println!("--- Solana: buy_exact_sol_in Underflow PoC ---");

    // 1. Attacker passes 1 lamport (dust amount)
    let spendable_sol_in: u64 = 1;
    let total_fee_bps: u64 = 100; // 1% fee
    
    // 2. Calculate net_sol (using IDL floor division formula)
    // 1 * 10000 / 10100 = 0
    let net_sol = (spendable_sol_in * 10_000) / (10_000 + total_fee_bps);
    println!("Net SOL calculated: {}", net_sol);

    // 3. Calculate tokens out
    let virtual_token_reserves: u64 = 1_000_000_000;
    let virtual_sol_reserves: u64 = 30_000_000_000;

    // 4. VULNERABILITY: (net_sol - 1) causes underflow when net_sol == 0
    println!("Attempting to calculate tokens_out...");
    let net_sol_minus_one = net_sol - 1; // Panics here!
    
    let tokens_out = (net_sol_minus_one * virtual_token_reserves) / (virtual_sol_reserves + net_sol_minus_one);
    println!("Tokens out: {}", tokens_out);
}
