import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  PublicKey,
  Keypair,
  Connection,
  clusterApiUrl,
  SystemProgram,
  LAMPORTS_PER_SOL
} from "@solana/web3.js";
import { ALL_TAROT_CARDS, getCardName } from "./cards";

// CyberDamus Program ID (from Anchor.toml)
const PROGRAM_ID = new PublicKey("CYBER1111111111111111111111111111111111111111");

// Devnet connection
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

// Test wallet (in production, this would be user's wallet)
const wallet = Keypair.generate();

export class CyberDamusClient {
  private program: Program<any>;
  private provider: anchor.AnchorProvider;

  constructor(connection: Connection, wallet: Keypair) {
    this.provider = new anchor.AnchorProvider(
      connection,
      new anchor.Wallet(wallet),
      { commitment: "confirmed" }
    );
    anchor.setProvider(this.provider);

    // In a real app, you'd load the IDL from the deployed program
    // For now, we'll use a mock IDL structure
    this.program = new Program({} as any, PROGRAM_ID, this.provider);
  }

  // Get PDA addresses
  getOracleStatePDA(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("oracle_state")],
      PROGRAM_ID
    );
  }

  getCardLibraryPDA(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("card_library")],
      PROGRAM_ID
    );
  }

  getUserRecordPDA(user: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("user_record"), user.toBuffer()],
      PROGRAM_ID
    );
  }

  getFortunePDA(fortuneId: number): [PublicKey, number] {
    const fortuneIdBuffer = Buffer.alloc(8);
    fortuneIdBuffer.writeBigUInt64LE(BigInt(fortuneId));

    return PublicKey.findProgramAddressSync(
      [Buffer.from("fortune"), fortuneIdBuffer],
      PROGRAM_ID
    );
  }

  // Initialize the oracle
  async initialize(treasuryWallet: PublicKey, fee: number): Promise<string> {
    const [oracleStatePDA] = this.getOracleStatePDA();
    const [cardLibraryPDA] = this.getCardLibraryPDA();

    console.log("🔮 Initializing CyberDamus Oracle...");
    console.log("Oracle State PDA:", oracleStatePDA.toString());
    console.log("Card Library PDA:", cardLibraryPDA.toString());
    console.log("Fee:", `${fee / LAMPORTS_PER_SOL} SOL`);

    try {
      const tx = await this.program.methods
        .initialize(new anchor.BN(fee))
        .accounts({
          oracleState: oracleStatePDA,
          cardLibrary: cardLibraryPDA,
          authority: this.provider.wallet.publicKey,
          treasury: treasuryWallet,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("✅ Oracle initialized! Transaction:", tx);
      return tx;
    } catch (error) {
      console.error("❌ Failed to initialize oracle:", error);
      throw error;
    }
  }

  // Upload cards in batches
  async uploadCards(): Promise<void> {
    const [oracleStatePDA] = this.getOracleStatePDA();
    const [cardLibraryPDA] = this.getCardLibraryPDA();

    console.log("📝 Uploading 78 Tarot cards...");

    const batchSize = 8; // Upload 8 cards per transaction
    const totalCards = ALL_TAROT_CARDS.length;

    for (let i = 0; i < totalCards; i += batchSize) {
      const batch = ALL_TAROT_CARDS.slice(i, i + batchSize);
      const startIndex = i;

      console.log(`📦 Uploading batch ${Math.floor(i/batchSize) + 1}/${Math.ceil(totalCards/batchSize)} (cards ${startIndex}-${startIndex + batch.length - 1})...`);

      try {
        const tx = await this.program.methods
          .uploadCards(startIndex, batch)
          .accounts({
            oracleState: oracleStatePDA,
            cardLibrary: cardLibraryPDA,
            authority: this.provider.wallet.publicKey,
          })
          .rpc();

        console.log(`✅ Batch uploaded! Transaction: ${tx.substring(0, 8)}...`);

        // Add delay to avoid rate limiting
        await new Promise(resolve => setTimeout(resolve, 1000));

      } catch (error) {
        console.error(`❌ Failed to upload batch starting at index ${startIndex}:`, error);
        throw error;
      }
    }

    console.log("🎉 All 78 cards uploaded successfully!");
  }

  // Create a fortune reading
  async divineFortune(): Promise<{
    fortuneId: number;
    cards: number[];
    rarity: string;
    transaction: string;
  }> {
    const [oracleStatePDA] = this.getOracleStatePDA();
    const [cardLibraryPDA] = this.getCardLibraryPDA();
    const [userRecordPDA] = this.getUserRecordPDA(this.provider.wallet.publicKey);

    // Get current fortune counter to determine fortune ID
    const oracleState = await this.program.account.oracleState.fetch(oracleStatePDA);
    const fortuneId = oracleState.fortuneCounter.toNumber();

    const [fortunePDA] = this.getFortunePDA(fortuneId);

    console.log("🔮 Creating fortune reading...");
    console.log("Fortune ID:", fortuneId);
    console.log("Fortune PDA:", fortunePDA.toString());

    try {
      const tx = await this.program.methods
        .divineFortune()
        .accounts({
          oracleState: oracleStatePDA,
          cardLibrary: cardLibraryPDA,
          userRecord: userRecordPDA,
          fortune: fortunePDA,
          user: this.provider.wallet.publicKey,
          treasury: oracleState.treasury,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      // Fetch the created fortune
      const fortune = await this.program.account.fortune.fetch(fortunePDA);

      const rarityNames = ['Common', 'Uncommon', 'Rare', 'Epic', 'Legendary'];
      const rarity = rarityNames[Object.keys(fortune.rarity)[0]] || 'Unknown';

      console.log("✅ Fortune created! Transaction:", tx);

      return {
        fortuneId: fortune.fortuneId.toNumber(),
        cards: fortune.cards,
        rarity,
        transaction: tx
      };

    } catch (error) {
      console.error("❌ Failed to create fortune:", error);
      throw error;
    }
  }

  // Display fortune reading
  async displayFortune(fortuneId: number): Promise<void> {
    const [fortunePDA] = this.getFortunePDA(fortuneId);

    try {
      const fortune = await this.program.account.fortune.fetch(fortunePDA);
      const [cardId1, cardId2, cardId3] = fortune.cards;

      console.log("\n" + "═".repeat(60));
      console.log("🔮 CYBERDAMUS FORTUNE READING");
      console.log("═".repeat(60));
      console.log(`Fortune #${fortune.fortuneId.toNumber()}`);
      console.log(`Created: ${new Date(fortune.timestamp.toNumber() * 1000).toLocaleString()}`);
      console.log(`Owner: ${fortune.owner.toString()}`);

      const rarityNames = ['Common', 'Uncommon', 'Rare', 'Epic', 'Legendary'];
      const rarity = rarityNames[Object.keys(fortune.rarity)[0]] || 'Unknown';
      console.log(`Rarity: ${rarity}`);

      console.log("\n📖 YOUR READING:");
      console.log(`🕐 PAST:    ${getCardName(cardId1)} (${cardId1})`);
      console.log(`⚡ PRESENT: ${getCardName(cardId2)} (${cardId2})`);
      console.log(`🌟 FUTURE:  ${getCardName(cardId3)} (${cardId3})`);

      console.log("\n🎴 CARD VISUALIZATION:");
      this.displayCardArt(cardId1, cardId2, cardId3);

      console.log("═".repeat(60));

    } catch (error) {
      console.error("❌ Failed to fetch fortune:", error);
      throw error;
    }
  }

  // Simple ASCII art representation
  private displayCardArt(card1: number, card2: number, card3: number): void {
    const getCardSymbol = (cardId: number): string => {
      if (cardId < 22) return "⭐"; // Major Arcana
      if (cardId < 36) return "⚡"; // Wands
      if (cardId < 50) return "♡"; // Cups
      if (cardId < 64) return "⚔"; // Swords
      return "◈"; // Pentacles
    };

    console.log(`    ┌─────┐   ┌─────┐   ┌─────┐`);
    console.log(`    │  ${getCardSymbol(card1)}  │   │  ${getCardSymbol(card2)}  │   │  ${getCardSymbol(card3)}  │`);
    console.log(`    │  ${card1.toString().padStart(2)}  │   │  ${card2.toString().padStart(2)}  │   │  ${card3.toString().padStart(2)}  │`);
    console.log(`    └─────┘   └─────┘   └─────┘`);
    console.log(`     PAST     PRESENT    FUTURE`);
  }

  // Get user statistics
  async getUserStats(): Promise<{
    totalFortunes: number;
    lastFortune: Date | null;
    cooldownUntil: Date | null;
    dailyCount: number;
  }> {
    const [userRecordPDA] = this.getUserRecordPDA(this.provider.wallet.publicKey);

    try {
      const userRecord = await this.program.account.userRecord.fetch(userRecordPDA);

      return {
        totalFortunes: userRecord.totalFortunes.toNumber(),
        lastFortune: userRecord.lastFortuneTimestamp.toNumber() > 0
          ? new Date(userRecord.lastFortuneTimestamp.toNumber() * 1000)
          : null,
        cooldownUntil: userRecord.cooldownUntil.toNumber() > Date.now() / 1000
          ? new Date(userRecord.cooldownUntil.toNumber() * 1000)
          : null,
        dailyCount: userRecord.dailyCount
      };
    } catch (error) {
      // User record doesn't exist yet
      return {
        totalFortunes: 0,
        lastFortune: null,
        cooldownUntil: null,
        dailyCount: 0
      };
    }
  }
}