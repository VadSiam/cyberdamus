use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod utils;
pub mod cards;

use instructions::*;

declare_id!("CYBER1111111111111111111111111111111111111111");

#[program]
pub mod cyberdamus {
    use super::*;

    /// Initialize the CyberDamus oracle with configuration
    pub fn initialize(
        ctx: Context<Initialize>,
        fee: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, fee)
    }

    /// Upload card SVGs in batches (max 10 per transaction)
    pub fn upload_cards(
        ctx: Context<UploadCards>,
        start_index: u8,
        card_svgs: Vec<String>,
    ) -> Result<()> {
        instructions::upload_cards::handler(ctx, start_index, card_svgs)
    }

    /// Create a new fortune reading (main function)
    pub fn divine_fortune(ctx: Context<DivineFortune>) -> Result<()> {
        instructions::divine_fortune::handler(ctx)
    }

    /// Get fortune by ID (view function)
    pub fn get_fortune(ctx: Context<GetFortune>, _fortune_id: u64) -> Result<state::Fortune> {
        Ok(ctx.accounts.fortune.clone())
    }

    /// Get card SVG by ID (view function)
    pub fn get_card_svg(ctx: Context<GetCardSvg>, card_id: u8) -> Result<String> {
        let card_library = &ctx.accounts.card_library;

        require!(
            (card_id as usize) < card_library.cards.len(),
            state::ErrorCode::InvalidCardId
        );

        Ok(card_library.cards[card_id as usize].clone())
    }
}

#[derive(Accounts)]
#[instruction(fortune_id: u64)]
pub struct GetFortune<'info> {
    #[account(
        seeds = [b"fortune", fortune_id.to_le_bytes().as_ref()],
        bump = fortune.bump
    )]
    pub fortune: Account<'info, state::Fortune>,
}

#[derive(Accounts)]
#[instruction(card_id: u8)]
pub struct GetCardSvg<'info> {
    #[account(
        seeds = [b"card_library"],
        bump = card_library.bump
    )]
    pub card_library: Account<'info, state::CardLibrary>,
}