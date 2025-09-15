# 🔮 CyberDamus - Decentralized Tarot Oracle on Solana

**The first fully on-chain Tarot oracle where each fortune becomes a unique NFT**

CyberDamus combines ancient Tarot wisdom with modern blockchain technology, creating collectible digital fortunes with authentic card-drawing mechanics.

## ✨ Features

- **🎴 Authentic Tarot**: Fisher-Yates shuffle ensures no duplicate cards per reading
- **🏪 On-Chain SVGs**: All 78 card designs stored permanently on Solana
- **💎 NFT Fortunes**: Each reading becomes a unique, tradeable NFT
- **⭐ Natural Rarity**: Rarity determined by card combinations (no manipulation)
- **⏰ Cooldown System**: Respects Tarot tradition with usage limits
- **🎯 Gas Efficient**: Optimized for Solana's low-cost transactions

## 🏗️ Architecture

### Smart Contract Storage
- **CardLibrary**: 78 SVG designs (~150KB total)
- **Fortune NFTs**: Store only 3 card IDs + metadata
- **User Records**: Track usage limits and cooldowns

### Card Drawing Algorithm
```rust
// Virtual deck - each card drawn ONCE per fortune
1. Create deck [0..77]
2. Generate entropy (timestamp + slot + user + counter)
3. Fisher-Yates shuffle with entropy seed
4. Draw first 3 cards (unique, no replacement)
```

### Rarity System (Natural Probabilities)
- **Legendary (2.1%)**: Three Major Arcana cards
- **Epic (5.2%)**: Sequential card numbers
- **Rare (14.8%)**: Same suit cards
- **Uncommon (25.3%)**: Two Major Arcana cards
- **Common (52.6%)**: All other combinations

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- Solana CLI 1.16+
- Anchor CLI 0.29+
- Node.js 18+
- Phantom/Solflare wallet

### Installation

1. **Clone & Setup**
   ```bash
   git clone https://github.com/cyberdamus/cyberdamus
   cd cyberdamus
   ```

2. **Build Smart Contract**
   ```bash
   anchor build
   anchor test --skip-local-validator
   ```

3. **Setup Client**
   ```bash
   cd client
   npm install
   npm run build
   ```

### Deployment to Devnet

1. **Configure Solana**
   ```bash
   solana config set --url devnet
   solana-keygen new --outfile ~/.config/solana/id.json
   solana airdrop 2
   ```

2. **Deploy Program**
   ```bash
   anchor deploy --provider.cluster devnet
   ```

3. **Update Program ID**
   - Copy deployed program ID
   - Update `PROGRAM_ID` in `client/src/client.ts`
   - Update `cyberdamus` in `Anchor.toml`

4. **Initialize Oracle**
   ```bash
   cd client
   npm run test
   ```

## 🧪 Testing

### Run Full Test Suite
```bash
cd client
npm run test
```

The test suite will:
1. ✅ Initialize oracle with 0.01 SOL fee
2. ✅ Upload all 78 cards in batches
3. ✅ Create first fortune (should work)
4. ✅ Test cooldown system (second fortune should fail)
5. ✅ Verify card uniqueness
6. ✅ Display fortune reading with ASCII art

### Expected Output
```
🔮 CYBERDAMUS TEST SUITE
==================================================
✅ Oracle initialization successful!
🎉 All 78 cards uploaded successfully!
🎉 First fortune created!

🔮 CYBERDAMUS FORTUNE READING
══════════════════════════════════════
Fortune #0
Created: 12/14/2024, 3:14:15 PM
Rarity: Epic

📖 YOUR READING:
🕐 PAST:    The Fool (0)
⚡ PRESENT: The Magician (1)
🌟 FUTURE:  The High Priestess (2)

🎴 CARD VISUALIZATION:
    ┌─────┐   ┌─────┐   ┌─────┐
    │  ⭐  │   │  ⭐  │   │  ⭐  │
    │  00  │   │  01  │   │  02  │
    └─────┘   └─────┘   └─────┘
     PAST     PRESENT    FUTURE

✅ Cooldown working correctly!
🎉 ALL TESTS COMPLETED SUCCESSFULLY!
```

## 📖 Usage Examples

### Initialize Oracle
```typescript
import { CyberDamusClient } from './client/src/client';

const client = new CyberDamusClient(connection, wallet);

// Initialize with 0.01 SOL fee
await client.initialize(treasuryWallet.publicKey, 0.01 * LAMPORTS_PER_SOL);
```

### Upload Card Library
```typescript
// Upload all 78 cards (Major + Minor Arcana)
await client.uploadCards();
```

### Create Fortune
```typescript
const fortune = await client.divineFortune();
console.log('Fortune:', fortune);
// Output: { fortuneId: 0, cards: [7, 21, 45], rarity: 'Legendary' }
```

### Display Fortune
```typescript
await client.displayFortune(0);
// Shows beautiful ASCII art + interpretation
```

## 🔧 Configuration

### Fee Settings (Changeable)
- **Minimum**: 0.001 SOL
- **Maximum**: 0.1 SOL
- **Test Default**: 0.01 SOL
- **Production**: 0.05 SOL

### User Limits (Philosophy-Based)
- **1st Fortune**: Instant (first insight is pure)
- **2nd Fortune**: 30-minute cooldown (time for reflection)
- **3rd Fortune**: 2-hour cooldown (deep contemplation)
- **4th+ Fortune**: 24-hour cooldown (respect the cosmos)

## 📊 Project Stats

### File Structure
```
CyberDamus/
├── programs/cyberdamus/src/
│   ├── lib.rs              # Main program entry
│   ├── state.rs            # Data structures
│   ├── utils.rs            # Fisher-Yates + helpers
│   ├── cards.rs            # SVG card designs
│   └── instructions/       # Instruction handlers
├── client/src/
│   ├── client.ts           # TypeScript client
│   ├── cards.ts            # Card data + utilities
│   └── test.ts             # Comprehensive tests
└── README.md               # This file
```

### Smart Contract Size
- **Program**: ~50KB compiled
- **Card Library**: ~150KB (78 × 2KB SVGs)
- **Per Fortune**: ~100 bytes (just 3 numbers + metadata)

## 🌟 Roadmap

### Phase 1: Genesis Launch ✅
- [x] Core smart contract
- [x] Fisher-Yates card drawing
- [x] 78 SVG card designs
- [x] TypeScript client
- [x] Comprehensive test suite

### Phase 2: Production (Next)
- [ ] Deploy to Solana mainnet
- [ ] Web interface (cyberdamus.io)
- [ ] Magic Eden integration
- [ ] Mobile app (React Native)
- [ ] Holiday card themes

### Phase 3: Expansion
- [ ] Extended spreads (Celtic Cross - 10 cards)
- [ ] Oracle Lenormand (36 cards)
- [ ] Multi-language support
- [ ] DAO governance for holders
- [ ] Secondary marketplace features

## 🛡️ Security

### Smart Contract Safety
- ✅ No admin privileges after initialization
- ✅ Fee capped at 0.1 SOL maximum
- ✅ Cooldown prevents spam attacks
- ✅ Entropy from multiple sources (timestamp + slot + user)
- ✅ All card draws verifiable on-chain

### Randomness Sources
1. Unix timestamp
2. Solana slot number
3. User public key
4. Global fortune counter
5. SHA-256 hash combination

### Audit Status
- [x] Internal code review
- [ ] Community bug bounty (planned)
- [ ] Professional audit (planned)

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open Pull Request

## 📜 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Website**: [cyberdamus.io](https://cyberdamus.io) (coming soon)
- **Discord**: [discord.gg/cyberdamus](https://discord.gg/cyberdamus)
- **Twitter**: [@CyberDamusOracle](https://twitter.com/CyberDamusOracle)
- **Documentation**: [docs.cyberdamus.io](https://docs.cyberdamus.io)

## 🎯 Support

Need help? Found a bug?

- 🐛 **Issues**: [GitHub Issues](https://github.com/cyberdamus/cyberdamus/issues)
- 💬 **Discord**: Get help in our community
- 📧 **Email**: oracle@cyberdamus.io

---

**"The future is already written in the blockchain, we just help you read it."**

*Built with 🔮 on Solana*