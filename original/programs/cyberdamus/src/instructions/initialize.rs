use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = OracleState::LEN,
        seeds = [b"oracle_state"],
        bump
    )]
    pub oracle_state: Account<'info, OracleState>,

    #[account(
        init,
        payer = authority,
        space = CardLibrary::LEN,
        seeds = [b"card_library"],
        bump
    )]
    pub card_library: Account<'info, CardLibrary>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Treasury wallet for fee collection
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    fee: u64,
) -> Result<()> {
    let oracle_state = &mut ctx.accounts.oracle_state;
    let card_library = &mut ctx.accounts.card_library;

    // Validate fee (between 0.001 SOL and 0.1 SOL)
    let min_fee = 1_000_000;      // 0.001 SOL
    let max_fee = 100_000_000;    // 0.1 SOL

    require!(
        fee >= min_fee && fee <= max_fee,
        ErrorCode::InsufficientFunds
    );

    // Initialize oracle state
    oracle_state.authority = ctx.accounts.authority.key();
    oracle_state.treasury = ctx.accounts.treasury.key();
    oracle_state.fee = fee;
    oracle_state.fortune_counter = 0;
    oracle_state.card_library_initialized = false;
    oracle_state.bump = ctx.bumps.oracle_state;

    // Initialize empty card library
    card_library.cards = Vec::new();
    card_library.version = 1; // v1 = classic design
    card_library.last_updated = Clock::get()?.unix_timestamp;
    card_library.bump = ctx.bumps.card_library;

    msg!("🔮 CyberDamus Oracle initialized!");
    msg!("Authority: {}", oracle_state.authority);
    msg!("Treasury: {}", oracle_state.treasury);
    msg!("Fee: {} lamports ({:.3} SOL)", fee, fee as f64 / 1_000_000_000.0);

    Ok(())
}