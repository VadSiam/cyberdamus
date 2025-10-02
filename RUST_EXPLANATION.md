# 🦀 RUST & ANCHOR ГЛУБОКОЕ ПОГРУЖЕНИЕ

## Разбор mint_fortune_nft() на примере теста

Этот документ объясняет пошагово, как работает функция `mint_fortune_nft()` с освежением знаний по Rust.

---

## 📋 СОДЕРЖАНИЕ

1. [Базовые концепции Rust](#1-базовые-концепции-rust)
2. [Anchor Framework магия](#2-anchor-framework-магия)
3. [Пошаговое выполнение теста](#3-пошаговое-выполнение-теста)
4. [Детальный разбор handler()](#4-детальный-разбор-handler)
5. [CPI Calls (Cross-Program Invocation)](#5-cpi-calls)
6. [Математика Fisher-Yates](#6-математика-fisher-yates)

---

## 1. БАЗОВЫЕ КОНЦЕПЦИИ RUST

### 1.1 Владение (Ownership)

```rust
// В Rust каждое значение имеет ОДНОГО владельца
let x = String::from("hello");  // x владеет строкой
let y = x;                       // Владение передается y
// println!("{}", x);            // ❌ ОШИБКА! x больше не владеет строкой
println!("{}", y);               // ✅ OK
```

**Правила владения:**
1. Каждое значение имеет переменную-владельца
2. Может быть только один владелец одновременно
3. Когда владелец выходит из области видимости, значение удаляется

**Почему это важно:**
- Нет garbage collector (быстрая работа)
- Безопасность памяти гарантируется на этапе компиляции
- Невозможны race conditions

### 1.2 Заимствование (Borrowing)

```rust
// Immutable borrow (неизменяемое заимствование)
let s = String::from("hello");
let len = calculate_length(&s);  // &s = ссылка на s (заимствование)
println!("String: {}, Length: {}", s, len);  // s все еще доступна!

fn calculate_length(s: &String) -> usize {
    s.len()  // Читаем, но не изменяем
}

// Mutable borrow (изменяемое заимствование)
let mut s = String::from("hello");
change(&mut s);  // &mut s = изменяемая ссылка
println!("{}", s);  // "hello, world"

fn change(s: &mut String) {
    s.push_str(", world");
}
```

**Правила заимствования:**
1. Может быть МНОГО immutable ссылок (&T)
2. Может быть ОДНА mutable ссылка (&mut T)
3. Но НЕ ОДНОВРЕМЕННО! Либо много &T, либо одна &mut T

**В нашем коде:**
```rust
pub fn handler(ctx: Context<MintFortuneNft>) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;  // Берем изменяемую ссылку
    let user = &ctx.accounts.user;          // Берем неизменяемую ссылку

    // oracle можем изменить:
    oracle.total_fortunes += 1;  // ✅

    // user только читать:
    let key = user.key();  // ✅
    // user.something = 123;  // ❌ Ошибка!
}
```

### 1.3 Lifetime Annotations ('info)

```rust
// В нашем коде везде встречается 'info
#[derive(Accounts)]
pub struct MintFortuneNft<'info> {
    pub oracle: Account<'info, Oracle>,
    pub user: Signer<'info>,
    // ...
}
```

**Что это значит:**
- `'info` = lifetime parameter (параметр времени жизни)
- Говорит компилятору: "Все ссылки в этой структуре живут одинаково долго"
- В Anchor это означает: "Пока выполняется транзакция"

**Почему нужно:**
```rust
// БЕЗ lifetime (не скомпилируется):
struct MyStruct {
    reference: &String,  // ❌ Как долго живет эта ссылка?
}

// С lifetime (OK):
struct MyStruct<'a> {
    reference: &'a String,  // ✅ Живет столько же, сколько 'a
}
```

### 1.4 Result<T, E> и Error Handling

```rust
// Result = либо Ok(значение), либо Err(ошибка)
enum Result<T, E> {
    Ok(T),   // Успех
    Err(E),  // Ошибка
}

// В нашем коде:
pub fn handler(ctx: Context<MintFortuneNft>) -> Result<()> {
    // Result<()> = либо Ok(()), либо Err(ProgramError)

    // Проверка с возвратом ошибки:
    require!(
        **user.to_account_info().lamports.borrow() >= fee_lamports,
        OracleError::InsufficientFunds  // Если false, вернет эту ошибку
    );

    // Вызов функции с ? оператором:
    let clock = Clock::get()?;  // Если ошибка, сразу вернет Err

    // Возврат успеха:
    Ok(())  // () = unit type (пустое значение, как void)
}
```

**Оператор ?:**
```rust
// Длинный путь:
let clock = match Clock::get() {
    Ok(c) => c,
    Err(e) => return Err(e),
};

// С оператором ? (то же самое):
let clock = Clock::get()?;
```

### 1.5 Macros (Макросы)

```rust
// Макрос = код, который генерирует код во время компиляции
// В Rust макросы отличаются ! на конце

msg!("Hello");        // Макрос для логирования
println!("Hello");    // Макрос для печати
vec![1, 2, 3];        // Макрос для создания вектора

// Anchor макросы:
#[program]            // Генерирует entry point
#[derive(Accounts)]   // Генерирует код валидации аккаунтов
#[account]            // Генерирует код сериализации
```

**Почему макросы:**
- Меньше boilerplate кода
- Проверки на этапе компиляции
- В Anchor генерирует много служебного кода

### 1.6 Traits (Трейты)

```rust
// Trait = интерфейс (набор методов)
trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }
}

// В нашем коде:
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FortuneData {
    // AnchorSerialize = trait для сериализации
    // AnchorDeserialize = trait для десериализации
    // Clone = trait для копирования
}
```

---

## 2. ANCHOR FRAMEWORK МАГИЯ

### 2.1 Как работает #[program]

```rust
#[program]
pub mod cyberdamus_nft {
    pub fn mint_fortune_nft(ctx: Context<MintFortuneNft>) -> Result<()> {
        // Наш код
    }
}
```

**Что генерирует Anchor:**
1. Entry point функцию для Solana
2. Сериализацию/десериализацию
3. Маршрутизацию инструкций
4. Валидацию аккаунтов

**Реальный entry point (генерируется автоматически):**
```rust
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 1. Десериализовать instruction_data
    // 2. Определить, какую функцию вызвать
    // 3. Валидировать аккаунты
    // 4. Вызвать mint_fortune_nft()
    // 5. Сериализовать результат
}
```

### 2.2 Как работает Context<T>

```rust
pub struct Context<'a, 'b, 'c, 'info, T> {
    pub program_id: &'a Pubkey,           // ID нашей программы
    pub accounts: &'b mut T,               // Структура с аккаунтами
    pub remaining_accounts: &'c [AccountInfo<'info>],
    pub bumps: BTreeMap<String, u8>,       // PDA bumps
}

// В нашем коде:
pub fn handler(ctx: Context<MintFortuneNft>) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;  // Доступ к аккаунтам
    let bump = ctx.bumps.oracle;            // PDA bump для oracle
}
```

### 2.3 Как работает #[derive(Accounts)]

```rust
#[derive(Accounts)]
pub struct MintFortuneNft<'info> {
    #[account(
        mut,                              // Аккаунт будет изменен
        seeds = [b"oracle"],              // PDA seeds
        bump,                             // Найти правильный bump
        constraint = oracle.is_initialized @ OracleError::NotInitialized
    )]
    pub oracle: Account<'info, Oracle>,
}
```

**Что генерирует Anchor:**
1. **Валидация PDA:** Проверяет, что oracle = PDA с seeds [b"oracle"]
2. **Десериализация:** Читает данные из аккаунта и парсит в Oracle struct
3. **Проверка constraint:** Выполняет `oracle.is_initialized == true`
4. **Проверка mut:** Гарантирует, что аккаунт может быть изменен
5. **Сохранение bump:** Добавляет bump в ctx.bumps для использования в CPI

### 2.4 Account Types

```rust
// Account<'info, T> - десериализованный аккаунт
pub oracle: Account<'info, Oracle>
// = AccountInfo + десериализация в Oracle

// Signer<'info> - аккаунт, который подписал транзакцию
pub user: Signer<'info>

// UncheckedAccount - НЕ десериализуется, только проверка права
pub treasury: UncheckedAccount<'info>

// Program<'info, T> - ссылка на другую программу
pub token_program: Program<'info, Token>

// Sysvar<'info, T> - системная переменная Solana
pub rent: Sysvar<'info, Rent>
```

---

## 3. ПОШАГОВОЕ ВЫПОЛНЕНИЕ ТЕСТА

Разберем, что происходит при выполнении теста "Successfully mints fortune NFT".

### Шаг 1: Подготовка (TypeScript)

```typescript
// 1. Создаем пользователя
const user = anchor.web3.Keypair.generate();
// user.publicKey = новый Pubkey
// user.secretKey = приватный ключ (64 bytes)

// 2. Airdrop SOL
const airdropTx = await provider.connection.requestAirdrop(
  user.publicKey,
  1 * anchor.web3.LAMPORTS_PER_SOL  // 1,000,000,000 lamports
);
// Отправляет RPC запрос к validator: "Дай 1 SOL этому pubkey"

// 3. Ждем подтверждения
await provider.connection.confirmTransaction(airdropTx);
// Ждет, пока транзакция попадет в блок
```

### Шаг 2: Деривация PDA (Program Derived Address)

```typescript
// Вычисляем mint PDA
const [mintPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("mint"),
    Buffer.from(new anchor.BN(0).toArray("le", 8))
  ],
  program.programId
);

// Что происходит внутри:
// 1. seeds = ["mint" + 0 (8 bytes little-endian)]
// 2. for bump in 255..0:
// 3.   address = hash(seeds + [bump] + program_id)
// 4.   if address не на кривой ed25519:
// 5.     return (address, bump)
```

**Почему PDA:**
- PDA = адрес БЕЗ приватного ключа
- Только программа может "подписывать" от имени PDA
- Детерминированный (одни и те же seeds → один PDA)

### Шаг 3: Вызов функции

```typescript
const tx = await program.methods
  .mintFortuneNft()         // 1. Имя функции
  .accounts({               // 2. Аккаунты
    oracle: oraclePda,
    user: user.publicKey,
    // ...
  })
  .signers([user])          // 3. Кто подписывает
  .rpc();                   // 4. Отправить транзакцию
```

**Что происходит:**
1. **Сериализация:** `program.methods.mintFortuneNft()` создает instruction data
2. **Создание транзакции:** Формирует Solana Transaction
3. **Подписание:** `user` подписывает транзакцию своим secretKey
4. **Отправка:** `.rpc()` отправляет на RPC endpoint
5. **Ожидание:** Anchor ждет подтверждения (finalized)

---

## 4. ДЕТАЛЬНЫЙ РАЗБОР handler()

Теперь разберем каждую строку функции `mint_fortune_nft::handler()`.

### Часть 1: Получение ссылок

```rust
pub fn handler(ctx: Context<MintFortuneNft>) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;
    let user = &ctx.accounts.user;
```

**Что происходит:**
1. `ctx` передается по значению (move)
2. `&mut ctx.accounts.oracle` = изменяемое заимствование
3. `&ctx.accounts.user` = неизменяемое заимствование

**Типы:**
- `oracle: &mut Account<'info, Oracle>`
- `user: &Signer<'info>`

### Часть 2: Проверка баланса

```rust
let fee_lamports = Oracle::FIXED_FEE;  // 50_000_000 (константа)

require!(
    **user.to_account_info().lamports.borrow() >= fee_lamports,
    OracleError::InsufficientFunds
);
```

**Разбор `**user.to_account_info().lamports.borrow()`:**

```rust
// user: &Signer<'info>
// .to_account_info() → &AccountInfo<'info>
// .lamports → Rc<RefCell<&'a mut u64>>  (умный указатель)
// .borrow() → Ref<&'a mut u64>           (заимствование)
// *         → &'a mut u64                 (разыменование)
// *         → u64                          (значение)
```

**Почему так сложно:**
- Solana требует, чтобы балансы изменялись через RefCell
- RefCell = проверка заимствования в runtime (не в compile-time)
- Rc = Reference Counted (счетчик ссылок)

**require! макрос:**
```rust
// Эквивалентно:
if !(**user.to_account_info().lamports.borrow() >= fee_lamports) {
    return Err(OracleError::InsufficientFunds.into());
}
```

### Часть 3: Transfer fee

```rust
anchor_lang::system_program::transfer(
    anchor_lang::context::CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: user.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        },
    ),
    fee_lamports,
)?;
```

**Что это:**
- **CPI (Cross-Program Invocation)** = вызов другой программы
- Вызываем System Program для transfer SOL
- `?` оператор = если ошибка, вернуть Err

**Структура CPI:**
1. `CpiContext::new()` создает контекст для вызова
2. `system_program.to_account_info()` = программа, которую вызываем
3. `Transfer { from, to }` = аккаунты для transfer
4. `fee_lamports` = параметр функции

**Что происходит в System Program:**
```rust
// System Program выполняет:
from.lamports -= fee_lamports;
to.lamports += fee_lamports;
```

### Часть 4: Генерация карт

```rust
let clock = Clock::get()?;
let timestamp = clock.unix_timestamp;  // i64
let slot = clock.slot;                 // u64
```

**Clock sysvar:**
- Системная переменная Solana
- Обновляется каждый блок
- Содержит: timestamp, slot, epoch

```rust
let recent_blockhash = &clock.epoch_start_timestamp.to_le_bytes();
let mut padded_hash = [0u8; 32];
padded_hash[..8].copy_from_slice(recent_blockhash);
```

**⚠️ ПРОБЛЕМА (которую нужно исправить):**
- `epoch_start_timestamp` = СТАТИЧНОЕ значение (меняется раз в 2 дня)
- Нужен настоящий recent_blockhash для сильной энтропии

```rust
let cards = generate_fortune_cards(
    &user.key(),              // Pubkey пользователя
    oracle.total_fortunes,    // u64 счетчик
    timestamp,                // i64 время
    &padded_hash,             // [u8; 32] blockhash
);
```

**generate_fortune_cards() разберем отдельно ниже.**

### Часть 5: Обновление состояния

```rust
oracle.total_fortunes += 1;
let fortune_number = oracle.total_fortunes;
```

**Важно:**
- `oracle` = &mut Account<'info, Oracle>
- Изменения сохранятся автоматически в конце транзакции
- Anchor сериализует oracle обратно в аккаунт

### Часть 6: Mint NFT

```rust
mint_to(
    CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.nft_mint.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: oracle.to_account_info(),
        },
        &[&[
            b"oracle",
            &[ctx.bumps.oracle],
        ]],
    ),
    1,  // Количество токенов
)?;
```

**CpiContext::new_with_signer:**
- `new_with_signer` = CPI с PDA подписью
- `&[&[b"oracle", &[ctx.bumps.oracle]]]` = seeds для PDA

**Что происходит:**
1. Token Program проверяет: authority = PDA(seeds=[b"oracle", bump])
2. Token Program создает 1 токен в mint
3. Token Program переводит 1 токен на user_nft_account

### Часть 7: Создание Metadata

```rust
create_metadata_accounts_v3(
    CpiContext::new_with_signer(...),
    DataV2 {
        name: metadata_title,
        symbol: "TAROT".to_string(),
        uri: metadata_image,
        // ...
    },
    true,  // is_mutable
    true,  // update_authority_is_signer
    None,  // collection_details
)?;
```

**DataV2:**
- Struct из Metaplex Token Metadata программы
- Содержит все metadata для NFT

**is_mutable = true:**
- Metadata можно обновить позже
- Нужно для update_authority (oracle PDA)

### Часть 8: Логирование

```rust
msg!(
    "🔮 Fortune #{} created! User: {}, Cards: [{}, {}, {}], Fee: {} lamports",
    fortune_number,
    user.key(),
    cards[0], cards[1], cards[2],
    fee_lamports
);
```

**msg! макрос:**
- Записывает в program logs
- Видно в explorer (Solscan, Solana Explorer)
- Полезно для debugging

### Часть 9: Возврат успеха

```rust
    Ok(())
}
```

**Ok(()):**
- Возвращает Result::Ok с unit type ()
- Транзакция успешна
- Anchor сохранит все изменения аккаунтов

---

## 5. CPI CALLS

### 5.1 Что такое CPI

**CPI (Cross-Program Invocation)** = одна программа вызывает другую.

```
Our Program              Token Program
    |                         |
    | mint_to()               |
    |------------------------>|
    |                         | (mint 1 token)
    |                         |
    |<------------------------|
    |         success         |
```

### 5.2 Зачем CpiContext::new_with_signer

**Проблема:** PDA не имеет приватного ключа.

**Решение:** Program может "подписывать" от имени PDA используя seeds.

```rust
CpiContext::new_with_signer(
    program.to_account_info(),
    accounts,
    &[&[b"oracle", &[bump]]]  // ← seeds для PDA подписи
)
```

**Как работает:**
1. Solana Runtime проверяет: вызывающая программа = program_id из PDA
2. Runtime вычисляет PDA(seeds + bump + program_id)
3. Если PDA совпадает с authority, то "подпись" валидна

### 5.3 Типы CPI в нашем коде

```rust
// 1. System Program Transfer (SOL transfer)
anchor_lang::system_program::transfer(...)

// 2. Token Program Mint (создание токенов)
mint_to(...)

// 3. Metaplex Metadata (создание metadata)
create_metadata_accounts_v3(...)
```

---

## 6. МАТЕМАТИКА FISHER-YATES

### 6.1 Алгоритм

```rust
pub fn generate_fortune_cards(
    user_key: &Pubkey,
    oracle_counter: u64,
    timestamp: i64,
    recent_blockhash: &[u8; 32],
) -> [u8; 3] {
    // 1. Собираем энтропию
    let mut seed_data = Vec::new();
    seed_data.extend_from_slice(&user_key.to_bytes());
    seed_data.extend_from_slice(&oracle_counter.to_le_bytes());
    seed_data.extend_from_slice(&timestamp.to_le_bytes());
    seed_data.extend_from_slice(recent_blockhash);

    // 2. Хешируем
    let hash = anchor_lang::solana_program::hash::hash(&seed_data);
    let hash_bytes = hash.to_bytes();  // [u8; 32]

    // 3. Создаем колоду [0, 1, 2, ..., 77]
    let mut deck: [u8; 78] = [0; 78];
    for i in 0..78 {
        deck[i] = i as u8;
    }

    // 4. Fisher-Yates shuffle
    let mut hash_index = 0;
    for i in (1..78).rev() {  // i = 77, 76, 75, ..., 1
        let random_byte = hash_bytes[hash_index % 32];
        hash_index += 1;

        let j = (random_byte as usize) % (i + 1);
        deck.swap(i, j);
    }

    // 5. Возвращаем первые 3 карты
    [deck[0], deck[1], deck[2]]
}
```

### 6.2 Почему Fisher-Yates дает равномерное распределение

**Математическое доказательство:**

На каждом шаге i:
- Мы выбираем случайный j из [0, i]
- Меняем местами deck[i] и deck[j]

**Вероятность для любой перестановки:**

```
Итерация 77: выбираем из 78 элементов → вероятность 1/78
Итерация 76: выбираем из 77 элементов → вероятность 1/77
...
Итерация 1:  выбираем из 2 элементов  → вероятность 1/2

Итоговая вероятность любой перестановки:
P = (1/78) × (1/77) × ... × (1/2) = 1/(78!)

Всего возможных перестановок: 78!

Значит, каждая перестановка равновероятна!
```

### 6.3 Почему НЕ просто `hash % 78` для каждой карты?

**Плохой подход:**
```rust
// ❌ НЕПРАВИЛЬНО
let card1 = hash[0] % 78;
let card2 = hash[1] % 78;
let card3 = hash[2] % 78;
// Проблема: могут быть дубликаты!
```

**Правильный подход (Fisher-Yates):**
```rust
// ✅ ПРАВИЛЬНО
// Перемешиваем всю колоду
// Берем первые 3 карты
// Гарантия: нет дубликатов
```

### 6.4 Источники энтропии

```rust
user_key         // 32 bytes - уникальный для каждого юзера
oracle_counter   // 8 bytes  - увеличивается каждый минт
timestamp        // 8 bytes  - время в секундах
recent_blockhash // 32 bytes - хеш блока (должен быть настоящий!)
---
ИТОГО: 80 bytes энтропии → SHA256 → 32 bytes hash
```

**Зачем столько источников:**
- **user_key:** Разные юзеры → разные карты
- **oracle_counter:** Один юзер → разные карты каждый минт
- **timestamp:** Минты в разное время → разные карты
- **recent_blockhash:** Непредсказуемость (нельзя заранее узнать)

---

## 7. РЕЗЮМЕ: ПОЛНЫЙ FLOW ТРАНЗАКЦИИ

### 7.1 Client → Blockchain

```
1. TypeScript создает транзакцию:
   - Instruction data: [функция_id, параметры]
   - Accounts: [oracle, user, treasury, ...]
   - Signatures: [user.sign()]

2. Отправка на RPC:
   - HTTP POST /
   - Body: { jsonrpc: "2.0", method: "sendTransaction", params: [...] }

3. Validator получает транзакцию:
   - Проверяет подписи
   - Загружает program (BPF bytecode)
   - Загружает аккаунты из state
```

### 7.2 Blockchain → Program

```
4. Solana Runtime:
   - Создает AccountInfo для каждого аккаунта
   - Вызывает process_instruction()

5. Anchor framework:
   - Десериализует instruction_data
   - Определяет функцию: mint_fortune_nft
   - Валидирует аккаунты (#[derive(Accounts)])
   - Десериализует Account<'info, Oracle>

6. Наш код (handler):
   - Проверяет баланс
   - Transfer fee (CPI)
   - Генерирует карты (Fisher-Yates)
   - Mint token (CPI)
   - Create metadata (CPI)

7. Anchor завершает:
   - Сериализует oracle обратно
   - Сохраняет изменения в state

8. Solana Runtime:
   - Записывает изменения в block
   - Возвращает success
```

### 7.3 Blockchain → Client

```
9. Validator возвращает:
   - HTTP 200 OK
   - Body: { result: "transaction_id" }

10. Client ждет подтверждения:
   - Polling: getTransaction(tx_id)
   - Пока status != "finalized"

11. Тест проверяет:
   - NFT создан
   - Balance уменьшился
   - Metadata корректна
   - Cards уникальны
```

---

## 8. ЧЕКЛИСТ: ЧТО НУЖНО ЗНАТЬ

### ✅ Rust Basics
- [x] Ownership (владение)
- [x] Borrowing (&T, &mut T)
- [x] Lifetimes ('a)
- [x] Result<T, E>
- [x] Macros (msg!, require!)
- [x] Traits (derive)

### ✅ Anchor Concepts
- [x] #[program] macro
- [x] Context<T>
- [x] #[derive(Accounts)]
- [x] Account<'info, T>
- [x] Signer<'info>
- [x] UncheckedAccount
- [x] PDA (Program Derived Address)

### ✅ Solana Concepts
- [x] AccountInfo
- [x] Sysvar (Clock, Rent)
- [x] CPI (Cross-Program Invocation)
- [x] Lamports
- [x] Seeds и bumps

### ✅ Our Program Logic
- [x] Oracle state
- [x] Fisher-Yates shuffle
- [x] NFT minting flow
- [x] Metadata creation
- [x] Fee collection

---

## 9. СЛЕДУЮЩИЕ ШАГИ

1. **Запустить тесты:**
   ```bash
   anchor test
   ```

2. **Исправить критические проблемы:**
   - Blockhash энтропия
   - Master Edition
   - Collection NFT

3. **Запустить статистический тест:**
   ```bash
   anchor test -- --grep "Statistical test"
   ```

4. **Изучить детали:**
   - Открой `mint_fortune_nft.rs`
   - Добавь `msg!()` для debugging
   - Запусти тесты и смотри logs

---

**Автор:** Claude (AI Assistant)
**Дата:** 2025-10-01
**Версия:** 1.0 - Initial Rust explanation
