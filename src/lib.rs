// 🔮 CyberDamus - Solana Playground Version
// Decentralized Tarot Oracle on Solana - All-in-One File

use anchor_lang::prelude::*;
use anchor_lang::system_program;
use sha2::{Digest, Sha256};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod cyberdamus {
    use super::*;

    /// Initialize the CyberDamus oracle with configuration
    pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
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

        // Initialize empty card library (for 78 cards)
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

    /// Upload card SVGs in batches (78 full Tarot deck)
    pub fn upload_cards(
        ctx: Context<UploadCards>,
        start_index: u8,
        card_svgs: Vec<String>,
    ) -> Result<()> {
        let oracle_state = &mut ctx.accounts.oracle_state;
        let card_library = &mut ctx.accounts.card_library;

        // Validate batch size (max 10 cards per transaction)
        require!(card_svgs.len() <= 10, ErrorCode::CardBatchTooLarge);

        // Validate start index (78 cards max for playground)
        require!((start_index as usize) <= CardLibrary::MAX_CARDS, ErrorCode::InvalidCardId);

        // Ensure we don't exceed total card limit
        let end_index = start_index as usize + card_svgs.len();
        require!(end_index <= CardLibrary::MAX_CARDS, ErrorCode::CardBatchTooLarge);

        // If this is the first upload, initialize the vector with proper size
        if card_library.cards.is_empty() {
            card_library.cards = vec![String::new(); CardLibrary::MAX_CARDS];
        }

        // Upload cards starting from start_index
        for (i, svg) in card_svgs.iter().enumerate() {
            let card_index = start_index as usize + i;

            // Validate SVG size (max 500 bytes per card for playground)
            require!(svg.len() <= 500, ErrorCode::CardBatchTooLarge);

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

    /// Create a new fortune reading (main function)
    pub fn divine_fortune(ctx: Context<DivineFortune>) -> Result<()> {
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

        // Use Fisher-Yates algorithm to draw 3 unique cards (from full 78-card deck)
        let drawn_cards = fisher_yates_draw_three(entropy_seed);

        // Calculate rarity (full deck)
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
            9 => msg!("🎯 10th Fortune! Building momentum!"),
            99 => msg!("🏆 100th Fortune! A milestone achievement!"),
            _ => {}
        }

        Ok(())
    }

    /// Get fortune by ID (view function)
    pub fn get_fortune(ctx: Context<GetFortune>, _fortune_id: u64) -> Result<Fortune> {
        Ok(ctx.accounts.fortune.clone())
    }

    /// Get card SVG by ID (view function)
    pub fn get_card_svg(ctx: Context<GetCardSvg>, card_id: u8) -> Result<String> {
        let card_library = &ctx.accounts.card_library;

        require!(
            (card_id as usize) < card_library.cards.len(),
            ErrorCode::InvalidCardId
        );

        Ok(card_library.cards[card_id as usize].clone())
    }
}

// ============================================================================
// STATE STRUCTURES
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RarityTier {
    Common,      // 52.6% - Mixed cards
    Uncommon,    // 25.3% - Two sequential
    Rare,        // 14.8% - Three sequential
    Epic,        // 5.2% - All even or odd
    Legendary,   // 2.1% - 0, 10, 20 (special Major Arcana)
}

impl Default for RarityTier {
    fn default() -> Self {
        RarityTier::Common
    }
}

#[account]
pub struct OracleState {
    pub authority: Pubkey,          // Admin authority
    pub treasury: Pubkey,           // Fee collection wallet
    pub fee: u64,                   // Fee in lamports (0.01 SOL = 10_000_000)
    pub fortune_counter: u64,       // Global counter for fortune numbering
    pub card_library_initialized: bool, // Whether cards are uploaded
    pub bump: u8,                   // PDA bump
}

impl OracleState {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1 + 1; // discriminator + fields
}

#[account]
pub struct CardLibrary {
    pub cards: Vec<String>,         // 22 SVG strings (Major Arcana only)
    pub version: u8,                // Design version (1 = classic, 2 = christmas, etc)
    pub last_updated: i64,          // Timestamp of last update
    pub bump: u8,                   // PDA bump
}

impl CardLibrary {
    pub const MAX_CARDS: usize = 78; // Full Tarot deck for playground
    // Calculation: 78 cards * 400 bytes per SVG = ~31KB + overhead
    pub const LEN: usize = 8 + 4 + (78 * 400) + 1 + 8 + 1; // ~32KB max
}

#[account]
pub struct Fortune {
    pub fortune_id: u64,            // Unique fortune number
    pub owner: Pubkey,              // Current owner (transferable NFT)
    pub cards: [u8; 3],             // Three card IDs [Past, Present, Future]
    pub timestamp: i64,             // When fortune was created
    pub block_height: u64,          // Block when created (for entropy proof)
    pub rarity: RarityTier,         // Calculated rarity
    pub entropy_seed: [u8; 32],     // Seed used for generation (proof of randomness)
    pub bump: u8,                   // PDA bump
}

impl Fortune {
    pub const LEN: usize = 8 + 8 + 32 + 3 + 8 + 8 + 1 + 32 + 1; // discriminator + fields
}

#[account]
pub struct UserRecord {
    pub user: Pubkey,               // User's wallet address
    pub last_fortune_timestamp: i64, // When last fortune was created
    pub daily_count: u8,            // How many fortunes today
    pub last_reset_day: u32,        // Day counter for reset (Unix days)
    pub cooldown_until: i64,        // Timestamp when next fortune is allowed
    pub total_fortunes: u64,        // Lifetime total (for achievements)
    pub bump: u8,                   // PDA bump
}

impl UserRecord {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 4 + 8 + 8 + 1; // discriminator + fields

    pub fn reset_daily_if_needed(&mut self, current_time: i64) {
        let current_day = (current_time / 86400) as u32; // Unix days
        if current_day > self.last_reset_day {
            self.daily_count = 0;
            self.last_reset_day = current_day;
        }
    }

    pub fn can_create_fortune(&self, current_time: i64) -> Result<()> {
        if current_time < self.cooldown_until {
            let remaining = self.cooldown_until - current_time;
            msg!("⏰ Cooldown active. {} seconds remaining", remaining);
            return Err(error!(ErrorCode::CooldownActive));
        }
        Ok(())
    }

    pub fn update_after_fortune(&mut self, current_time: i64) {
        self.last_fortune_timestamp = current_time;
        self.total_fortunes += 1;

        // Update daily count (already reset if needed)
        self.daily_count += 1;

        // Calculate next cooldown based on daily count
        self.cooldown_until = current_time + match self.daily_count {
            1 => 0,          // 1st fortune: no cooldown
            2 => 30 * 60,    // 2nd fortune: 30 minutes
            3 => 2 * 3600,   // 3rd fortune: 2 hours
            _ => 24 * 3600,  // 4th+ fortune: 24 hours
        };
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

pub fn generate_entropy_seed(
    user: &Pubkey,
    timestamp: i64,
    slot: u64,
    fortune_counter: u64,
) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();

    // Combine all entropy sources
    hasher.update(user.to_bytes());
    hasher.update(timestamp.to_le_bytes());
    hasher.update(slot.to_le_bytes());
    hasher.update(fortune_counter.to_le_bytes());

    // Add some randomness from the clock
    let clock = Clock::get()?;
    hasher.update(clock.slot.to_le_bytes());

    Ok(hasher.finalize().into())
}

pub fn fisher_yates_draw_three(seed: [u8; 32]) -> [u8; 3] {
    // Create virtual deck [0, 1, 2, ..., 77] - Full Tarot deck
    let mut deck: Vec<u8> = (0..78).collect();

    // Use seed to create deterministic pseudo-random sequence
    let mut rng_state = u64::from_le_bytes([
        seed[0], seed[1], seed[2], seed[3],
        seed[4], seed[5], seed[6], seed[7],
    ]);

    // Fisher-Yates shuffle - only need to shuffle first 3 positions
    for i in (1..=2).rev() {
        // Generate pseudo-random number using LCG (Linear Congruential Generator)
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let j = (rng_state % ((i + 1) as u64)) as usize;
        deck.swap(i, j);
    }

    // For the first position, shuffle with entire remaining deck
    for i in (3..78).rev() {
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let j = (rng_state % ((i + 1) as u64)) as usize;
        deck.swap(i, j);
    }

    // Final shuffle for position 0
    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
    let j = (rng_state % 78) as usize;
    deck.swap(0, j);

    [deck[0], deck[1], deck[2]]
}

pub fn calculate_rarity(cards: [u8; 3]) -> RarityTier {
    // Count Major Arcana cards (0-21)
    let major_count = cards.iter().filter(|&&card| card < 22).count();

    // Check for three Major Arcana (Legendary)
    if major_count == 3 {
        return RarityTier::Legendary; // 2.1% chance
    }

    // Check for sequential numbers (Epic)
    if is_sequential(cards) {
        return RarityTier::Epic; // 5.2% chance
    }

    // Check for same suit (Rare) - only applies to Minor Arcana
    if same_suit_minor_arcana(cards) {
        return RarityTier::Rare; // 14.8% chance
    }

    // Check for two Major Arcana (Uncommon)
    if major_count == 2 {
        return RarityTier::Uncommon; // 25.3% chance
    }

    // Everything else is Common
    RarityTier::Common // 52.6% chance
}

fn is_sequential(cards: [u8; 3]) -> bool {
    let mut sorted_cards = cards;
    sorted_cards.sort();

    // Check if they form a sequence (difference of 1 between adjacent cards)
    sorted_cards[1] == sorted_cards[0] + 1 && sorted_cards[2] == sorted_cards[1] + 1
}

fn same_suit_minor_arcana(cards: [u8; 3]) -> bool {
    // Filter out Major Arcana (0-21), only check Minor Arcana (22-77)
    let minor_cards: Vec<u8> = cards.iter()
        .filter(|&&card| card >= 22)
        .copied()
        .collect();

    // Need at least 2 minor arcana cards to determine same suit
    if minor_cards.len() < 2 {
        return false;
    }

    // Minor Arcana structure: 22-77 (56 cards)
    // 4 suits × 14 cards each = 56 cards
    // Card 22-35: Wands, 36-49: Cups, 50-63: Swords, 64-77: Pentacles
    let get_suit = |card: u8| -> u8 {
        if card < 22 { return 255; } // Major Arcana, no suit
        (card - 22) / 14 // 0=Wands, 1=Cups, 2=Swords, 3=Pentacles
    };

    let first_suit = get_suit(minor_cards[0]);
    minor_cards.iter().all(|&card| get_suit(card) == first_suit)
}

pub fn get_card_name(card_id: u8) -> &'static str {
    match card_id {
        // Major Arcana (0-21)
        0 => "The Fool",
        1 => "The Magician",
        2 => "The High Priestess",
        3 => "The Empress",
        4 => "The Emperor",
        5 => "The Hierophant",
        6 => "The Lovers",
        7 => "The Chariot",
        8 => "Strength",
        9 => "The Hermit",
        10 => "Wheel of Fortune",
        11 => "Justice",
        12 => "The Hanged Man",
        13 => "Death",
        14 => "Temperance",
        15 => "The Devil",
        16 => "The Tower",
        17 => "The Star",
        18 => "The Moon",
        19 => "The Sun",
        20 => "Judgement",
        21 => "The World",

        // Minor Arcana - Wands (22-35)
        22..=35 => {
            let rank = (card_id - 22) + 1;
            match rank {
                1 => "Ace of Wands",
                11 => "Page of Wands",
                12 => "Knight of Wands",
                13 => "Queen of Wands",
                14 => "King of Wands",
                _ => "Wands",
            }
        },

        // Minor Arcana - Cups (36-49)
        36..=49 => {
            let rank = (card_id - 36) + 1;
            match rank {
                1 => "Ace of Cups",
                11 => "Page of Cups",
                12 => "Knight of Cups",
                13 => "Queen of Cups",
                14 => "King of Cups",
                _ => "Cups",
            }
        },

        // Minor Arcana - Swords (50-63)
        50..=63 => {
            let rank = (card_id - 50) + 1;
            match rank {
                1 => "Ace of Swords",
                11 => "Page of Swords",
                12 => "Knight of Swords",
                13 => "Queen of Swords",
                14 => "King of Swords",
                _ => "Swords",
            }
        },

        // Minor Arcana - Pentacles (64-77)
        64..=77 => {
            let rank = (card_id - 64) + 1;
            match rank {
                1 => "Ace of Pentacles",
                11 => "Page of Pentacles",
                12 => "Knight of Pentacles",
                13 => "Queen of Pentacles",
                14 => "King of Pentacles",
                _ => "Pentacles",
            }
        },

        _ => "Unknown Card"
    }
}

pub fn format_fortune_reading(cards: [u8; 3], rarity: RarityTier) -> String {
    let past = get_card_name(cards[0]);
    let present = get_card_name(cards[1]);
    let future = get_card_name(cards[2]);

    let rarity_str = match rarity {
        RarityTier::Common => "Common",
        RarityTier::Uncommon => "Uncommon",
        RarityTier::Rare => "Rare",
        RarityTier::Epic => "Epic",
        RarityTier::Legendary => "Legendary ⭐",
    };

    format!(
        "🔮 Fortune Reading [{}]\n📍 Past: {} ({})\n⚡ Present: {} ({})\n🌟 Future: {} ({})",
        rarity_str,
        past, cards[0],
        present, cards[1],
        future, cards[2]
    )
}

// ============================================================================
// ACCOUNT STRUCTURES
// ============================================================================

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

#[derive(Accounts)]
#[instruction(fortune_id: u64)]
pub struct GetFortune<'info> {
    #[account(
        seeds = [b"fortune", fortune_id.to_le_bytes().as_ref()],
        bump = fortune.bump
    )]
    pub fortune: Account<'info, Fortune>,
}

#[derive(Accounts)]
#[instruction(card_id: u8)]
pub struct GetCardSvg<'info> {
    #[account(
        seeds = [b"card_library"],
        bump = card_library.bump
    )]
    pub card_library: Account<'info, CardLibrary>,
}

// ============================================================================
// ERROR CODES
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Fortune creation is on cooldown")]
    CooldownActive,
    #[msg("Daily fortune limit exceeded")]
    DailyLimitExceeded,
    #[msg("Card library not initialized")]
    CardLibraryNotInitialized,
    #[msg("Invalid card ID")]
    InvalidCardId,
    #[msg("Card batch too large")]
    CardBatchTooLarge,
    #[msg("Insufficient funds for fortune fee")]
    InsufficientFunds,
    #[msg("Invalid authority")]
    InvalidAuthority,
}