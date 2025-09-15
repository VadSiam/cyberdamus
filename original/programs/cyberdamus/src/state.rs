use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RarityTier {
    Common,      // 52.6% - Mixed cards
    Uncommon,    // 25.3% - Two Major Arcana
    Rare,        // 14.8% - Same suit cards
    Epic,        // 5.2% - Sequential numbers
    Legendary,   // 2.1% - Three Major Arcana
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
    pub cards: Vec<String>,         // 78 SVG strings
    pub version: u8,                // Design version (1 = classic, 2 = christmas, etc)
    pub last_updated: i64,          // Timestamp of last update
    pub bump: u8,                   // PDA bump
}

impl CardLibrary {
    pub const MAX_CARDS: usize = 78;
    // Rough calculation: 78 cards * 2KB per SVG = ~156KB + overhead
    pub const LEN: usize = 8 + 4 + (78 * 2048) + 1 + 8 + 1; // ~159KB max
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