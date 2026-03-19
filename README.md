# snap-coin-opcode

A Rust library for encoding and decoding SNAP Coin opcode messages.
Wallet-to-wallet structured communication using micro-payment amounts as opcodes.

## Why This Is Possible

Encoding communication into micro-payment amounts is only practical on a
feeless chain. On most blockchain networks every transaction carries a fee —
sending 30 individual opcode transactions to have a conversation would cost
more in fees than the messages are worth.

SNAP Coin is feeless by design. That changes everything.

It means a full wallet-to-wallet conversation — questions, answers, scheduling,
workflow signals — costs nothing beyond the dust amounts used as opcodes.
Those amounts are not burned, they arrive at the destination wallet intact.

This transforms SNAP transactions from simple value transfer into a
structured communication layer. The chain becomes a permanent, ordered,
machine-readable message archive — something that is only viable when
fees are not a barrier to micro-transaction volume.

## How It Works

SNAP Coin transactions are atomic to 8 decimal places.
This library uses that precision to encode meaning into transaction amounts.

Every micro-payment sent between two wallets can carry an opcode:
```
0.0FFOOOO0

0     = reserved (future use)
FF    = family   (00-99)
OOOO  = opcode   (0000-9999)
0     = reserved (future use)
```

Example:
```
0.00100010  =  family 01 (handshake),  opcode 0001  =  HELLO
0.00300210  =  family 03 (scheduling), opcode 0021  =  MEET_TOMORROW_1400
0.00700010  =  family 07 (question),   opcode 0001  =  AVAILABLE?
0.00800010  =  family 08 (answer),     opcode 0001  =  YES_AVAILABLE
```

Any standard SNAP wallet can send opcode messages today.
No protocol changes required. No special wallet needed.
The dictionary is the only key needed to encode or decode.

## Dictionary

`dictionary/dictionary.json` is the single source of truth for all opcodes.

It defines:
- token name
- family and opcode
- on-chain amount
- category
- meaning
- display label (for phrases)
- question/answer links

The dictionary is versioned. All consumers — web app, mobile wallet,
chain scanner — load the same file.

### Families

| Family | Category   | Description                        |
|--------|------------|------------------------------------|
| 01     | handshake  | HELLO, ACK, BYE                    |
| 02     | response   | YES, NO, ACCEPT, REJECT            |
| 03     | scheduling | meeting phrases by day and hour    |
| 04     | workflow   | QUOTE, JOB, INVOICE, PAYMENT       |
| 05     | status     | URGENT, PENDING, DELAYED           |
| 06     | single_word| fallback single words              |
| 07     | question   | AVAILABLE?, CONFIRMED?, etc        |
| 08     | answer     | YES_AVAILABLE, NO_UNAVAILABLE, etc |

### Words vs Phrases

**Words** are single concept opcodes:
```
HELLO, ACK, YES, NO, ACCEPT, URGENT
```

**Phrases** are complete meaning in one transaction:
```
MEET_TOMORROW_1400  =  "Meeting requested tomorrow at 14:00"
```

A phrase saves multiple transactions — the full meaning is encoded in one opcode.

### Question / Answer

Questions in family 07 are linked to answers in family 08.
A UI can use this to automatically surface the correct answer options
when a question opcode is received.
```
Q  0.00700010  AVAILABLE?       Are you available?
   A  0.00800010  YES_AVAILABLE    Yes I am available
   A  0.00800110  NO_UNAVAILABLE   No I am not available
```

## Example Conversation

Raw ledger view — what appears on chain:
```
SNAP_A1xxx  ->  SNAP_B1xxx  0.00100010
SNAP_B1xxx  ->  SNAP_A1xxx  0.00100020
SNAP_A1xxx  ->  SNAP_B1xxx  0.00700010
SNAP_B1xxx  ->  SNAP_A1xxx  0.00800010
SNAP_A1xxx  ->  SNAP_B1xxx  0.00300210
SNAP_B1xxx  ->  SNAP_A1xxx  0.00200030
SNAP_B1xxx  ->  SNAP_A1xxx  0.00400060
SNAP_A1xxx  ->  SNAP_B1xxx  0.00400080
```

Decoded conversation view:
```
SNAP_A1xxx  ->  SNAP_B1xxx  [HANDSHAKE]  Initiate contact
SNAP_B1xxx  ->  SNAP_A1xxx  [HANDSHAKE]  Received and understood
SNAP_A1xxx  ->  SNAP_B1xxx  [QUESTION]   Are you available?
SNAP_B1xxx  ->  SNAP_A1xxx  [ANSWER]     Yes I am available
SNAP_A1xxx  ->  SNAP_B1xxx  [SCHEDULING] Meeting requested tomorrow at 14:00
SNAP_B1xxx  ->  SNAP_A1xxx  [RESPONSE]   Approve the prior request
SNAP_B1xxx  ->  SNAP_A1xxx  [WORKFLOW]   Job is complete
SNAP_A1xxx  ->  SNAP_B1xxx  [WORKFLOW]   Payment has been sent
```

Same transactions. Two perspectives.

## Usage
```rust
use snap_coin_opcode::{Dictionary, Compiler, Decoder};

let dict = Dictionary::load("dictionary/dictionary.json").unwrap();
let compiler = Compiler::new(&dict);
let decoder = Decoder::new(&dict);

// encode
let compiled = compiler.compile(&["HELLO"]).unwrap();
// compiled.amounts -> pass directly to snap-coin-pay submit_withdrawal()

// decode
let decoded = decoder.decode_message(&compiled.amounts);
for opcode in decoded.opcodes {
    println!("{}", opcode.meaning);
}
```

## Examples
```bash
cargo run --example basic_convo       # raw ledger + decoded conversation
cargo run --example dict_inspect      # all opcodes grouped by family
cargo run --example phrase_lookup     # all phrase opcodes
cargo run --example question_answer   # question and answer pairs
```

## Project Structure
```
snap-coin-opcode/
├── Cargo.toml
├── dictionary/
│   └── dictionary.json        # SOT - all opcodes, families, meanings
└── src/
    ├── lib.rs                 # public API
    ├── dictionary.rs          # load and query dictionary
    ├── compiler.rs            # tokens -> opcode amounts
    └── decoder.rs             # amounts -> tokens
```

## Integration

This library is designed to be used as a dependency by:

- `snap-msg` — Axum/WebSocket communication app
- SNAP mobile wallet — encode/decode opcodes natively
- `snapcoin-db-inspector` — opcode scan mode for chain history

The dictionary file is the only shared dependency between all consumers.

## Tests
```bash
cargo test
```

## License

MIT
