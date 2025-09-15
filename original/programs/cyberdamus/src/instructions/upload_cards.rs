use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UploadCards<'info> {
    #[account(
        mut,
        seeds = [b"oracle_state"],
        bump = oracle_state.bump
    )]
    pub oracle_state: Account<'info, OracleState>,

    #[account(
        mut,
        seeds = [b"card_library"],
        bump = card_library.bump
    )]
    pub card_library: Account<'info, CardLibrary>,

    #[account(
        mut,
        constraint = authority.key() == oracle_state.authority @ ErrorCode::InvalidAuthority
    )]
    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<UploadCards>,
    start_index: u8,
    card_svgs: Vec<String>,
) -> Result<()> {
    let oracle_state = &mut ctx.accounts.oracle_state;
    let card_library = &mut ctx.accounts.card_library;

    // Validate batch size (max 10 cards per transaction to avoid size limits)
    require!(
        card_svgs.len() <= 10,
        ErrorCode::CardBatchTooLarge
    );

    // Validate start index
    require!(
        (start_index as usize) <= CardLibrary::MAX_CARDS,
        ErrorCode::InvalidCardId
    );

    // Ensure we don't exceed total card limit
    let end_index = start_index as usize + card_svgs.len();
    require!(
        end_index <= CardLibrary::MAX_CARDS,
        ErrorCode::CardBatchTooLarge
    );

    // If this is the first upload, initialize the vector with proper size
    if card_library.cards.is_empty() {
        card_library.cards = vec![String::new(); CardLibrary::MAX_CARDS];
    }

    // Upload cards starting from start_index
    for (i, svg) in card_svgs.iter().enumerate() {
        let card_index = start_index as usize + i;

        // Validate SVG size (max 2KB per card)
        require!(
            svg.len() <= 2048,
            ErrorCode::CardBatchTooLarge
        );

        card_library.cards[card_index] = svg.clone();

        msg!("📝 Card {} uploaded ({} bytes)", card_index, svg.len());
    }

    // Update metadata
    card_library.last_updated = Clock::get()?.unix_timestamp;

    // Check if all 78 cards are now uploaded
    let uploaded_cards = card_library.cards.iter()
        .filter(|card| !card.is_empty())
        .count();

    if uploaded_cards == CardLibrary::MAX_CARDS {
        oracle_state.card_library_initialized = true;
        msg!("🎉 All 78 cards uploaded! Oracle is ready for fortunes.");
    } else {
        msg!("📊 Progress: {}/{} cards uploaded", uploaded_cards, CardLibrary::MAX_CARDS);
    }

    msg!("🔮 Batch upload complete: {} cards uploaded starting from index {}",
         card_svgs.len(), start_index);

    Ok(())
}