use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct DivineFortune<'info> {
    #[account(
        mut,
        seeds = [b"oracle_state"],
        bump = oracle_state.bump
    )]
    pub oracle_state: Account<'info, OracleState>,

    #[account(
        seeds = [b"card_library"],
        bump = card_library.bump
    )]
    pub card_library: Account<'info, CardLibrary>,

    #[account(
        init_if_needed,
        payer = user,
        space = UserRecord::LEN,
        seeds = [b"user_record", user.key().as_ref()],
        bump
    )]
    pub user_record: Account<'info, UserRecord>,

    #[account(
        init,
        payer = user,
        space = Fortune::LEN,
        seeds = [b"fortune", oracle_state.fortune_counter.to_le_bytes().as_ref()],
        bump
    )]
    pub fortune: Account<'info, Fortune>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Treasury wallet for fee collection
    #[account(
        mut,
        constraint = treasury.key() == oracle_state.treasury @ ErrorCode::InvalidAuthority
    )]
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DivineFortune>) -> Result<()> {
    let oracle_state = &mut ctx.accounts.oracle_state;
    let user_record = &mut ctx.accounts.user_record;
    let fortune = &mut ctx.accounts.fortune;

    // Verify card library is initialized
    require!(
        oracle_state.card_library_initialized,
        ErrorCode::CardLibraryNotInitialized
    );

    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    // Initialize user record if this is the first time
    if user_record.user == Pubkey::default() {
        user_record.user = ctx.accounts.user.key();
        user_record.last_fortune_timestamp = 0;
        user_record.daily_count = 0;
        user_record.last_reset_day = 0;
        user_record.cooldown_until = 0;
        user_record.total_fortunes = 0;
        user_record.bump = ctx.bumps.user_record;

        msg!("🆕 New user record created for {}", user_record.user);
    }

    // Reset daily count if needed
    user_record.reset_daily_if_needed(current_time);

    // Check cooldown
    user_record.can_create_fortune(current_time)?;

    // Generate entropy seed from multiple sources
    let entropy_seed = generate_entropy_seed(
        &ctx.accounts.user.key(),
        current_time,
        clock.slot,
        oracle_state.fortune_counter,
    )?;

    // Use Fisher-Yates algorithm to draw 3 unique cards
    let drawn_cards = fisher_yates_draw_three(entropy_seed);

    // Calculate rarity
    let rarity = calculate_rarity(drawn_cards);

    // Create the fortune NFT
    fortune.fortune_id = oracle_state.fortune_counter;
    fortune.owner = ctx.accounts.user.key();
    fortune.cards = drawn_cards;
    fortune.timestamp = current_time;
    fortune.block_height = clock.slot;
    fortune.rarity = rarity;
    fortune.entropy_seed = entropy_seed;
    fortune.bump = ctx.bumps.fortune;

    // Transfer fee to treasury
    let fee = oracle_state.fee;
    if fee > 0 {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                },
            ),
            fee,
        )?;

        msg!("💰 Fee transferred: {} lamports", fee);
    }

    // Update counters
    oracle_state.fortune_counter += 1;
    user_record.update_after_fortune(current_time);

    // Log the fortune
    let reading = format_fortune_reading(drawn_cards, rarity);
    msg!("🔮 Fortune #{} created!", fortune.fortune_id);
    msg!("{}", reading);
    msg!("⏰ Next fortune available in: {} seconds",
         user_record.cooldown_until.saturating_sub(current_time));

    // Special messages for milestones
    match fortune.fortune_id {
        0 => msg!("🎉 GENESIS FORTUNE! The very first CyberDamus reading!"),
        99 => msg!("🎯 100th Fortune! You're part of the early legends!"),
        999 => msg!("🏆 1000th Fortune! A milestone achievement!"),
        _ => {}
    }

    Ok(())
}