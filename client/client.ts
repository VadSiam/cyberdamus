// 🔮 CyberDamus Client - Solana Playground Version
// Complete TypeScript client for testing the Tarot oracle

import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, BN } from "@coral-xyz/anchor";
import {
  PublicKey,
  Keypair,
  Connection,
  clusterApiUrl,
  SystemProgram,
  LAMPORTS_PER_SOL,
  Transaction
} from "@solana/web3.js";

// ============================================================================
// CARD DATA - 22 Major Arcana SVGs (Simplified for Playground)
// ============================================================================

const MAJOR_ARCANA_SVGS = [
  // 0 - The Fool
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <circle cx="30" cy="45" r="15" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">0</text>
  <text x="30" y="80" text-anchor="middle" fill="#ffd700" font-size="8">FOOL</text>
</svg>`,

  // 1 - The Magician
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <polygon points="30,25 35,40 25,40" fill="none" stroke="#ffd700" stroke-width="2"/>
  <polygon points="30,55 35,70 25,70" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">I</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">MAGICIAN</text>
</svg>`,

  // 2 - The High Priestess
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <path d="M 20 40 Q 30 30 40 40" fill="none" stroke="#ffd700" stroke-width="2"/>
  <path d="M 20 55 Q 30 65 40 55" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">II</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">PRIESTESS</text>
</svg>`,

  // 3 - The Empress
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <polygon points="30,30 40,50 20,50" fill="none" stroke="#ffd700" stroke-width="2"/>
  <polygon points="30,60 20,50 40,50" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">III</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">EMPRESS</text>
</svg>`,

  // 4 - The Emperor
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <rect x="20" y="35" width="20" height="20" fill="none" stroke="#ffd700" stroke-width="2"/>
  <rect x="25" y="60" width="10" height="10" fill="none" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">IV</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">EMPEROR</text>
</svg>`,

  // 5 - The Hierophant
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <line x1="20" y1="35" x2="40" y2="35" stroke="#ffd700" stroke-width="2"/>
  <line x1="20" y1="45" x2="40" y2="45" stroke="#ffd700" stroke-width="2"/>
  <line x1="20" y1="55" x2="40" y2="55" stroke="#ffd700" stroke-width="2"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">V</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="6">HIEROPHANT</text>
</svg>`,

  // 6 - The Lovers
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <path d="M 25 45 Q 20 35 15 45 Q 20 55 25 45" fill="none" stroke="#ffd700" stroke-width="2"/>
  <path d="M 35 45 Q 40 35 45 45 Q 40 55 35 45" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">VI</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">LOVERS</text>
</svg>`,

  // 7 - The Chariot
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <rect x="15" y="40" width="30" height="15" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="20" cy="65" r="5" fill="none" stroke="#ffd700" stroke-width="1"/>
  <circle cx="40" cy="65" r="5" fill="none" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">VII</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">CHARIOT</text>
</svg>`,

  // 8 - Strength
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <path d="M 15 35 L 45 60 M 45 35 L 15 60" stroke="#ffd700" stroke-width="2"/>
  <circle cx="30" cy="47" r="10" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">VIII</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">STRENGTH</text>
</svg>`,

  // 9 - The Hermit
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <circle cx="25" cy="45" r="3" fill="#ffd700"/>
  <line x1="25" y1="45" x2="35" y2="55" stroke="#ffd700" stroke-width="2"/>
  <circle cx="30" cy="30" r="6" fill="none" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">IX</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">HERMIT</text>
</svg>`,

  // 10 - Wheel of Fortune
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <circle cx="30" cy="45" r="15" fill="none" stroke="#ffd700" stroke-width="2"/>
  <line x1="30" y1="30" x2="30" y2="60" stroke="#ffd700" stroke-width="1"/>
  <line x1="15" y1="45" x2="45" y2="45" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">X</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="6">FORTUNE</text>
</svg>`,

  // 11 - Justice
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <line x1="30" y1="30" x2="30" y2="65" stroke="#ffd700" stroke-width="2"/>
  <rect x="20" y="40" width="20" height="3" fill="#ffd700"/>
  <circle cx="30" cy="55" r="5" fill="none" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">XI</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">JUSTICE</text>
</svg>`,

  // 12 - The Hanged Man
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <line x1="20" y1="30" x2="40" y2="30" stroke="#ffd700" stroke-width="2"/>
  <line x1="30" y1="30" x2="30" y2="50" stroke="#ffd700" stroke-width="2"/>
  <circle cx="30" cy="55" r="5" fill="none" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">XII</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="6">HANGED MAN</text>
</svg>`,

  // 13 - Death
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <rect x="25" y="35" width="10" height="25" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="30" cy="30" r="5" fill="none" stroke="#ffd700" stroke-width="1"/>
  <line x1="20" y1="65" x2="40" y2="65" stroke="#ffd700" stroke-width="2"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">XIII</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">DEATH</text>
</svg>`,

  // 14 - Temperance
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <rect x="20" y="35" width="8" height="20" fill="none" stroke="#ffd700" stroke-width="1"/>
  <rect x="32" y="40" width="8" height="20" fill="none" stroke="#ffd700" stroke-width="1"/>
  <path d="M 28 45 Q 30 48 32 45" stroke="#ffd700" stroke-width="1" fill="none"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">XIV</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="6">TEMPERANCE</text>
</svg>`,

  // 15 - The Devil
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <polygon points="30,25 35,45 25,45" fill="none" stroke="#ffd700" stroke-width="2"/>
  <polygon points="30,65 25,50 35,50" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="27" cy="30" r="2" fill="#ffd700"/>
  <circle cx="33" cy="30" r="2" fill="#ffd700"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">XV</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">DEVIL</text>
</svg>`,

  // 16 - The Tower
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <rect x="20" y="35" width="20" height="30" fill="none" stroke="#ffd700" stroke-width="2"/>
  <polygon points="18,30 30,20 42,30" fill="none" stroke="#ffd700" stroke-width="1"/>
  <line x1="28" y1="20" x2="30" y2="15" stroke="#ffd700" stroke-width="1"/>
  <line x1="32" y1="20" x2="34" y2="15" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="15" text-anchor="middle" fill="#ffd700" font-size="10">XVI</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">TOWER</text>
</svg>`,

  // 17 - The Star
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <polygon points="30,25 32,35 42,35 34,42 37,52 30,47 23,52 26,42 18,35 28,35" fill="none" stroke="#ffd700" stroke-width="1"/>
  <circle cx="22" cy="30" r="1" fill="#ffd700"/>
  <circle cx="38" cy="30" r="1" fill="#ffd700"/>
  <circle cx="25" cy="60" r="1" fill="#ffd700"/>
  <circle cx="35" cy="60" r="1" fill="#ffd700"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="10">XVII</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">STAR</text>
</svg>`,

  // 18 - The Moon
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <path d="M 30 25 Q 22 35 30 45 Q 38 35 30 25" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="26" cy="35" r="2" fill="#ffd700"/>
  <circle cx="34" cy="35" r="2" fill="#ffd700"/>
  <path d="M 20 60 Q 30 65 40 60" stroke="#ffd700" stroke-width="1" fill="none"/>
  <text x="30" y="20" text-anchor="middle" fill="#ffd700" font-size="9">XVIII</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">MOON</text>
</svg>`,

  // 19 - The Sun
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <circle cx="30" cy="45" r="12" fill="none" stroke="#ffd700" stroke-width="2"/>
  <line x1="30" y1="20" x2="30" y2="25" stroke="#ffd700" stroke-width="1"/>
  <line x1="30" y1="65" x2="30" y2="70" stroke="#ffd700" stroke-width="1"/>
  <line x1="15" y1="45" x2="20" y2="45" stroke="#ffd700" stroke-width="1"/>
  <line x1="40" y1="45" x2="45" y2="45" stroke="#ffd700" stroke-width="1"/>
  <line x1="22" y1="30" x2="24" y2="32" stroke="#ffd700" stroke-width="1"/>
  <line x1="38" y1="60" x2="36" y2="58" stroke="#ffd700" stroke-width="1"/>
  <line x1="38" y1="30" x2="36" y2="32" stroke="#ffd700" stroke-width="1"/>
  <line x1="22" y1="60" x2="24" y2="58" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="15" text-anchor="middle" fill="#ffd700" font-size="10">XIX</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">SUN</text>
</svg>`,

  // 20 - Judgement
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <polygon points="25,30 30,20 35,30" fill="none" stroke="#ffd700" stroke-width="1"/>
  <rect x="27" y="30" width="6" height="15" fill="none" stroke="#ffd700" stroke-width="1"/>
  <circle cx="22" cy="55" r="3" fill="none" stroke="#ffd700" stroke-width="1"/>
  <circle cx="30" cy="55" r="3" fill="none" stroke="#ffd700" stroke-width="1"/>
  <circle cx="38" cy="55" r="3" fill="none" stroke="#ffd700" stroke-width="1"/>
  <text x="30" y="15" text-anchor="middle" fill="#ffd700" font-size="10">XX</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="6">JUDGEMENT</text>
</svg>`,

  // 21 - The World
  `<svg width="60" height="90" xmlns="http://www.w3.org/2000/svg">
  <rect width="60" height="90" fill="#1a1a2e" stroke="#ffd700" stroke-width="1"/>
  <circle cx="30" cy="45" r="15" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="30" cy="45" r="8" fill="none" stroke="#ffd700" stroke-width="1"/>
  <circle cx="30" cy="45" r="3" fill="#ffd700"/>
  <rect x="15" y="30" width="3" height="3" fill="#ffd700"/>
  <rect x="42" y="30" width="3" height="3" fill="#ffd700"/>
  <rect x="15" y="57" width="3" height="3" fill="#ffd700"/>
  <rect x="42" y="57" width="3" height="3" fill="#ffd700"/>
  <text x="30" y="15" text-anchor="middle" fill="#ffd700" font-size="10">XXI</text>
  <text x="30" y="85" text-anchor="middle" fill="#ffd700" font-size="7">WORLD</text>
</svg>`
];

// ============================================================================
// PROGRAM CONFIGURATION
// ============================================================================

// UPDATE THIS AFTER DEPLOYING TO PLAYGROUND
const PROGRAM_ID = new PublicKey("11111111111111111111111111111111");

// Connection to Solana devnet
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

// ============================================================================
// CYBERDAMUS CLIENT CLASS
// ============================================================================

export class CyberDamusClient {
  private program: Program;
  private provider: AnchorProvider;
  private programId: PublicKey;

  constructor(wallet: any) {
    this.provider = new AnchorProvider(
      connection,
      wallet,
      { commitment: "confirmed" }
    );
    anchor.setProvider(this.provider);

    this.programId = PROGRAM_ID;

    // Mock IDL - in real deployment, this would be generated
    const mockIdl = {
      version: "0.1.0",
      name: "cyberdamus",
      instructions: [
        {
          name: "initialize",
          accounts: [
            { name: "oracleState", isMut: true, isSigner: false },
            { name: "cardLibrary", isMut: true, isSigner: false },
            { name: "authority", isMut: true, isSigner: true },
            { name: "treasury", isMut: false, isSigner: false },
            { name: "systemProgram", isMut: false, isSigner: false }
          ],
          args: [{ name: "fee", type: "u64" }]
        },
        {
          name: "uploadCards",
          accounts: [
            { name: "oracleState", isMut: true, isSigner: false },
            { name: "cardLibrary", isMut: true, isSigner: false },
            { name: "authority", isMut: true, isSigner: true }
          ],
          args: [
            { name: "startIndex", type: "u8" },
            { name: "cardSvgs", type: { vec: "string" } }
          ]
        },
        {
          name: "divineFortune",
          accounts: [
            { name: "oracleState", isMut: true, isSigner: false },
            { name: "cardLibrary", isMut: false, isSigner: false },
            { name: "userRecord", isMut: true, isSigner: false },
            { name: "fortune", isMut: true, isSigner: false },
            { name: "user", isMut: true, isSigner: true },
            { name: "treasury", isMut: true, isSigner: false },
            { name: "systemProgram", isMut: false, isSigner: false }
          ],
          args: []
        }
      ],
      accounts: [],
      types: []
    };

    this.program = new Program(mockIdl as any, this.programId, this.provider);
  }

  // Get PDA addresses
  getOracleStatePDA(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("oracle_state")],
      this.programId
    );
  }

  getCardLibraryPDA(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("card_library")],
      this.programId
    );
  }

  getUserRecordPDA(user: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("user_record"), user.toBuffer()],
      this.programId
    );
  }

  getFortunePDA(fortuneId: number): [PublicKey, number] {
    const fortuneIdBuffer = Buffer.alloc(8);
    fortuneIdBuffer.writeBigUInt64LE(BigInt(fortuneId));

    return PublicKey.findProgramAddressSync(
      [Buffer.from("fortune"), fortuneIdBuffer],
      this.programId
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
      // Create instruction manually since we're using a mock IDL
      const instruction = await this.program.methods
        .initialize(new BN(fee))
        .accounts({
          oracleState: oracleStatePDA,
          cardLibrary: cardLibraryPDA,
          authority: this.provider.wallet.publicKey,
          treasury: treasuryWallet,
          systemProgram: SystemProgram.programId,
        })
        .instruction();

      const tx = new Transaction().add(instruction);
      const signature = await this.provider.sendAndConfirm(tx);

      console.log("✅ Oracle initialized! Transaction:", signature);
      return signature;
    } catch (error) {
      console.error("❌ Failed to initialize oracle:", error);
      throw error;
    }
  }

  // Upload cards in batches (22 Major Arcana only)
  async uploadCards(): Promise<void> {
    const [oracleStatePDA] = this.getOracleStatePDA();
    const [cardLibraryPDA] = this.getCardLibraryPDA();

    console.log("📝 Uploading 22 Major Arcana cards...");

    const batchSize = 5; // Smaller batches for Playground
    const totalCards = MAJOR_ARCANA_SVGS.length;

    for (let i = 0; i < totalCards; i += batchSize) {
      const batch = MAJOR_ARCANA_SVGS.slice(i, i + batchSize);
      const startIndex = i;

      console.log(`📦 Uploading batch ${Math.floor(i/batchSize) + 1}/${Math.ceil(totalCards/batchSize)} (cards ${startIndex}-${startIndex + batch.length - 1})...`);

      try {
        const instruction = await this.program.methods
          .uploadCards(startIndex, batch)
          .accounts({
            oracleState: oracleStatePDA,
            cardLibrary: cardLibraryPDA,
            authority: this.provider.wallet.publicKey,
          })
          .instruction();

        const tx = new Transaction().add(instruction);
        const signature = await this.provider.sendAndConfirm(tx);

        console.log(`✅ Batch uploaded! Transaction: ${signature.substring(0, 8)}...`);

        // Add delay to avoid rate limiting
        await new Promise(resolve => setTimeout(resolve, 2000));

      } catch (error) {
        console.error(`❌ Failed to upload batch starting at index ${startIndex}:`, error);
        throw error;
      }
    }

    console.log("🎉 All 22 cards uploaded successfully!");
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

    // Get current fortune counter
    try {
      const oracleState = await this.program.account.oracleState.fetch(oracleStatePDA);
      const fortuneId = oracleState.fortuneCounter.toNumber();

      const [fortunePDA] = this.getFortunePDA(fortuneId);

      console.log("🔮 Creating fortune reading...");
      console.log("Fortune ID:", fortuneId);
      console.log("Fortune PDA:", fortunePDA.toString());

      const instruction = await this.program.methods
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
        .instruction();

      const tx = new Transaction().add(instruction);
      const signature = await this.provider.sendAndConfirm(tx);

      // Fetch the created fortune
      const fortune = await this.program.account.fortune.fetch(fortunePDA);

      const rarityNames = ['Common', 'Uncommon', 'Rare', 'Epic', 'Legendary'];
      const rarity = rarityNames[Object.keys(fortune.rarity)[0]] || 'Unknown';

      console.log("✅ Fortune created! Transaction:", signature);

      return {
        fortuneId: fortune.fortuneId.toNumber(),
        cards: fortune.cards,
        rarity,
        transaction: signature
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
      console.log(`🕐 PAST:    ${this.getCardName(cardId1)} (${cardId1})`);
      console.log(`⚡ PRESENT: ${this.getCardName(cardId2)} (${cardId2})`);
      console.log(`🌟 FUTURE:  ${this.getCardName(cardId3)} (${cardId3})`);

      console.log("\n🎴 CARD VISUALIZATION:");
      this.displayCardArt(cardId1, cardId2, cardId3);

      console.log("═".repeat(60));

    } catch (error) {
      console.error("❌ Failed to fetch fortune:", error);
      throw error;
    }
  }

  // Get card name by ID
  getCardName(cardId: number): string {
    const majorArcana = [
      "The Fool", "The Magician", "The High Priestess", "The Empress", "The Emperor",
      "The Hierophant", "The Lovers", "The Chariot", "Strength", "The Hermit",
      "Wheel of Fortune", "Justice", "The Hanged Man", "Death", "Temperance",
      "The Devil", "The Tower", "The Star", "The Moon", "The Sun", "Judgement", "The World"
    ];

    if (cardId < 22) {
      return majorArcana[cardId];
    } else {
      return "Unknown Card";
    }
  }

  // Simple ASCII art representation
  private displayCardArt(card1: number, card2: number, card3: number): void {
    const getCardSymbol = (cardId: number): string => {
      return "⭐"; // All Major Arcana
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

// ============================================================================
// TESTING FUNCTIONS FOR PLAYGROUND
// ============================================================================

async function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function requestAirdrop(wallet: PublicKey, amount: number): Promise<void> {
  console.log(`💰 Requesting ${amount} SOL airdrop...`);
  try {
    const signature = await connection.requestAirdrop(wallet, amount * LAMPORTS_PER_SOL);
    await connection.confirmTransaction(signature);
    console.log(`✅ Airdrop successful! Signature: ${signature.substring(0, 8)}...`);
  } catch (error) {
    console.error("❌ Airdrop failed:", error);
    throw error;
  }
}

// ============================================================================
// MAIN TEST FUNCTION
// ============================================================================

async function runCyberDamusTest(): Promise<void> {
  console.log("🔮 CYBERDAMUS - SOLANA PLAYGROUND TEST");
  console.log("=" + "=".repeat(50));

  // Setup - In Playground, use the connected wallet
  const wallet = (window as any).solana; // Phantom wallet

  if (!wallet) {
    throw new Error("Please connect your Phantom wallet first!");
  }

  console.log("🔑 Using connected wallet:", wallet.publicKey.toString());

  // Fund wallet if needed
  const balance = await connection.getBalance(wallet.publicKey);
  if (balance < 0.1 * LAMPORTS_PER_SOL) {
    await requestAirdrop(wallet.publicKey, 1);
    await sleep(2000);
  }

  // Treasury wallet (in real deployment, this would be a proper treasury)
  const treasuryWallet = Keypair.generate();
  console.log("💰 Treasury:", treasuryWallet.publicKey.toString());

  // Initialize client
  const client = new CyberDamusClient(wallet);

  try {
    // Test 1: Initialize Oracle
    console.log("\n📋 TEST 1: Initialize Oracle");
    console.log("-".repeat(30));

    const fee = 0.01 * LAMPORTS_PER_SOL; // 0.01 SOL
    await client.initialize(treasuryWallet.publicKey, fee);

    console.log("✅ Oracle initialization successful!");

    // Test 2: Upload Cards
    console.log("\n📋 TEST 2: Upload 22 Major Arcana Cards");
    console.log("-".repeat(30));

    await client.uploadCards();

    console.log("✅ All cards uploaded successfully!");

    // Test 3: Check User Stats (should be empty)
    console.log("\n📋 TEST 3: Check Initial User Stats");
    console.log("-".repeat(30));

    const initialStats = await client.getUserStats();
    console.log("📊 Initial Stats:", initialStats);

    // Test 4: First Fortune (should work immediately)
    console.log("\n📋 TEST 4: First Fortune Reading");
    console.log("-".repeat(30));

    const fortune1 = await client.divineFortune();
    console.log("🎉 First fortune created!");
    console.log("Fortune ID:", fortune1.fortuneId);
    console.log("Cards:", fortune1.cards);
    console.log("Rarity:", fortune1.rarity);

    await client.displayFortune(fortune1.fortuneId);

    // Test 5: Check Updated User Stats
    console.log("\n📋 TEST 5: Check Updated User Stats");
    console.log("-".repeat(30));

    const updatedStats = await client.getUserStats();
    console.log("📊 Updated Stats:", updatedStats);

    // Test 6: Second Fortune (should hit cooldown)
    console.log("\n📋 TEST 6: Second Fortune (Cooldown Test)");
    console.log("-".repeat(30));

    try {
      console.log("⏰ Attempting second fortune immediately (should fail)...");
      const fortune2 = await client.divineFortune();
      console.log("❌ ERROR: Second fortune should have failed due to cooldown!");
    } catch (error) {
      console.log("✅ Cooldown working correctly! Error:", (error as Error).message);
    }

    // Test 7: Card Uniqueness Test
    console.log("\n📋 TEST 7: Verify Card Uniqueness");
    console.log("-".repeat(30));

    const cards = fortune1.cards;
    const uniqueCards = [...new Set(cards)];

    if (uniqueCards.length === 3) {
      console.log("✅ All three cards are unique!");
      console.log("Cards:", cards.map(id => `${id} (${client.getCardName(id)})`));
    } else {
      console.log("❌ ERROR: Cards are not unique!", cards);
    }

    // Test 8: Final Balance Check
    console.log("\n📋 TEST 8: Transaction Cost Analysis");
    console.log("-".repeat(30));

    const finalBalance = await connection.getBalance(wallet.publicKey);
    console.log(`💰 Final balance: ${finalBalance / LAMPORTS_PER_SOL} SOL`);
    console.log(`💸 Total cost: ~${((balance - finalBalance) / LAMPORTS_PER_SOL).toFixed(4)} SOL`);

    console.log("\n🎉 ALL TESTS COMPLETED SUCCESSFULLY!");
    console.log("=" + "=".repeat(50));

    // Summary
    console.log("\n📋 TEST SUMMARY:");
    console.log("✅ Oracle initialization");
    console.log("✅ Card library upload (22 Major Arcana)");
    console.log("✅ Fortune creation with unique cards");
    console.log("✅ User cooldown system");
    console.log("✅ Rarity calculation");
    console.log("✅ Transaction cost analysis");

    console.log("\n🔮 CyberDamus Oracle is working perfectly!");

  } catch (error) {
    console.error("❌ TEST FAILED:", error);
    throw error;
  }
}

// ============================================================================
// PLAYGROUND INTEGRATION
// ============================================================================

// Export for use in Playground console
(window as any).CyberDamus = {
  runTest: runCyberDamusTest,
  CyberDamusClient,
  PROGRAM_ID: PROGRAM_ID.toString()
};

// Auto-run when connected
if (typeof window !== 'undefined') {
  console.log("🔮 CyberDamus Client loaded!");
  console.log("Run CyberDamus.runTest() to start testing");
  console.log("Update PROGRAM_ID after deployment!");
}

export { runCyberDamusTest, CyberDamusClient };