# 🔮 CyberDamus - Solana Playground Deployment Guide

**Deploy CyberDamus Tarot Oracle directly in your browser using Solana Playground**

## 🚀 Quick Start (5 Minutes)

### Step 1: Open Solana Playground
1. Go to **https://beta.solpg.io/**
2. **Connect your wallet** (Phantom/Solflare)
3. **Switch to Devnet** (bottom left)

### Step 2: Create New Project
1. Click **"Create a new project"**
2. Choose **"Anchor (Rust)"** template ✅
3. Name it **"CyberDamus"**

### Step 3: Copy Files to Playground

#### Copy the Smart Contract:
1. Open `src/lib.rs` in Playground
2. **Delete all content**
3. **Copy & paste** entire content from `playground/src/lib.rs`

#### Copy the Client:
1. Open `client/client.ts` in Playground
2. **Delete all content**
3. **Copy & paste** entire content from `playground/client/client.ts`

#### Update Anchor.toml:
1. Open `Anchor.toml` in Playground
2. **Replace content** with content from `playground/Anchor.toml`

### Step 4: Build & Deploy
1. Click **"Build"** button (wait for compilation)
2. Click **"Deploy"** button
3. **Copy the Program ID** from output (looks like: `CYBER...`)

### Step 5: Update Program ID
1. In `client/client.ts`, find line:
   ```typescript
   const PROGRAM_ID = new PublicKey("11111111111111111111111111111111");
   ```
2. **Replace** with your deployed Program ID:
   ```typescript
   const PROGRAM_ID = new PublicKey("YOUR_ACTUAL_PROGRAM_ID_HERE");
   ```

### Step 6: Run Tests
1. Open **browser console** (F12)
2. Run the test:
   ```javascript
   CyberDamus.runTest()
   ```

## 📋 Expected Output

```
🔮 CYBERDAMUS - SOLANA PLAYGROUND TEST
==================================================
🔑 Using connected wallet: 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
💰 Requesting 1 SOL airdrop...
✅ Airdrop successful!

📋 TEST 1: Initialize Oracle
------------------------------
🔮 Initializing CyberDamus Oracle...
Oracle State PDA: 5xKJ8...
Card Library PDA: 8mF2...
Fee: 0.01 SOL
✅ Oracle initialization successful!

📋 TEST 2: Upload 22 Major Arcana Cards
------------------------------
📝 Uploading 22 Major Arcana cards...
📦 Uploading batch 1/5 (cards 0-4)...
✅ Batch uploaded! Transaction: A7bF3...
📦 Uploading batch 2/5 (cards 5-9)...
✅ Batch uploaded! Transaction: B8cG4...
[... continues for all batches ...]
🎉 All 22 cards uploaded successfully!

📋 TEST 4: First Fortune Reading
------------------------------
🔮 Creating fortune reading...
Fortune ID: 0
🎉 First fortune created!
Fortune ID: 0
Cards: [7, 15, 2]
Rarity: Uncommon

🔮 CYBERDAMUS FORTUNE READING
══════════════════════════════════════
Fortune #0
Created: 12/14/2024, 3:14:15 PM
Rarity: Uncommon

📖 YOUR READING:
🕐 PAST:    The Chariot (7)
⚡ PRESENT: The Devil (15)
🌟 FUTURE:  The High Priestess (2)

🎴 CARD VISUALIZATION:
    ┌─────┐   ┌─────┐   ┌─────┐
    │  ⭐  │   │  ⭐  │   │  ⭐  │
    │  07  │   │  15  │   │  02  │
    └─────┘   └─────┘   └─────┘
     PAST     PRESENT    FUTURE

📋 TEST 6: Second Fortune (Cooldown Test)
------------------------------
⏰ Attempting second fortune immediately (should fail)...
✅ Cooldown working correctly! Error: Cooldown active. 1800 seconds remaining

🎉 ALL TESTS COMPLETED SUCCESSFULLY!
```

## 🔧 Key Features Tested

### ✅ **Authentic Tarot Mechanics**
- Fisher-Yates shuffle ensures no duplicate cards
- 22 Major Arcana cards (0-21)
- Unique card combinations every time

### ✅ **Rarity System (Major Arcana Only)**
- **Legendary**: 0 (Fool) + 10 (Fortune) + 20 (Judgement) - "The Journey"
- **Epic**: All even or all odd numbers
- **Rare**: Three sequential numbers (e.g., 5-6-7)
- **Uncommon**: Two sequential numbers
- **Common**: Everything else

### ✅ **User Protection**
- 1st fortune: Instant
- 2nd fortune: 30-minute cooldown
- 3rd fortune: 2-hour cooldown
- 4th+ fortune: 24-hour cooldown

### ✅ **On-Chain Storage**
- SVGs stored permanently in smart contract
- NFTs contain only 3 card IDs (lightweight)
- ~11KB total storage (vs 156KB in full version)

## 🎯 Manual Testing Options

### Test Individual Functions:
```javascript
// Initialize only
const client = new CyberDamus.CyberDamusClient(window.solana);

// Check user stats
const stats = await client.getUserStats();
console.log("User stats:", stats);

// Get specific fortune
await client.displayFortune(0);

// Check program ID
console.log("Program ID:", CyberDamus.PROGRAM_ID);
```

### Check Account States:
```javascript
// Get oracle state
const [oracleStatePDA] = client.getOracleStatePDA();
console.log("Oracle State PDA:", oracleStatePDA.toString());

// Get card library
const [cardLibraryPDA] = client.getCardLibraryPDA();
console.log("Card Library PDA:", cardLibraryPDA.toString());
```

## 🐛 Troubleshooting

### ❌ **Build Errors**
- **Check Rust syntax** in `lib.rs`
- **Verify all dependencies** in Cargo.toml
- **Make sure Anchor version** is 0.29.0

### ❌ **Deploy Errors**
- **Check wallet balance** (need SOL for deployment)
- **Switch to devnet** if on localnet
- **Try smaller contract** if too large

### ❌ **Client Errors**
- **Update Program ID** after deployment
- **Connect wallet** before running tests
- **Check console** for detailed error messages

### ❌ **Transaction Errors**
- **Insufficient funds**: Request airdrop
- **Account not found**: Run initialization first
- **Cooldown active**: Wait for cooldown period

## 📊 Size Comparison

| Component | Full Version | Playground Version | Reduction |
|-----------|-------------|-------------------|-----------|
| Cards | 78 (156KB) | 22 (11KB) | 93% smaller |
| SVG Size | 2KB each | 300B each | 85% smaller |
| Program | ~800 lines | ~650 lines | 19% smaller |
| Client | Multiple files | Single file | Simplified |

## 🎮 Next Steps

1. **Test different card combinations** to see various rarities
2. **Try multiple users** with different wallets
3. **Experiment with cooldown system** timing
4. **Check transaction costs** and optimize
5. **Add more features** to the full version

## 🔗 Links

- **Full Project**: See main repository for complete 78-card version
- **Solana Playground**: https://beta.solpg.io/
- **Anchor Docs**: https://anchor-lang.com/
- **Solana Docs**: https://docs.solana.com/

---

**Ready to deploy your decentralized Tarot oracle! 🔮**

*The cards have been shuffled, the code is ready, destiny awaits...*