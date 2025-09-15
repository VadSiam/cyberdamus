# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## CyberDamus - Decentralized Tarot Oracle

CyberDamus is a Solana-based smart contract that creates an authentic Tarot oracle where each fortune reading becomes a unique NFT. The core innovation is storing 78 SVG card designs on-chain and using Fisher-Yates shuffle algorithm to ensure no duplicate cards per reading.

## Development Commands

### Smart Contract (Anchor/Rust)
```bash
# Build the smart contract
anchor build

# Test on localnet
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Check program logs
solana logs --url devnet
```

### TypeScript Client
```bash
cd client
npm install
npm run build     # Compile TypeScript
npm run test      # Run full test suite
npm run dev       # Run client directly
```

### Testing Individual Components
```bash
# Test smart contract only
anchor test --skip-local-validator

# Test specific client functionality
cd client && npx ts-node src/client.ts

# Run single test scenario
cd client && npx ts-node -e "import('./src/test').then(t => t.runFullTestSuite())"
```

## Core Architecture

### Smart Contract Structure (`programs/cyberdamus/src/`)

**Critical Data Flow**: Oracle initialization → Card library upload → Fortune creation → NFT with 3 card IDs

1. **`state.rs`** - Core data structures:
   - `OracleState`: Global config (fee, counter, treasury)
   - `CardLibrary`: Stores all 78 SVG strings on-chain (~150KB)
   - `Fortune`: NFT data (just 3 card IDs + metadata)
   - `UserRecord`: Cooldown/limit tracking per user

2. **`utils.rs`** - Critical algorithms:
   - `fisher_yates_draw_three()`: Ensures 3 unique cards (0-77 range)
   - `generate_entropy_seed()`: Multi-source randomness (timestamp + slot + user + counter)
   - `calculate_rarity()`: Natural probability-based rarity (Legendary to Common)

3. **`instructions/`** - Main program functions:
   - `initialize`: Setup oracle with fee and treasury
   - `upload_cards`: Batch upload SVGs (8-10 per transaction due to size limits)
   - `divine_fortune`: Main fortune creation with cooldown enforcement

4. **`cards.rs`** - SVG asset generation for all 78 Tarot cards

### Key Constraints & Design Decisions

**Storage Model**: SVGs stored once in contract, NFTs only store 3 numbers (card IDs). This saves massive storage costs vs. storing SVGs in each NFT.

**Fisher-Yates Implementation**: Custom deterministic shuffle using LCG (Linear Congruential Generator) with entropy seed. Ensures each card can only appear once per reading.

**User Limits**: Philosophy-based cooldown system:
- 1st fortune: Instant
- 2nd fortune: 30-minute cooldown
- 3rd fortune: 2-hour cooldown
- 4th+ fortune: 24-hour cooldown

**PDA Seeds**:
- Oracle State: `[b"oracle_state"]`
- Card Library: `[b"card_library"]`
- User Record: `[b"user_record", user.key()]`
- Fortune: `[b"fortune", fortune_id.to_le_bytes()]`

### Client Architecture (`client/src/`)

**`client.ts`**: Main CyberDamusClient class with methods for all contract interactions. Uses Anchor framework for type-safe contract calls.

**`cards.ts`**: Complete 78-card SVG library matching contract storage. Includes utility functions for card naming and display.

**`test.ts`**: Comprehensive test suite that validates:
- Oracle initialization
- Card upload in batches
- Fortune creation with uniqueness
- Cooldown enforcement
- Cost analysis

## Important Program Constants

```rust
// Card library constraints
CardLibrary::MAX_CARDS = 78
CardLibrary::LEN = ~159KB (includes SVG storage)

// Fee limits (enforced in contract)
MIN_FEE = 1_000_000 lamports (0.001 SOL)
MAX_FEE = 100_000_000 lamports (0.1 SOL)

// Upload constraints
MAX_BATCH_SIZE = 10 cards per transaction
MAX_SVG_SIZE = 2048 bytes per card
```

## Development Notes

**Transaction Size Limits**: Card uploads must be batched due to Solana transaction size limits. The upload_cards instruction handles this with start_index parameter.

**Entropy Sources**: Uses multiple randomness sources to prevent manipulation: Unix timestamp, Solana slot, user pubkey, global counter. All combined via SHA-256.

**Rarity Calculation**: Based on natural probability of card combinations, not artificial manipulation:
- Legendary (2.1%): All 3 Major Arcana
- Epic (5.2%): Sequential numbers
- Rare (14.8%): Same suit cards
- Uncommon (25.3%): Two Major Arcana
- Common (52.6%): Everything else

**Program ID**: Currently set to placeholder `CYBER1111111111111111111111111111111111111111` - must be updated after deployment.

## Client Configuration

The TypeScript client requires updating the PROGRAM_ID constant in `client/src/client.ts` after deploying the smart contract. The client handles wallet connection, transaction creation, and result parsing automatically.