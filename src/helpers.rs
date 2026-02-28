use crate::{
    accounts::{
        BondingCurve, Global, BONDING_CURVE_SEED, BONDING_CURVE_V2_SEED, CREATOR_VAULT_SEED,
        EVENT_AUTHORITY_SEED, GLOBAL_SEED, GLOBAL_VOLUME_ACCUMULATOR_SEED,
        USER_VOLUME_ACCUMULATOR_SEED,
    },
    instructions::{
        BuyBuilder, BuyExactSolInBuilder, InitUserVolumeAccumulatorBuilder, SellBuilder,
    },
    types::OptionBool,
    PUMP_ID,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, hash::Hash, message::Message, pubkey::Pubkey,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;

const FEE_CONFIG: Pubkey = Pubkey::from_str_const("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt");
const FEE_PROGRAM: Pubkey = Pubkey::from_str_const("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ");
const SPL_TOKEN_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const TOKEN_2022_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

pub struct PumpFunResolvedData {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub token_program: Pubkey,
    pub fee_recipient: Pubkey,
    pub is_cashback_coin: bool,
}

pub fn pick_fee_recipient(global: &crate::accounts::Global) -> Pubkey {
    let zero = Pubkey::default();
    for fr in &global.fee_recipients {
        let pk: Pubkey = Pubkey::new_from_array(fr.to_bytes());
        if pk != zero {
            return pk;
        }
    }
    Pubkey::new_from_array(global.fee_recipient.to_bytes())
}

pub fn derive_bonding_curve_v2(mint: &Pubkey) -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(&[BONDING_CURVE_V2_SEED, mint.as_ref()], &PUMP_ID);
    pda
}

pub fn build_buy_tx_cached(
    resolved: &PumpFunResolvedData,
    wallet: &Pubkey,
    token_amount: u64,
    max_sol_cost: u64,
    priority_fee: u64,
    recent_blockhash: Hash,
) -> Result<Transaction, String> {
    let (bonding_curve, _) =
        Pubkey::find_program_address(&[BONDING_CURVE_SEED, resolved.mint.as_ref()], &PUMP_ID);
    let associated_bonding_curve = get_associated_token_address_with_program_id(
        &bonding_curve,
        &resolved.mint,
        &resolved.token_program,
    );
    let associated_user = get_associated_token_address_with_program_id(
        wallet,
        &resolved.mint,
        &resolved.token_program,
    );
    let (global, _) = Pubkey::find_program_address(&[GLOBAL_SEED], &PUMP_ID);
    let (event_authority, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &PUMP_ID);
    let (creator_vault, _) =
        Pubkey::find_program_address(&[CREATOR_VAULT_SEED, resolved.creator.as_ref()], &PUMP_ID);
    let (global_volume_acc, _) =
        Pubkey::find_program_address(&[GLOBAL_VOLUME_ACCUMULATOR_SEED], &PUMP_ID);
    let (user_volume_acc, _) =
        Pubkey::find_program_address(&[USER_VOLUME_ACCUMULATOR_SEED, wallet.as_ref()], &PUMP_ID);
    let bonding_curve_v2 = derive_bonding_curve_v2(&resolved.mint);

    let buy_ix = BuyBuilder::new()
        .global(global)
        .fee_recipient(resolved.fee_recipient)
        .mint(resolved.mint)
        .bonding_curve(bonding_curve)
        .associated_bonding_curve(associated_bonding_curve)
        .associated_user(associated_user)
        .user(*wallet)
        .token_program(resolved.token_program)
        .creator_vault(creator_vault)
        .event_authority(event_authority)
        .global_volume_accumulator(global_volume_acc)
        .user_volume_accumulator(user_volume_acc)
        .fee_config(FEE_CONFIG)
        .fee_program(FEE_PROGRAM)
        .amount(token_amount)
        .max_sol_cost(max_sol_cost)
        .track_volume(OptionBool::new(false))
        .add_remaining_account(solana_instruction::AccountMeta::new_readonly(
            bonding_curve_v2,
            false,
        ))
        .instruction();

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(200_000),
        ComputeBudgetInstruction::set_compute_unit_price(priority_fee),
        create_associated_token_account_idempotent(
            wallet,
            wallet,
            &resolved.mint,
            &resolved.token_program,
        ),
        buy_ix,
    ];

    let message = Message::new(&instructions, Some(wallet));
    let mut tx = Transaction::new_unsigned(message);
    tx.message.recent_blockhash = recent_blockhash;
    Ok(tx)
}

pub fn build_init_user_volume_accumulator_ix(
    wallet: &Pubkey,
) -> solana_sdk::instruction::Instruction {
    let (event_authority, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &PUMP_ID);
    let (user_volume_acc, _) =
        Pubkey::find_program_address(&[USER_VOLUME_ACCUMULATOR_SEED, wallet.as_ref()], &PUMP_ID);
    InitUserVolumeAccumulatorBuilder::new()
        .payer(*wallet)
        .user(*wallet)
        .user_volume_accumulator(user_volume_acc)
        .event_authority(event_authority)
        .program(PUMP_ID)
        .instruction()
}

pub fn build_buy_exact_sol_in_tx_cached(
    resolved: &PumpFunResolvedData,
    wallet: &Pubkey,
    spendable_sol_in: u64,
    min_tokens_out: u64,
    priority_fee: u64,
    recent_blockhash: Hash,
) -> Result<Transaction, String> {
    let (bonding_curve, _) =
        Pubkey::find_program_address(&[BONDING_CURVE_SEED, resolved.mint.as_ref()], &PUMP_ID);
    let associated_bonding_curve = get_associated_token_address_with_program_id(
        &bonding_curve,
        &resolved.mint,
        &resolved.token_program,
    );
    let associated_user = get_associated_token_address_with_program_id(
        wallet,
        &resolved.mint,
        &resolved.token_program,
    );
    let (global, _) = Pubkey::find_program_address(&[GLOBAL_SEED], &PUMP_ID);
    let (event_authority, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &PUMP_ID);
    let (creator_vault, _) =
        Pubkey::find_program_address(&[CREATOR_VAULT_SEED, resolved.creator.as_ref()], &PUMP_ID);
    let (global_volume_acc, _) =
        Pubkey::find_program_address(&[GLOBAL_VOLUME_ACCUMULATOR_SEED], &PUMP_ID);
    let (user_volume_acc, _) =
        Pubkey::find_program_address(&[USER_VOLUME_ACCUMULATOR_SEED, wallet.as_ref()], &PUMP_ID);
    let bonding_curve_v2 = derive_bonding_curve_v2(&resolved.mint);

    let buy_ix = BuyExactSolInBuilder::new()
        .global(global)
        .fee_recipient(resolved.fee_recipient)
        .mint(resolved.mint)
        .bonding_curve(bonding_curve)
        .associated_bonding_curve(associated_bonding_curve)
        .associated_user(associated_user)
        .user(*wallet)
        .token_program(resolved.token_program)
        .creator_vault(creator_vault)
        .event_authority(event_authority)
        .global_volume_accumulator(global_volume_acc)
        .user_volume_accumulator(user_volume_acc)
        .fee_config(FEE_CONFIG)
        .fee_program(FEE_PROGRAM)
        .spendable_sol_in(spendable_sol_in)
        .min_tokens_out(min_tokens_out)
        .track_volume(OptionBool::new(false))
        .add_remaining_account(solana_instruction::AccountMeta::new_readonly(
            bonding_curve_v2,
            false,
        ))
        .instruction();

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(200_000),
        ComputeBudgetInstruction::set_compute_unit_price(priority_fee),
        create_associated_token_account_idempotent(
            wallet,
            wallet,
            &resolved.mint,
            &resolved.token_program,
        ),
        buy_ix,
    ];

    let message = Message::new(&instructions, Some(wallet));
    let mut tx = Transaction::new_unsigned(message);
    tx.message.recent_blockhash = recent_blockhash;
    Ok(tx)
}

pub fn build_sell_tx_cached(
    resolved: &PumpFunResolvedData,
    wallet: &Pubkey,
    token_amount: u64,
    min_sol_output: u64,
    priority_fee: u64,
    recent_blockhash: Hash,
) -> Result<Transaction, String> {
    let (bonding_curve, _) =
        Pubkey::find_program_address(&[BONDING_CURVE_SEED, resolved.mint.as_ref()], &PUMP_ID);
    let associated_bonding_curve = get_associated_token_address_with_program_id(
        &bonding_curve,
        &resolved.mint,
        &resolved.token_program,
    );
    let associated_user = get_associated_token_address_with_program_id(
        wallet,
        &resolved.mint,
        &resolved.token_program,
    );
    let (global, _) = Pubkey::find_program_address(&[GLOBAL_SEED], &PUMP_ID);
    let (event_authority, _) = Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &PUMP_ID);
    let (creator_vault, _) =
        Pubkey::find_program_address(&[CREATOR_VAULT_SEED, resolved.creator.as_ref()], &PUMP_ID);

    let bonding_curve_v2 = derive_bonding_curve_v2(&resolved.mint);

    let mut sell_builder = SellBuilder::new();
    sell_builder
        .global(global)
        .fee_recipient(resolved.fee_recipient)
        .mint(resolved.mint)
        .bonding_curve(bonding_curve)
        .associated_bonding_curve(associated_bonding_curve)
        .associated_user(associated_user)
        .user(*wallet)
        .creator_vault(creator_vault)
        .token_program(resolved.token_program)
        .event_authority(event_authority)
        .fee_config(FEE_CONFIG)
        .fee_program(FEE_PROGRAM)
        .amount(token_amount)
        .min_sol_output(min_sol_output);

    if resolved.is_cashback_coin {
        let (user_volume_acc, _) = Pubkey::find_program_address(
            &[USER_VOLUME_ACCUMULATOR_SEED, wallet.as_ref()],
            &PUMP_ID,
        );
        sell_builder
            .add_remaining_account(solana_instruction::AccountMeta::new(user_volume_acc, false));
    }
    sell_builder.add_remaining_account(solana_instruction::AccountMeta::new_readonly(
        bonding_curve_v2,
        false,
    ));

    let sell_ix = sell_builder.instruction();

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(200_000),
        ComputeBudgetInstruction::set_compute_unit_price(priority_fee),
        sell_ix,
    ];

    let message = Message::new(&instructions, Some(wallet));
    let mut tx = Transaction::new_unsigned(message);
    tx.message.recent_blockhash = recent_blockhash;
    Ok(tx)
}

pub fn build_buy_tx(
    rpc: &RpcClient,
    wallet: &Pubkey,
    mint: &Pubkey,
    token_amount: u64,
    max_sol_cost: u64,
    priority_fee: u64,
) -> Result<Transaction, String> {
    let (global_pda, _) = Pubkey::find_program_address(&[GLOBAL_SEED], &PUMP_ID);
    let global_account = rpc
        .get_account(&global_pda)
        .map_err(|e| format!("Failed to fetch global: {}", e))?;
    let global_data = Global::from_bytes(&global_account.data)
        .map_err(|e| format!("Failed to deserialize global: {}", e))?;

    let (bonding_curve, _) =
        Pubkey::find_program_address(&[BONDING_CURVE_SEED, mint.as_ref()], &PUMP_ID);
    let bonding_curve_account = rpc
        .get_account(&bonding_curve)
        .map_err(|e| format!("Failed to fetch bonding curve: {}", e))?;
    let curve_data = BondingCurve::from_bytes(&bonding_curve_account.data)
        .map_err(|e| format!("Failed to deserialize bonding curve: {}", e))?;

    let mint_account = rpc
        .get_account(mint)
        .map_err(|e| format!("Failed to fetch mint: {}", e))?;
    let token_program = if mint_account.owner == TOKEN_2022_PROGRAM_ID {
        TOKEN_2022_PROGRAM_ID
    } else {
        SPL_TOKEN_PROGRAM_ID
    };

    let resolved = PumpFunResolvedData {
        mint: *mint,
        creator: curve_data.creator,
        token_program,
        fee_recipient: pick_fee_recipient(&global_data),
        is_cashback_coin: curve_data.is_cashback_coin,
    };

    let blockhash = rpc
        .get_latest_blockhash()
        .map_err(|e| format!("Failed to get blockhash: {}", e))?;

    build_buy_tx_cached(
        &resolved,
        wallet,
        token_amount,
        max_sol_cost,
        priority_fee,
        blockhash,
    )
}

pub fn build_sell_tx(
    rpc: &RpcClient,
    wallet: &Pubkey,
    mint: &Pubkey,
    token_amount: u64,
    min_sol_output: u64,
    priority_fee: u64,
) -> Result<Transaction, String> {
    let (global_pda, _) = Pubkey::find_program_address(&[GLOBAL_SEED], &PUMP_ID);
    let global_account = rpc
        .get_account(&global_pda)
        .map_err(|e| format!("Failed to fetch global: {}", e))?;
    let global_data = Global::from_bytes(&global_account.data)
        .map_err(|e| format!("Failed to deserialize global: {}", e))?;

    let (bonding_curve, _) =
        Pubkey::find_program_address(&[BONDING_CURVE_SEED, mint.as_ref()], &PUMP_ID);
    let bonding_curve_account = rpc
        .get_account(&bonding_curve)
        .map_err(|e| format!("Failed to fetch bonding curve: {}", e))?;
    let curve_data = BondingCurve::from_bytes(&bonding_curve_account.data)
        .map_err(|e| format!("Failed to deserialize bonding curve: {}", e))?;

    let mint_account = rpc
        .get_account(mint)
        .map_err(|e| format!("Failed to fetch mint: {}", e))?;
    let token_program = if mint_account.owner == TOKEN_2022_PROGRAM_ID {
        TOKEN_2022_PROGRAM_ID
    } else {
        SPL_TOKEN_PROGRAM_ID
    };

    let resolved = PumpFunResolvedData {
        mint: *mint,
        creator: curve_data.creator,
        token_program,
        fee_recipient: pick_fee_recipient(&global_data),
        is_cashback_coin: curve_data.is_cashback_coin,
    };

    let blockhash = rpc
        .get_latest_blockhash()
        .map_err(|e| format!("Failed to get blockhash: {}", e))?;

    build_sell_tx_cached(
        &resolved,
        wallet,
        token_amount,
        min_sol_output,
        priority_fee,
        blockhash,
    )
}
