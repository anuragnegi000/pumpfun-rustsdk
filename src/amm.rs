pub const PUMP_FEE_BPS: u64 = 100;

pub fn calculate_tokens_out(
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    sol_amount_in: u64,
    fee_bps: u64,
) -> u64 {
    if virtual_sol_reserves == 0 || virtual_token_reserves == 0 || sol_amount_in == 0 {
        return 0;
    }
    let fee = (sol_amount_in as u128 * fee_bps as u128 / 10000) as u64;
    let sol_in_after_fee = sol_amount_in.saturating_sub(fee) as u128;
    let virtual_sol = virtual_sol_reserves as u128;
    let virtual_token = virtual_token_reserves as u128;
    let k = virtual_sol * virtual_token;
    let new_virtual_sol = virtual_sol + sol_in_after_fee;
    let new_virtual_token = k / new_virtual_sol;
    virtual_token.saturating_sub(new_virtual_token) as u64
}

pub fn calculate_sol_out(
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    token_amount_in: u64,
    fee_bps: u64,
) -> u64 {
    if virtual_sol_reserves == 0 || virtual_token_reserves == 0 || token_amount_in == 0 {
        return 0;
    }
    let virtual_sol = virtual_sol_reserves as u128;
    let virtual_token = virtual_token_reserves as u128;
    let token_in = token_amount_in as u128;
    let k = virtual_sol * virtual_token;
    let new_virtual_token = virtual_token + token_in;
    let new_virtual_sol = k / new_virtual_token;
    let sol_out_gross = virtual_sol.saturating_sub(new_virtual_sol) as u64;
    let fee = (sol_out_gross as u128 * fee_bps as u128 / 10000) as u64;
    sol_out_gross.saturating_sub(fee)
}

pub fn calculate_sol_required(
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    token_amount_out: u64,
    fee_bps: u64,
) -> u64 {
    if token_amount_out >= virtual_token_reserves {
        return u64::MAX;
    }
    let virtual_sol = virtual_sol_reserves as u128;
    let virtual_token = virtual_token_reserves as u128;
    let tokens_out = token_amount_out as u128;
    let k = virtual_sol * virtual_token;
    let new_virtual_token = virtual_token - tokens_out;
    let new_virtual_sol = k / new_virtual_token;
    let sol_before_fee = (new_virtual_sol - virtual_sol) as u64;
    let fee = (sol_before_fee as u128 * fee_bps as u128 / 10000) as u64;
    sol_before_fee.saturating_add(fee).saturating_add(1)
}

pub fn calculate_price(virtual_sol_reserves: u64, virtual_token_reserves: u64) -> f64 {
    if virtual_token_reserves == 0 {
        return 0.0;
    }
    (virtual_sol_reserves as f64) / (virtual_token_reserves as f64)
}
