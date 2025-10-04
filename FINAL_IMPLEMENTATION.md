# 🔮 CYBERDAMUS TOKEN-2022 - ФИНАЛЬНЫЙ ПЛАН РЕАЛИЗАЦИИ

## 📊 ТЕКУЩИЙ СТАТУС РАЗРАБОТКИ (2025-10-04)

### 🔄 МИГРАЦИЯ: NFT → TOKEN-2022
- **Предыдущая архитектура:** Metaplex NFT (deprecated)
- **Новая архитектура:** Token-2022 с Metadata Extension
- **Причины миграции:**
  - 67% дешевле (0.0057 SOL vs 0.0175 SOL per mint)
  - 100% децентрализация (метаданные on-chain)
  - Полная свобода пользователей (no freeze authority)
  - Единый cards.json для всех токенов (нет дубликатов)
  - Карты видны в имени токена (любой кошелек)

### ⚠️ TOKEN-2022 IMPLEMENTATION (IN PROGRESS)
- **Oracle structure:** Готова (IPFS hash storage)
- **Fisher-Yates algorithm:** Готов
- **Token-2022 mint:** TODO - заменить mint_fortune_nft.rs
- **Name encoding:** TODO - "CyberDamus #AABBCC" logic
- **Metadata Extension:** TODO - additional_metadata setup
- **IPFS cards.json:** TODO - create structure

### 🎯 СЛЕДУЮЩИЕ ШАГИ
- [ ] Создать cards.json с метаданными 78 карт
- [ ] Загрузить 78 PNG файлов + cards.json на IPFS
- [ ] Реализовать mint_fortune_token() с Token-2022
- [ ] Написать тесты для Token-2022
- [ ] Фаза 3: Frontend Development (парсинг имени)
- [ ] Фаза 4: Тестирование на Devnet
- [ ] Фаза 5: Mainnet Deployment

## 📌 КЛЮЧЕВЫЕ РЕШЕНИЯ
✅ **Архитектура:** Token-2022 с Metadata Extension (не NFT!)
✅ **Дизайн карт:** НЕИЗМЕННЫЙ, один навсегда (78 PNG на IPFS)
✅ **Метаданные:** Единый cards.json для ВСЕХ токенов
✅ **Name encoding:** "CyberDamus #AABBCC" (карты видны сразу)
✅ **Freeze authority:** None (полная свобода пользователей)
✅ **Изменяемый параметр:** ТОЛЬКО комиссия (раз в 3-6 месяцев)
✅ **Разработка:** ЛОКАЛЬНО, не в Playground

## 🏗️ АРХИТЕКТУРА

### 1. ORACLE STRUCTURE (без изменений)
```rust
// Единственная структура данных
pub struct Oracle {
    authority: Pubkey,              // 32 bytes - администратор
    treasury: Pubkey,               // 32 bytes - кошелек комиссий
    total_fortunes: u64,            // 8 bytes - счетчик токенов
    ipfs_base_hash: [u8; 46],       // 46 bytes - базовый IPFS хеш
    is_initialized: bool,           // 1 byte - флаг
    reserved: [u8; 5],              // 5 bytes - резерв
    total_size: 132 bytes           // Общий размер (8 discriminator + 124)
}
```

### 2. IPFS СТРУКТУРА (новая)
```
Единая директория на IPFS:
├── cards.json   (ОДИН файл для ВСЕХ токенов)
├── 0.png        (The Fool)
├── 1.png        (The Magician)
├── 2.png        (The High Priestess)
├── ...
└── 77.png       (King of Pentacles)

Базовый хеш: bafybei...
Доступ к картинкам: ipfs://{base_hash}/{card_id}.png
Доступ к метаданным: ipfs://{base_hash}/cards.json
```

### 3. TOKEN-2022 СТРУКТУРА (новая)
```
On-chain хранится (Metadata Extension, ~103 bytes):
- name: "CyberDamus #AABBCC"  (21 bytes)
  где AA, BB, CC = 2-значные ID карт (00-77)
  Пример: "#000377" = карты [0, 3, 77]
- symbol: "TAROT"  (5 bytes)
- uri: "ipfs://{CID}/cards.json"  (40 bytes)
- additional_metadata: [("fortune_number", "377")]  (37 bytes)
- freeze_authority: None

cards.json на IPFS (общий для всех токенов):
{
  "cards": [
    {"id": 0, "name": "The Fool", "image": "ipfs://{CID}/0.png"},
    {"id": 1, "name": "The Magician", "image": "ipfs://{CID}/1.png"},
    ...
    {"id": 77, "name": "King of Pentacles", "image": "ipfs://{CID}/77.png"}
  ]
}

Компоненты Token-2022:
- Mint Account (Token-2022 + Metadata Extension, supply=1, decimals=0)
- Token Account (владение пользователя)
- Metadata встроена в Mint Account (не отдельный аккаунт!)

Frontend обработка:
1. Парсинг name: "CyberDamus #000377" → [0, 3, 77]
2. Fetch cards.json по uri
3. Фильтрация: cards.filter(c => [0,3,77].includes(c.id))
4. Отображение: Past (0), Present (3), Future (77)
```

## 💰 ЭКОНОМИКА ПРОЕКТА (Token-2022)

### РАЗОВЫЕ ЗАТРАТЫ (при деплое):
- Программа (BPF bytecode): 0.52 SOL ($104)
  - Anchor версия с Token-2022: ~90KB
  - Без Metaplex зависимостей: экономия -80KB
- Oracle PDA (132 bytes): 0.003 SOL ($0.60)
- **ИТОГО:** 0.523 SOL (~$104.60)

### СТОИМОСТЬ ОДНОГО TOKEN-2022:
- Mint account (Token-2022): 0.002 SOL ($0.40)
- Metadata Extension (~103 bytes): 0.0012 SOL ($0.24)
- Token account: 0.002 SOL ($0.40)
- Transaction fees: 0.0005 SOL ($0.10)
- **ИТОГО:** 0.0057 SOL ($1.14)
- **ЭКОНОМИЯ vs NFT:** 67% (-0.0118 SOL)

### ЦЕНА ДЛЯ ПОЛЬЗОВАТЕЛЯ:
- Комиссия оракула: 0.05 SOL ($10) - ФИКСИРОВАННАЯ
- Покрытие Token-2022: 0.0057 SOL ($1.14)
- Gas fee: 0.0005 SOL ($0.10)
- **ИТОГО:** 0.0562 SOL (~$11.24)
- **ДЕШЕВЛЕ vs NFT:** 17% (-$2.36)

### ПРИБЫЛЬ:
- С каждого токена: 0.0443 SOL (~$8.86) чистыми
- Окупаемость: после **12 токенов** (vs 17 NFT)
- 100 токенов = **3.90 SOL ($780)** (vs 2.73 SOL для NFT)
- 1000 токенов = **43.77 SOL ($8,754)** (vs 31.98 SOL для NFT)
- ROI: **777%** на каждом токене!

## 🔧 ТЕХНИЧЕСКАЯ РЕАЛИЗАЦИЯ

### ФАЗА 1: ПОДГОТОВКА (День 1-2) ✅ ЗАВЕРШЕНА
1. Установка Rust, Solana CLI, Anchor
2. Создание проекта `anchor init cyberdamus_nft`
3. Добавление Token-2022 зависимостей
4. Конфигурация Cargo.toml с оптимизациями

### ФАЗА 2: TOKEN-2022 MIGRATION (День 3-5) ⚠️ IN PROGRESS
**TODO - Заменить mint_fortune_nft.rs:**
```rust
// Только 2 функции!
pub fn initialize_oracle(ipfs_base_hash: [u8; 46])  // ✅ Готова - IPFS hash storage
pub fn mint_fortune_token()                          // ⚠️ TODO - Token-2022 mint
// УБРАЛИ: upload_cards(), update_fee(), rarity - не нужны!
```

**Что есть (из NFT implementation):**
- ✅ Oracle структура (132 bytes)
- ✅ Fisher-Yates алгоритм для генерации уникальных карт
- ✅ Все 78 названий карт Таро в get_card_name()
- ✅ Transfer fee to treasury

**Что нужно заменить:**
- ⚠️ TODO: Заменить Metaplex на Token-2022 program
- ⚠️ TODO: Encode cards в name: "CyberDamus #AABBCC"
- ⚠️ TODO: Set additional_metadata: fortune_number
- ⚠️ TODO: Удалить Master Edition/Collection код (не нужно!)
- ⚠️ TODO: Исправить blockhash энтропию
- ⚠️ TODO: Emergency pause механизм

**Anchor оптимизации (TODO):**
```toml
# Cargo.toml - агрессивные оптимизации
[features]
no-serde = []     # -15KB (минимальная сериализация)
no-std = []       # -25KB (без стандартной библиотеки)
no-idl = []       # -20KB (без интерфейса)
no-log-ix-name = [] # -10KB (без логирования)

[profile.release-optimized]
strip = "symbols" # -10KB (убираем debug символы)
panic = "abort"   # -5KB (без panic handling)
opt-level = "z"   # Максимальная оптимизация размера
lto = "fat"       # Link Time Optimization
```

### ФАЗА 2.5: ANCHOR → VANILLA SOLANA MIGRATION (День 6-10) - OPTIONAL
**Цель:** Дальнейшая оптимизация размера программы с ~90KB до 60-80KB

**Статус:** Рассматривается после успешного Token-2022 тестирования на devnet

**План конвертации (если понадобится):**
```rust
// 1. Удалить все Anchor macros
// 2. Заменить на чистый Solana Rust
// 3. Ручная сериализация/десериализация
// 4. Прямые CPI calls без Anchor wrapper

// ДО (Anchor):
#[program]
pub mod cyberdamus_nft {
    pub fn initialize_oracle(ctx: Context<InitializeOracle>, ...) -> Result<()>
}

// ПОСЛЕ (Vanilla Solana):
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => initialize_oracle(accounts, &instruction_data[1..]),
        1 => mint_fortune_token(accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
```

**Преимущества:**
- Размер: 200KB → 60-80KB (-60-70%)
- Gas: меньше compute units
- Контроль: полный контроль над каждой операцией
- Простота: всего 2 функции легко конвертировать

**Недостатки:**
- Время: +3-5 дней разработки
- Тестирование: нужно переписать все тесты
- Отладка: меньше автоматических проверок Anchor

**Решение:** Конвертируем после успешного devnet тестирования Anchor версии

### ФАЗА 3: NFT МИНТИНГ (День 11-13)
```typescript
// 1. Генерация 3 случайных карт on-chain: [5, 23, 67]
// 2. On-chain создание:
//    - SPL токен (supply=1)
//    - Metadata с Past картой как image
//    - Master Edition Account
//    - Collection verification
// 3. Off-chain (опционально):
//    - Создание metadata JSON
//    - Загрузка JSON на IPFS
//    - Update metadata URI (если нужно)
```

### ФАЗА 4: FRONTEND (День 14-17)
- React приложение
- Подключение кошельков (Phantom, Solflare)
- Визуализация расклада
- История гаданий

### ФАЗА 5: ТЕСТИРОВАНИЕ (День 18-20)
- ✅ Devnet тестирование (100+ минтов)
- ✅ Проверка в Phantom (3 карты в attributes)
- ✅ Тест на Magic Eden (collection отображается)
- ✅ Проверка Master Edition (NFT торгуется)
- ✅ Проверка фиксированной комиссии
- ✅ Security audit (bug bounty)

### ФАЗА 6: MAINNET (День 21-23)
- Деплой как upgradeable program
- Первые 10 тестовых NFT
- Запуск для сообщества
- Мониторинг стабильности

### ФАЗА 7: ФИНАЛИЗАЦИЯ (Через 6-12 месяцев)
- Анализ стабильности программы (минимум 10,000 успешных минтов)
- Команда: `solana program set-upgrade-authority <PROGRAM_ID> --final`
- Программа становится immutable навсегда
- Анонс "CyberDamus Forever" - математическая гарантия неизменности
- Публикация финального аудита безопасности

## 🔒 ГАРАНТИИ И ДОВЕРИЕ

### НЕИЗМЕНЯЕМЫЕ ЭЛЕМЕНТЫ:
✅ Комиссия: 0.05 SOL (hardcoded навсегда)
✅ Дизайн всех 78 карт (навсегда один, pre-loaded на IPFS)
✅ NFT после минта (никто не может изменить)
✅ Алгоритм генерации (Fisher-Yates)
✅ Структура данных (Oracle 132 bytes)
✅ Master Edition (каждый NFT уникален, supply=1)
✅ Collection: "CyberDamus Tarot" / Symbol: "TAROT"

### ФИЛОСОФИЯ БЕЗ ОГРАНИЧЕНИЙ:
❌ НЕТ rate limits (дневных лимитов)
✅ Экономический барьер: 0.068 SOL за гадание
✅ Свобода выбора: пользователь сам решает, сколько гаданий нужно
✅ Нет патерналистских ограничений
✅ Emergency pause только на случай технических атак

### ЭТАПЫ ДОВЕРИЯ:
🔄 **Год 1 (Upgradeable):**
- Исправление критических багов возможно
- Изменения только через upgrade authority
- Все апдейты прозрачны в блокчейне
- Сообщество контролирует через `solana program show`

🔒 **После года (Immutable):**
- Команда: `set-upgrade-authority --final`
- Программа становится неизменяемой НАВСЕГДА
- Математическая гарантия неизменности
- Невозможны никакие модификации

## 📁 СТРУКТУРА ФАЙЛОВ
```
cyberdamus_nft/
├── programs/
│   └── src/
│       ├── lib.rs          # Основной контракт (2 функции)
│       ├── state.rs        # Oracle структура (130 bytes)
│       └── instructions/   # Минтинг и инициализация
├── app/
│   ├── src/               # React frontend
│   └── public/            # Статика
├── tests/                 # Упрощенные тесты
└── scripts/
    ├── deploy.sh          # Upgradeable деплой
    └── setup_ipfs.ts      # Настройка IPFS directory
```

## 🚀 ЭВОЛЮЦИЯ USER FLOW

### MVP Devnet (СЕЙЧАС)
**devnet.cyberdamus.com**
```
Пользователь → Подключить Phantom → Нажать "Получить гадание" → Оплата 0.05 SOL → NFT в кошельке
```
- ✅ Веб-интерфейс (dApp)
- ✅ Вызов функции `mint_fortune_nft()` через Anchor
- ✅ Интеграция возможна (через IDL для разработчиков)
- ❌ Нельзя "просто отправить SOL" на адрес

**Для разработчиков:**
```typescript
await program.methods.mintFortuneNft().rpc();
```

### Mainnet v1.0 (через 1-2 недели)
**cyberdamus.com**
```
То же что devnet, но на реальной сети
```
- ✅ Проверенный код с devnet
- ✅ Настоящие IPFS картинки
- ✅ Upgrade authority = deployer (можем апгрейдить)
- 📚 Документация API для интеграторов (создаем на этом этапе)

### Mainnet v1.1 - UPGRADE (через 1-2 месяца)
**Добавляем "просто отправь SOL"**
```
Пользователь → Отправить 0.05 SOL на адрес → Автоматически получить NFT
```
- ✅ Старые интеграции работают БЕЗ изменений
- ✅ Fallback entrypoint обрабатывает простые переводы
- ✅ Работает из ЛЮБОГО кошелька (Phantom, Solflare, Backpack...)
- ✅ Интеграция в игры, приложения, другие сервисы

**Upgrade через:**
```bash
anchor upgrade target/deploy/cyberdamus_nft.so \
  --program-id 2zmR8N51Q7KYZqnzJJWaJkM3wbxwBqj2gimNPf8Ldqu7
```

### Mainnet v2.0 - IMMUTABLE (через год)
**Навсегда неизменная программа**
```bash
solana program set-upgrade-authority \
  2zmR8N51Q7KYZqnzJJWaJkM3wbxwBqj2gimNPf8Ldqu7 \
  --final
```
- ✅ Полное доверие: код нельзя изменить НИКОГДА
- ✅ Комиссия 0.05 SOL зафиксирована навсегда
- ✅ Гарантия неизменности для всех пользователей

---

## ✅ ЧЕКЛИСТ ЗАПУСКА

### Phase 2: Smart Contract ✅ COMPLETED
- [x] Программа разработана (Anchor 0.31.1)
- [x] Задеплоена на Devnet
  - [x] Program ID: `2zmR8N51Q7KYZqnzJJWaJkM3wbxwBqj2gimNPf8Ldqu7`
  - [x] Size: 304 KB (Anchor version)
  - [x] Deploy cost: 2.17 SOL
- [x] Oracle инициализирован на Devnet
  - [x] PDA: `22qT1BuA8LCXq3faEV3dbxmdmHAxwamTDFvVdsJ4eYxR`
  - [x] Authority/Treasury configured
  - [⚠️] IPFS hash: TEST placeholder (need real assets!)
- [x] Devnet scripts created
  - [x] `scripts/init_oracle_devnet.ts`
  - [x] `scripts/mint_nft_devnet.ts`

### Phase 3: IPFS Assets ⚠️ BLOCKER
- [ ] **CRITICAL:** Create 78 Tarot card designs
  - [ ] 0-21: Major Arcana (22 cards)
  - [ ] 22-35: Minor Arcana - Wands (14 cards)
  - [ ] 36-49: Minor Arcana - Cups (14 cards)
  - [ ] 50-63: Minor Arcana - Swords (14 cards)
  - [ ] 64-77: Minor Arcana - Pentacles (14 cards)
- [ ] Upload directory to IPFS (Pinata/NFT.Storage/web3.storage)
- [ ] Get real CID (base hash)
- [ ] Re-initialize Oracle OR add update function

### Phase 4: Critical Code Fixes (Before Mainnet)
- [ ] Master Edition Account добавлен
- [ ] Collection NFT создается при инициализации
- [ ] Blockhash энтропия исправлена
- [ ] Emergency pause механизм добавлен
- [ ] Metadata URI показывает Past карту

### Phase 5: Devnet Testing (After IPFS Ready)
- [x] Initialize oracle ✅
- [ ] Mint NFT с Master Edition
- [ ] Collection verification
- [ ] 100+ успешных минтов
- [ ] NFT правильно отображается:
  - [ ] В Phantom (Past карта как главное изображение)
  - [ ] Attributes содержат все 3 карты
  - [ ] На Magic Eden (collection "CyberDamus Tarot")
- [ ] Фиксированная комиссия 0.05 SOL работает
- [ ] Anchor → Vanilla Solana migration:
  - [ ] Конвертация кода
  - [ ] Тестирование
  - [ ] Сравнение размеров (200KB → 60-80KB)
- [ ] Frontend готов:
  - [ ] Подключение кошельков
  - [ ] Визуализация 3 карт
  - [ ] История гаданий
- [ ] Security audit пройден
- [ ] Upgradeable деплой на mainnet
- [ ] Первые 10 реальных NFT
- [ ] Анонс: Twitter/Discord/cyberdamus.com
- [ ] (Через год) Переход на immutable

## 🎯 ИТОГОВАЯ ОЦЕНКА
**РЕАЛИЗУЕМОСТЬ: 92%**
- Максимально простая архитектура (2 функции)
- Минимальные риски и сложности
- Оптимизированная стоимость деплоя ($104.60)
- Отличная маржа ($6.50 с NFT, ROI 265%)
- Быстрая окупаемость (17 NFT)
- Путь к полному доверию (immutable)
- Финальная оптимизация: 60-80KB (vanilla Solana)
- Философия свободы без искусственных ограничений
- Collection и Master Edition для полной совместимости с маркетплейсами

## 🚀 СЛЕДУЮЩИЕ ШАГИ (ПРИОРИТЕТ)

### ✅ Завершено (2025-10-02)
- Программа задеплоена на Devnet (Program: `2zmR8N51Q7KYZqnzJJWaJkM3wbxwBqj2gimNPf8Ldqu7`)
- Oracle инициализирован (PDA: `22qT1BuA8LCXq3faEV3dbxmdmHAxwamTDFvVdsJ4eYxR`)
- Devnet скрипты готовы

### ⚠️ Критический Блокер
1. **СОЗДАТЬ 78 TAROT CARD PNG** (0.png - 77.png)
   - Major Arcana: 0-21
   - Wands: 22-35
   - Cups: 36-49
   - Swords: 50-63
   - Pentacles: 64-77

2. **Загрузить на IPFS и получить CID**
   - Pinata / NFT.Storage / web3.storage
   - Получить реальный базовый хеш

3. **Пере-инициализировать Oracle**
   - Либо новый Oracle с реальным хешем
   - Либо добавить `update_ipfs_hash()` функцию

### Следующие Задачи
4. **Исправить критические проблемы кода:**
   - Master Edition Account
   - Blockhash энтропия
   - Collection NFT
   - Metadata URI → Past карта
   - Emergency pause механизм

5. **Devnet тестирование с реальным IPFS** (100+ минтов)

6. **Anchor → Vanilla Solana migration**

7. **Upgradeable деплой на mainnet**

8. **(Через год)** Финализация как immutable программа

---
*Документ создан: 2025-09-18*
*Документ обновлен: 2025-10-02*
*Версия: 1.3 - Devnet deployment status*

**Ключевые изменения v1.3:**
- ✅ Devnet deployment completed
- ✅ Program ID: `2zmR8N51Q7KYZqnzJJWaJkM3wbxwBqj2gimNPf8Ldqu7`
- ✅ Oracle PDA: `22qT1BuA8LCXq3faEV3dbxmdmHAxwamTDFvVdsJ4eYxR`
- ⚠️ IPFS hash placeholder identified - need real Tarot assets
- ✅ Обновлен чеклист с текущим прогрессом
- ✅ Приоритизированы следующие шаги (IPFS assets - blocker)