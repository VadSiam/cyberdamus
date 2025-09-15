# 🔮 CyberDamus - Solana Playground Implementation

**Decentralized Tarot Oracle on Solana - Full 78-Card Deck**

## Project Overview

CyberDamus is a decentralized Tarot oracle built on Solana that provides authentic fortune readings using all 78 Tarot cards. The implementation uses Fisher-Yates shuffle algorithm to ensure no duplicate cards per reading and stores lightweight SVGs on-chain.

## 🏗️ Architecture

### Smart Contract (`src/lib.rs`)
- **Language**: Rust with Anchor framework
- **Cards**: Full 78-card Tarot deck (22 Major + 56 Minor Arcana)
- **Storage**: ~32KB for all card SVGs (400 bytes each)
- **Shuffle**: Fisher-Yates algorithm for authentic randomness
- **Cooldown**: Progressive timing (30min → 2hr → 24hr)

### Client (`client/client.ts`)
- **Language**: TypeScript for Solana Playground
- **Cards**: 78 SVGs (22 hand-crafted Major + 56 generated Minor Arcana)
- **Upload**: Batch system (8 cards per transaction)
- **Display**: Local SVGs for immediate visualization

## 📊 Card Implementation

### Major Arcana (0-21)
Hand-crafted SVGs with symbolic designs:
- 60x90px dimensions for lightweight storage
- Unique visual elements for each card
- Golden color scheme (#ffd700) on dark background

### Minor Arcana (22-77)
Programmatically generated SVGs:
- **Wands (22-35)**: Fire/Red (#e74c3c) with ⚡ symbol
- **Cups (36-49)**: Water/Blue (#3498db) with ♡ symbol
- **Swords (50-63)**: Air/Purple (#9b59b6) with ⚔ symbol
- **Pentacles (64-77)**: Earth/Yellow (#f1c40f) with ◈ symbol

## 🎯 Rarity System

Natural probability-based rarity calculation:

- **Legendary (2.1%)**: All 3 Major Arcana cards
- **Epic (5.2%)**: Sequential numbers or all even/odd
- **Rare (14.8%)**: Same suit cards (Minor Arcana only)
- **Uncommon (25.3%)**: Two Major Arcana cards
- **Common (52.6%)**: Everything else

## 🔧 Key Features

### Authentic Tarot Mechanics
- Fisher-Yates shuffle ensures no duplicates
- Three-card spread: Past, Present, Future
- True randomness from multiple entropy sources

### User Protection
- Progressive cooldown system prevents spam
- 1st fortune: Instant
- 2nd fortune: 30-minute cooldown
- 3rd fortune: 2-hour cooldown
- 4th+ fortune: 24-hour cooldown

### On-Chain Storage
- SVGs stored permanently in smart contract
- NFTs contain only 3 card IDs (lightweight)
- Total storage: ~31KB (vs 156KB with full-size SVGs)

## 🚀 Deployment Structure

### For Solana Playground:
```
CyberDamus/
├── Anchor.toml          # Playground configuration
├── Cargo.toml           # Rust dependencies
├── client/
│   └── client.ts        # Complete 78-card client
└── src/
    └── lib.rs           # Smart contract
```

### Deployment Steps:
1. Push to GitHub
2. Import to beta.solpg.io via GitHub
3. Build & Deploy
4. Update PROGRAM_ID in client.ts
5. Run tests via browser console

## 💾 Technical Specifications

### Contract Constraints
```rust
MAX_CARDS = 78              // Full Tarot deck
MAX_BATCH_SIZE = 10         // Cards per upload transaction
MAX_SVG_SIZE = 500          // Bytes per card (playground)
MIN_FEE = 0.001 SOL        // Minimum fortune fee
MAX_FEE = 0.1 SOL          // Maximum fortune fee
```

### Storage Optimization
- Major Arcana: ~300 bytes each
- Minor Arcana: ~280 bytes each
- Total deck: ~22KB SVG data
- CardLibrary PDA: ~32KB max size

## 🎮 Usage Example

```typescript
// Initialize client
const client = new CyberDamusClient(window.solana);

// Initialize oracle
await client.initializeOracle(0.01 * LAMPORTS_PER_SOL);

// Upload all 78 cards
await client.uploadCards();

// Create fortune
const fortune = await client.divineFortune();

// Display result
await client.displayFortune(fortune.fortuneId);
```

## 🔬 Testing

### Playground Testing:
```javascript
// Run full test suite
CyberDamus.runTest()

// Check card count
console.log("Cards:", CyberDamus.cardCount); // 78

// Manual fortune creation
const client = new CyberDamus.CyberDamusClient(window.solana);
await client.divineFortune();
```

## 📈 Recent Updates

### Version History:
- **v1.0**: Original 22-card Major Arcana only
- **v2.0**: Upgraded to full 78-card deck
- **v2.1**: Code cleanup and optimization
- **v2.2**: Removed all unused legacy files

### Code Cleanup (Latest):
- Removed unused `original/` folder with legacy `cards.rs`
- Cleaned duplicate exports and unused variables
- Optimized import statements
- Maintained only deployment-essential files

## 🎯 Production Considerations

### Security:
- Multi-source entropy generation
- Cooldown prevents manipulation
- Fee collection to treasury wallet

### Scalability:
- Batch upload system for large datasets
- PDA-based account architecture
- Efficient SVG compression

### User Experience:
- Instant local card visualization
- Progressive web app compatibility
- Mobile-responsive design

## 🔗 Resources

- **Solana Playground**: https://beta.solpg.io/
- **Anchor Framework**: https://anchor-lang.com/
- **Tarot Card Meanings**: Standard Rider-Waite symbolism

---

**Ready for decentralized fortune telling! 🔮**

*"The cards reveal what the blockchain remembers..."*