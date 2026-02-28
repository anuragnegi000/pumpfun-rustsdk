use std::env;
use std::str::FromStr;

use pumpfun_new_rust_sdk::{
    accounts::{BondingCurve, Global, BONDING_CURVE_SEED, GLOBAL_SEED},
    helpers::{
        build_buy_exact_sol_in_tx_cached, build_buy_tx_cached, pick_fee_recipient,
        PumpFunResolvedData,
    },
    PUMP_ID,
};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSimulateTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
};

const TOKEN_2022: Pubkey = Pubkey::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
const SPL_TOKEN: Pubkey = Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

fn quote_buy(sol_amount: u64, virtual_sol_reserves: u64, virtual_token_reserves: u64) -> u64 {
    let sol = sol_amount as u128;
    let vsr = virtual_sol_reserves as u128;
    let vtr = virtual_token_reserves as u128;
    let tokens = sol * vtr / (vsr + sol);
    tokens as u64
}

fn main() {
    dotenvy::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: cargo run --example buy --features fetch -- <mint> <sol_amount> [--send] [--legacy-buy]"
        );
        std::process::exit(1);
    }

    let mint = Pubkey::from_str(&args[1]).expect("invalid mint address");
    let sol_amount: f64 = args[2].parse().expect("invalid sol amount");
    let should_send = args.iter().any(|a| a == "--send");
    let use_legacy_buy = args.iter().any(|a| a == "--legacy-buy");
    let slippage_bps: u64 = env::var("SLIPPAGE_BPS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(500);

    let rpc_url =
        env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");

    let key_bytes = bs58::decode(&private_key)
        .into_vec()
        .expect("invalid base58 private key");
    let keypair =
        Keypair::try_from(key_bytes.as_slice()).expect("invalid keypair bytes (need 64 bytes)");
    let wallet = keypair.pubkey();

    let mode_str = if use_legacy_buy {
        "Buy (legacy)"
    } else {
        "BuyExactSolIn"
    };
    println!("=== PumpFun {} ===", mode_str);
    println!("Wallet:      {}", wallet);
    println!("Mint:        {}", mint);
    println!("SOL:         {} SOL", sol_amount);
    println!("Slippage:    {} bps", slippage_bps);
    println!(
        "Mode:        {}",
        if should_send { "SEND" } else { "SIMULATE" }
    );
    println!();

    let rpc = RpcClient::new_with_commitment(&rpc_url, CommitmentConfig::confirmed());

    let (global_pda, _) = Pubkey::find_program_address(&[GLOBAL_SEED], &PUMP_ID);
    let global_account = rpc
        .get_account(&global_pda)
        .expect("failed to fetch Global");
    let global_data =
        Global::from_bytes(&global_account.data).expect("failed to deserialize Global");
    let fee_recipient = pick_fee_recipient(&global_data);

    let (bonding_curve, _) =
        Pubkey::find_program_address(&[BONDING_CURVE_SEED, mint.as_ref()], &PUMP_ID);
    let bc_account = rpc
        .get_account(&bonding_curve)
        .expect("failed to fetch bonding curve");
    let curve =
        BondingCurve::from_bytes(&bc_account.data).expect("failed to deserialize bonding curve");

    if curve.complete {
        eprintln!("ERROR: Bonding curve migrated");
        std::process::exit(1);
    }

    let mint_account = rpc.get_account(&mint).expect("failed to fetch mint");
    let token_program = if mint_account.owner == TOKEN_2022 {
        TOKEN_2022
    } else {
        SPL_TOKEN
    };

    let resolved = PumpFunResolvedData {
        mint,
        creator: curve.creator,
        token_program,
        fee_recipient,
        is_cashback_coin: curve.is_cashback_coin,
    };

    let sol_lamports = (sol_amount * LAMPORTS_PER_SOL as f64) as u64;
    let blockhash = rpc.get_latest_blockhash().expect("failed to get blockhash");
    let priority_fee = env::var("PRIORITY_FEE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000u64);

    let mut tx = if use_legacy_buy {
        let fee_bps = (global_data.fee_basis_points + global_data.creator_fee_basis_points) as u64;
        let net_sol = sol_lamports * 10000 / (10000 + fee_bps);
        let expected_tokens = quote_buy(
            net_sol,
            curve.virtual_sol_reserves,
            curve.virtual_token_reserves,
        );
        let min_tokens = expected_tokens * (10000 - slippage_bps) / 10000;

        println!(
            "Using Buy: amount={} ({:.2} tokens) max_sol_cost={}",
            min_tokens,
            min_tokens as f64 / 1_000_000.0,
            sol_lamports
        );
        build_buy_tx_cached(
            &resolved,
            &wallet,
            min_tokens,
            sol_lamports,
            priority_fee,
            blockhash,
        )
        .expect("failed to build Buy tx")
    } else {
        let fee_bps = (global_data.fee_basis_points + global_data.creator_fee_basis_points) as u64;
        let net_sol = sol_lamports * 10000 / (10000 + fee_bps);
        let expected_tokens = quote_buy(
            net_sol,
            curve.virtual_sol_reserves,
            curve.virtual_token_reserves,
        );
        let min_tokens_out = expected_tokens * (10000 - slippage_bps) / 10000;
        println!(
            "Using BuyExactSolIn: sol_in={} lamports, min_tokens_out={} ({:.2} tokens)",
            sol_lamports,
            min_tokens_out,
            min_tokens_out as f64 / 1_000_000.0,
        );
        build_buy_exact_sol_in_tx_cached(
            &resolved,
            &wallet,
            sol_lamports,
            min_tokens_out,
            priority_fee,
            blockhash,
        )
        .expect("failed to build BuyExactSolIn tx")
    };

    tx.sign(&[&keypair], blockhash);

    if should_send {
        println!(">>> SENDING...");
        match rpc.send_and_confirm_transaction(&tx) {
            Ok(sig) => {
                println!("SUCCESS! https://solscan.io/tx/{}", sig);
            }
            Err(e) => {
                eprintln!("FAILED: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!(">>> SIMULATING...");
        let config = RpcSimulateTransactionConfig {
            sig_verify: false,
            replace_recent_blockhash: true,
            ..Default::default()
        };
        match rpc.simulate_transaction_with_config(&tx, config) {
            Ok(result) => {
                if let Some(err) = result.value.err {
                    eprintln!("FAILED: {:?}", err);
                    if let Some(logs) = &result.value.logs {
                        for log in logs {
                            eprintln!("  {}", log);
                        }
                    }
                    std::process::exit(1);
                } else {
                    println!("SUCCESS! CU: {:?}", result.value.units_consumed);
                }
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                std::process::exit(1);
            }
        }
    }
}
