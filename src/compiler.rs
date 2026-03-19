// -----------------------------------------------------------------------------
// File: src/compiler.rs
// Project: snap-coin-opcode
// Description: Encode a list of tokens into opcode amounts for snap-coin-pay
// Version: 0.2.0
// -----------------------------------------------------------------------------

use crate::dictionary::Dictionary;

#[derive(Debug, Clone)]
pub struct CompiledMessage {
    pub amounts: Vec<u64>,
    pub tokens: Vec<String>,
}

pub struct Compiler<'a> {
    dictionary: &'a Dictionary,
}

impl<'a> Compiler<'a> {
    pub fn new(dictionary: &'a Dictionary) -> Self {
        Self { dictionary }
    }

    pub fn compile(&self, tokens: &[&str]) -> Result<CompiledMessage, String> {
        let mut amounts: Vec<u64> = Vec::new();
        let mut resolved_tokens: Vec<String> = Vec::new();

        for token in tokens {
            let entry = self.dictionary.lookup_token(token).ok_or_else(|| {
                format!("Unknown token: {}", token)
            })?;

            let amount = parse_amount(&entry.amount)?;
            amounts.push(amount);
            resolved_tokens.push(token.to_string());
        }

        Ok(CompiledMessage {
            amounts,
            tokens: resolved_tokens,
        })
    }
}

fn parse_amount(amount_str: &str) -> Result<u64, String> {
    let parts: Vec<&str> = amount_str.split('.').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid amount format: {}", amount_str));
    }

    let decimals = parts[1];
    if decimals.len() != 8 {
        return Err(format!(
            "Amount must have exactly 8 decimal places: {}",
            amount_str
        ));
    }

    decimals
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse amount {}: {}", amount_str, e))
}

// -----------------------------------------------------------------------------
// File: src/compiler.rs
// Project: snap-coin-opcode
// Created: 2026-03-19
// -----------------------------------------------------------------------------