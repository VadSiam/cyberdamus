use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};
use crate::state::RarityTier;

pub fn generate_entropy_seed(
    user: &Pubkey,
    timestamp: i64,
    slot: u64,
    fortune_counter: u64,
) -> [u8; 32] {
    let mut hasher = Sha256::new();

    // Combine all entropy sources
    hasher.update(user.to_bytes());
    hasher.update(timestamp.to_le_bytes());
    hasher.update(slot.to_le_bytes());
    hasher.update(fortune_counter.to_le_bytes());

    // Add some randomness from the clock
    let clock = Clock::get()?;
    hasher.update(clock.slot.to_le_bytes());

    hasher.finalize().into()
}

pub fn fisher_yates_draw_three(seed: [u8; 32]) -> [u8; 3] {
    // Create virtual deck [0, 1, 2, ..., 77]
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

        // Minor Arcana - simplified naming for now
        22..=35 => "Wands",      // Cards 22-35
        36..=49 => "Cups",       // Cards 36-49
        50..=63 => "Swords",     // Cards 50-63
        64..=77 => "Pentacles",  // Cards 64-77

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fisher_yates_uniqueness() {
        let seed = [1u8; 32];
        let cards = fisher_yates_draw_three(seed);

        // Ensure all three cards are unique
        assert_ne!(cards[0], cards[1]);
        assert_ne!(cards[1], cards[2]);
        assert_ne!(cards[0], cards[2]);

        // Ensure all cards are in valid range
        assert!(cards[0] < 78);
        assert!(cards[1] < 78);
        assert!(cards[2] < 78);
    }

    #[test]
    fn test_rarity_calculation() {
        // Test Legendary (all Major Arcana)
        assert_eq!(calculate_rarity([0, 10, 21]), RarityTier::Legendary);

        // Test Uncommon (two Major Arcana)
        assert_eq!(calculate_rarity([0, 10, 50]), RarityTier::Uncommon);

        // Test Sequential (Epic)
        assert_eq!(calculate_rarity([5, 6, 7]), RarityTier::Epic);

        // Test Common
        assert_eq!(calculate_rarity([25, 40, 65]), RarityTier::Common);
    }
}