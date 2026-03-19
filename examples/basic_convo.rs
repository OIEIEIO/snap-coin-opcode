// -----------------------------------------------------------------------------
// File: examples/basic_convo.rs
// Project: snap-coin-opcode
// Description: Raw ledger view and decoded conversation view for 2 wallets
// Version: 0.3.0
// -----------------------------------------------------------------------------

use snap_coin_opcode::{Compiler, Decoder, Dictionary};

struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

fn format_amount(raw: u64) -> String {
    format!("0.{:08}", raw)
}

fn build_ledger(
    sender: &str,
    receiver: &str,
    tokens: &[&str],
    compiler: &Compiler,
) -> Vec<Transaction> {
    let compiled = compiler.compile(tokens).expect("compile failed");
    compiled
        .amounts
        .iter()
        .map(|&a| Transaction {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount: a,
        })
        .collect()
}

fn main() {
    let dict = Dictionary::load("dictionary/dictionary.json").expect("failed to load dictionary");
    let compiler = Compiler::new(&dict);
    let decoder = Decoder::new(&dict);

    let wallet_a = "SNAP_A1xxx";
    let wallet_b = "SNAP_B1xxx";

    // build full ledger
    let mut ledger: Vec<Transaction> = Vec::new();

    let messages: Vec<(&str, &str, &[&str])> = vec![
        (wallet_a, wallet_b, &["HELLO"]),
        (wallet_b, wallet_a, &["ACK"]),
        (wallet_a, wallet_b, &["AVAILABLE?"]),
        (wallet_b, wallet_a, &["YES_AVAILABLE"]),
        (wallet_a, wallet_b, &["MEET_TOMORROW_1400"]),
        (wallet_b, wallet_a, &["ACCEPT"]),
        (wallet_b, wallet_a, &["JOB_COMPLETE"]),
        (wallet_a, wallet_b, &["PAYMENT_SENT"]),
    ];

    for (sender, receiver, tokens) in &messages {
        let txs = build_ledger(sender, receiver, tokens, &compiler);
        ledger.extend(txs);
    }

    // raw ledger view
    println!("\n=== RAW LEDGER: {} <-> {} ===\n", wallet_a, wallet_b);
    for tx in &ledger {
        println!(
            "{}  ->  {}  {}",
            tx.sender,
            tx.receiver,
            format_amount(tx.amount)
        );
    }

    // decoded conversation view
    println!("\n=== DECODED CONVERSATION: {} <-> {} ===\n", wallet_a, wallet_b);
    for tx in &ledger {
        match decoder.decode_amount(tx.amount) {
            Some(opcode) => {
                println!(
                    "{}  ->  {}  [{}] {}",
                    tx.sender,
                    tx.receiver,
                    opcode.category.to_uppercase(),
                    opcode.meaning
                );
            }
            None => {
                println!(
                    "{}  ->  {}  unknown opcode {}",
                    tx.sender,
                    tx.receiver,
                    format_amount(tx.amount)
                );
            }
        }
    }

    println!("\n=== End ===\n");
}

// -----------------------------------------------------------------------------
// File: examples/basic_convo.rs
// Project: snap-coin-opcode
// Created: 2026-03-19
// -----------------------------------------------------------------------------