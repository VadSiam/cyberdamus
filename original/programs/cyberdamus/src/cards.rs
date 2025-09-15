use anchor_lang::prelude::*;

pub fn get_major_arcana_svgs() -> Vec<String> {
    vec![
        // 0 - The Fool
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="75" r="25" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">0</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="10" font-weight="bold">FOOL</text>
</svg>"#.to_string(),

        // 1 - The Magician
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <polygon points="50,40 58,60 42,60" fill="none" stroke="#ffd700" stroke-width="3"/>
  <polygon points="50,90 60,110 40,110" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">I</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">MAGICIAN</text>
</svg>"#.to_string(),

        // 2 - The High Priestess
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <path d="M 30 60 Q 50 40 70 60" fill="none" stroke="#ffd700" stroke-width="3"/>
  <path d="M 30 90 Q 50 110 70 90" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">II</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">PRIESTESS</text>
</svg>"#.to_string(),

        // 3 - The Empress
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <polygon points="50,45 65,75 35,75" fill="none" stroke="#ffd700" stroke-width="3"/>
  <polygon points="50,95 35,75 65,75" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">III</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">EMPRESS</text>
</svg>"#.to_string(),

        // 4 - The Emperor
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <rect x="35" y="50" width="30" height="30" fill="none" stroke="#ffd700" stroke-width="3"/>
  <rect x="42" y="85" width="16" height="16" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">IV</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">EMPEROR</text>
</svg>"#.to_string(),

        // 5 - The Hierophant
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <line x1="30" y1="50" x2="70" y2="50" stroke="#ffd700" stroke-width="3"/>
  <line x1="30" y1="70" x2="70" y2="70" stroke="#ffd700" stroke-width="3"/>
  <line x1="30" y1="90" x2="70" y2="90" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">V</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="8" font-weight="bold">HIEROPHANT</text>
</svg>"#.to_string(),

        // 6 - The Lovers
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <path d="M 40 65 Q 30 50 25 65 Q 30 80 40 65" fill="none" stroke="#ffd700" stroke-width="3"/>
  <path d="M 60 65 Q 70 50 75 65 Q 70 80 60 65" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">VI</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">LOVERS</text>
</svg>"#.to_string(),

        // 7 - The Chariot
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <rect x="25" y="60" width="50" height="25" fill="none" stroke="#ffd700" stroke-width="3"/>
  <circle cx="35" cy="95" r="8" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="65" cy="95" r="8" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">VII</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">CHARIOT</text>
</svg>"#.to_string(),

        // 8 - Strength
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <path d="M 25 50 L 75 100 M 75 50 L 25 100" stroke="#ffd700" stroke-width="4"/>
  <circle cx="50" cy="75" r="15" fill="none" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">VIII</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">STRENGTH</text>
</svg>"#.to_string(),

        // 9 - The Hermit
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <circle cx="45" cy="65" r="4" fill="#ffd700"/>
  <line x1="45" y1="65" x2="65" y2="85" stroke="#ffd700" stroke-width="3"/>
  <circle cx="50" cy="45" r="10" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">IX</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">HERMIT</text>
</svg>"#.to_string(),

        // 10 - Wheel of Fortune
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="75" r="25" fill="none" stroke="#ffd700" stroke-width="3"/>
  <line x1="50" y1="50" x2="50" y2="100" stroke="#ffd700" stroke-width="2"/>
  <line x1="25" y1="75" x2="75" y2="75" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">X</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="8" font-weight="bold">FORTUNE</text>
</svg>"#.to_string(),

        // 11 - Justice
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <line x1="50" y1="40" x2="50" y2="110" stroke="#ffd700" stroke-width="4"/>
  <rect x="35" y="55" width="30" height="5" fill="#ffd700"/>
  <circle cx="50" cy="90" r="8" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XI</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">JUSTICE</text>
</svg>"#.to_string(),

        // 12 - The Hanged Man
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <line x1="30" y1="45" x2="70" y2="45" stroke="#ffd700" stroke-width="3"/>
  <line x1="50" y1="45" x2="50" y2="75" stroke="#ffd700" stroke-width="3"/>
  <circle cx="50" cy="85" r="8" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XII</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="8" font-weight="bold">HANGED MAN</text>
</svg>"#.to_string(),

        // 13 - Death
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <rect x="40" y="50" width="20" height="40" fill="none" stroke="#ffd700" stroke-width="3"/>
  <circle cx="50" cy="40" r="8" fill="none" stroke="#ffd700" stroke-width="2"/>
  <line x1="35" y1="100" x2="65" y2="100" stroke="#ffd700" stroke-width="3"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XIII</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">DEATH</text>
</svg>"#.to_string(),

        // 14 - Temperance
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <rect x="30" y="50" width="15" height="30" fill="none" stroke="#ffd700" stroke-width="2"/>
  <rect x="55" y="60" width="15" height="30" fill="none" stroke="#ffd700" stroke-width="2"/>
  <path d="M 45 65 Q 50 70 55 65" stroke="#ffd700" stroke-width="2" fill="none"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XIV</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="8" font-weight="bold">TEMPERANCE</text>
</svg>"#.to_string(),

        // 15 - The Devil
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <polygon points="50,40 60,65 40,65" fill="none" stroke="#ffd700" stroke-width="3"/>
  <polygon points="50,100 40,75 60,75" fill="none" stroke="#ffd700" stroke-width="3"/>
  <circle cx="45" cy="45" r="3" fill="#ffd700"/>
  <circle cx="55" cy="45" r="3" fill="#ffd700"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XV</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">DEVIL</text>
</svg>"#.to_string(),

        // 16 - The Tower
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <rect x="35" y="50" width="30" height="50" fill="none" stroke="#ffd700" stroke-width="3"/>
  <polygon points="30,45 50,30 70,45" fill="none" stroke="#ffd700" stroke-width="2"/>
  <line x1="45" y1="30" x2="48" y2="20" stroke="#ffd700" stroke-width="2"/>
  <line x1="52" y1="30" x2="55" y2="20" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XVI</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">TOWER</text>
</svg>"#.to_string(),

        // 17 - The Star
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <polygon points="50,35 55,50 70,50 58,60 63,75 50,65 37,75 42,60 30,50 45,50" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="35" cy="45" r="2" fill="#ffd700"/>
  <circle cx="65" cy="45" r="2" fill="#ffd700"/>
  <circle cx="40" cy="90" r="2" fill="#ffd700"/>
  <circle cx="60" cy="90" r="2" fill="#ffd700"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XVII</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">STAR</text>
</svg>"#.to_string(),

        // 18 - The Moon
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <path d="M 50 40 Q 35 55 50 70 Q 65 55 50 40" fill="none" stroke="#ffd700" stroke-width="3"/>
  <circle cx="42" cy="55" r="3" fill="#ffd700"/>
  <circle cx="58" cy="55" r="3" fill="#ffd700"/>
  <path d="M 30 90 Q 50 100 70 90" stroke="#ffd700" stroke-width="2" fill="none"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XVIII</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">MOON</text>
</svg>"#.to_string(),

        // 19 - The Sun
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="65" r="20" fill="none" stroke="#ffd700" stroke-width="3"/>
  <line x1="50" y1="30" x2="50" y2="35" stroke="#ffd700" stroke-width="2"/>
  <line x1="50" y1="95" x2="50" y2="100" stroke="#ffd700" stroke-width="2"/>
  <line x1="25" y1="65" x2="30" y2="65" stroke="#ffd700" stroke-width="2"/>
  <line x1="70" y1="65" x2="75" y2="65" stroke="#ffd700" stroke-width="2"/>
  <line x1="35" y1="40" x2="38" y2="43" stroke="#ffd700" stroke-width="2"/>
  <line x1="65" y1="90" x2="62" y2="87" stroke="#ffd700" stroke-width="2"/>
  <line x1="65" y1="40" x2="62" y2="43" stroke="#ffd700" stroke-width="2"/>
  <line x1="35" y1="90" x2="38" y2="87" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XIX</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">SUN</text>
</svg>"#.to_string(),

        // 20 - Judgement
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <polygon points="40,45 50,30 60,45" fill="none" stroke="#ffd700" stroke-width="2"/>
  <rect x="45" y="45" width="10" height="25" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="35" cy="85" r="5" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="85" r="5" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="65" cy="85" r="5" fill="none" stroke="#ffd700" stroke-width="2"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XX</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="8" font-weight="bold">JUDGEMENT</text>
</svg>"#.to_string(),

        // 21 - The World
        r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="75" r="25" fill="none" stroke="#ffd700" stroke-width="3"/>
  <circle cx="50" cy="75" r="15" fill="none" stroke="#ffd700" stroke-width="2"/>
  <circle cx="50" cy="75" r="5" fill="#ffd700"/>
  <rect x="25" y="50" width="5" height="5" fill="#ffd700"/>
  <rect x="70" y="50" width="5" height="5" fill="#ffd700"/>
  <rect x="25" y="95" width="5" height="5" fill="#ffd700"/>
  <rect x="70" y="95" width="5" height="5" fill="#ffd700"/>
  <text x="50" y="25" text-anchor="middle" fill="#ffd700" font-size="14" font-weight="bold">XXI</text>
  <text x="50" y="135" text-anchor="middle" fill="#ffd700" font-size="9" font-weight="bold">WORLD</text>
</svg>"#.to_string(),
    ]
}

pub fn get_minor_arcana_placeholders() -> Vec<String> {
    let mut cards = Vec::new();

    let suits = vec!["WANDS", "CUPS", "SWORDS", "PENTACLES"];
    let colors = vec!["#e74c3c", "#3498db", "#9b59b6", "#f1c40f"];
    let symbols = vec!["⚡", "♡", "⚔", "◈"];

    for (suit_idx, (suit, color, symbol)) in suits.iter().zip(colors.iter()).zip(symbols.iter()).enumerate() {
        for rank in 1..=14 {
            let card_id = 22 + suit_idx * 14 + (rank - 1);
            let rank_name = match rank {
                1 => "A".to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                _ => rank.to_string(),
            };

            let svg = format!(r#"<svg width="100" height="150" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="150" fill="#1a1a2e" stroke="{color}" stroke-width="2"/>
  <text x="50" y="30" text-anchor="middle" fill="{color}" font-size="20" font-weight="bold">{symbol}</text>
  <text x="50" y="60" text-anchor="middle" fill="{color}" font-size="18" font-weight="bold">{rank_name}</text>
  <text x="50" y="135" text-anchor="middle" fill="{color}" font-size="10" font-weight="bold">{suit}</text>
</svg>"#, color = color, symbol = symbol, rank_name = rank_name, suit = suit);

            cards.push(svg);
        }
    }

    cards
}

pub fn get_all_tarot_cards() -> Vec<String> {
    let mut all_cards = get_major_arcana_svgs();
    all_cards.extend(get_minor_arcana_placeholders());
    all_cards
}