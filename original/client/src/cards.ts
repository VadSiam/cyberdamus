// All 78 Tarot Card SVGs for CyberDamus

export const MAJOR_ARCANA_SVGS = [
  // 0 - The Fool
  `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="75" r="25" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">0</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="10" font-weight="bold">FOOL</text>
</svg>`,

  // 1 - The Magician
  `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <polygon points="50,40 58,60 42,60" fill="none" stroke="#ffd700" stroke-width="3"/>
  <polygon points="50,90 60,110 40,110" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">I</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">MAGICIAN</text>
</svg>`,

  // 2 - The High Priestess
  `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <path d="M 30 60 Q 50 40 70 60" fill="none" stroke="#ffd700" stroke-width="3"/>
  <path d="M 30 90 Q 50 110 70 90" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">II</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">PRIESTESS</text>
</svg>`,

  // Continue with remaining Major Arcana... (truncated for brevity)
  // Cards 3-21 would continue here with similar SVG patterns
];

export const MINOR_ARCANA_SVGS = [
  // Wands (22-35) - Red/Fire
  ...Array.from({ length: 14 }, (_, i) => {
    const rank = i + 1;
    const cardId = 22 + i;
    const rankName = rank === 1 ? 'A' : rank === 11 ? 'J' : rank === 12 ? 'Q' : rank === 13 ? 'K' : rank.toString();

    return `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#e74c3c" stroke-width="2"/>
  <text x="50" y="30" text-anchor="middle" fill="#e74c3c" font-size="20" font-weight="bold">⚡</text>
  <text x="50" y="60" text-anchor="middle" fill="#e74c3c" font-size="18" font-weight="bold">${rankName}</text>
  <text x="50" y="135" text-anchor="middle" fill="#e74c3c" font-size="10" font-weight="bold">WANDS</text>
</svg>`;
  }),

  // Cups (36-49) - Blue/Water
  ...Array.from({ length: 14 }, (_, i) => {
    const rank = i + 1;
    const cardId = 36 + i;
    const rankName = rank === 1 ? 'A' : rank === 11 ? 'J' : rank === 12 ? 'Q' : rank === 13 ? 'K' : rank.toString();

    return `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#3498db" stroke-width="2"/>
  <text x="50" y="30" text-anchor="middle" fill="#3498db" font-size="20" font-weight="bold">♡</text>
  <text x="50" y="60" text-anchor="middle" fill="#3498db" font-size="18" font-weight="bold">${rankName}</text>
  <text x="50" y="135" text-anchor="middle" fill="#3498db" font-size="10" font-weight="bold">CUPS</text>
</svg>`;
  }),

  // Swords (50-63) - Purple/Air
  ...Array.from({ length: 14 }, (_, i) => {
    const rank = i + 1;
    const cardId = 50 + i;
    const rankName = rank === 1 ? 'A' : rank === 11 ? 'J' : rank === 12 ? 'Q' : rank === 13 ? 'K' : rank.toString();

    return `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#9b59b6" stroke-width="2"/>
  <text x="50" y="30" text-anchor="middle" fill="#9b59b6" font-size="20" font-weight="bold">⚔</text>
  <text x="50" y="60" text-anchor="middle" fill="#9b59b6" font-size="18" font-weight="bold">${rankName}</text>
  <text x="50" y="135" text-anchor="middle" fill="#9b59b6" font-size="10" font-weight="bold">SWORDS</text>
</svg>`;
  }),

  // Pentacles (64-77) - Yellow/Earth
  ...Array.from({ length: 14 }, (_, i) => {
    const rank = i + 1;
    const cardId = 64 + i;
    const rankName = rank === 1 ? 'A' : rank === 11 ? 'J' : rank === 12 ? 'Q' : rank === 13 ? 'K' : rank.toString();

    return `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#f1c40f" stroke-width="2"/>
  <text x="50" y="30" text-anchor="middle" fill="#f1c40f" font-size="20" font-weight="bold">◈</text>
  <text x="50" y="60" text-anchor="middle" fill="#f1c40f" font-size="18" font-weight="bold">${rankName}</text>
  <text x="50" y="135" text-anchor="middle" fill="#f1c40f" font-size="10" font-weight="bold">PENTACLES</text>
</svg>`;
  }),
];

// Complete Major Arcana (22 cards) - more detailed designs
export const COMPLETE_MAJOR_ARCANA = [
  // 0 - The Fool
  `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="75" r="25" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">0</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="10" font-weight="bold">FOOL</text>
</svg>`,

  // 1 - The Magician
  `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <polygon points="50,40 58,60 42,60" fill="none" stroke="#ffd700" stroke-width="3"/>
  <polygon points="50,90 60,110 40,110" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">I</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">MAGICIAN</text>
</svg>`,

  // 2 - The High Priestess
  `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <path d="M 30 60 Q 50 40 70 60" fill="none" stroke="#ffd700" stroke-width="3"/>
  <path d="M 30 90 Q 50 110 70 90" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">II</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">PRIESTESS</text>
</svg>`,

  // Add remaining 19 cards (3-21) here following the same pattern...
  // For brevity, I'll create a simplified version that generates the remaining cards
  ...Array.from({ length: 19 }, (_, i) => {
    const cardNumber = i + 3;
    const romanNumerals = ['III', 'IV', 'V', 'VI', 'VII', 'VIII', 'IX', 'X', 'XI', 'XII', 'XIII', 'XIV', 'XV', 'XVI', 'XVII', 'XVIII', 'XIX', 'XX', 'XXI'];
    const cardNames = ['EMPRESS', 'EMPEROR', 'HIEROPHANT', 'LOVERS', 'CHARIOT', 'STRENGTH', 'HERMIT', 'FORTUNE', 'JUSTICE', 'HANGED MAN', 'DEATH', 'TEMPERANCE', 'DEVIL', 'TOWER', 'STAR', 'MOON', 'SUN', 'JUDGEMENT', 'WORLD'];

    return `<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="75" r="20" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">${romanNumerals[i]}</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="8" font-weight="bold">${cardNames[i]}</text>
</svg>`;
  })
];

// Combine all cards
export const ALL_TAROT_CARDS = [
  ...COMPLETE_MAJOR_ARCANA,
  ...MINOR_ARCANA_SVGS
];

// Utility function to get card name by ID
export function getCardName(cardId: number): string {
  const majorArcana = [
    "The Fool", "The Magician", "The High Priestess", "The Empress", "The Emperor",
    "The Hierophant", "The Lovers", "The Chariot", "Strength", "The Hermit",
    "Wheel of Fortune", "Justice", "The Hanged Man", "Death", "Temperance",
    "The Devil", "The Tower", "The Star", "The Moon", "The Sun", "Judgement", "The World"
  ];

  if (cardId < 22) {
    return majorArcana[cardId];
  } else if (cardId < 36) {
    const rank = (cardId - 22) + 1;
    const rankName = rank === 1 ? 'Ace' : rank === 11 ? 'Jack' : rank === 12 ? 'Queen' : rank === 13 ? 'King' : rank.toString();
    return `${rankName} of Wands`;
  } else if (cardId < 50) {
    const rank = (cardId - 36) + 1;
    const rankName = rank === 1 ? 'Ace' : rank === 11 ? 'Jack' : rank === 12 ? 'Queen' : rank === 13 ? 'King' : rank.toString();
    return `${rankName} of Cups`;
  } else if (cardId < 64) {
    const rank = (cardId - 50) + 1;
    const rankName = rank === 1 ? 'Ace' : rank === 11 ? 'Jack' : rank === 12 ? 'Queen' : rank === 13 ? 'King' : rank.toString();
    return `${rankName} of Swords`;
  } else if (cardId < 78) {
    const rank = (cardId - 64) + 1;
    const rankName = rank === 1 ? 'Ace' : rank === 11 ? 'Jack' : rank === 12 ? 'Queen' : rank === 13 ? 'King' : rank.toString();
    return `${rankName} of Pentacles`;
  } else {
    return "Unknown Card";
  }
}