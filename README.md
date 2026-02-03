# Solana Escrow Program

A native Solana program implementing a simple escrow system using Program Derived Addresses (PDAs).

## Overview

This escrow program allows a maker to lock SOL in escrow for a taker. The maker can release funds to the taker or cancel and reclaim them.

## Features

- **InitEscrow**: Maker deposits SOL into a PDA-controlled escrow account
- **Release**: Maker releases funds to the taker
- **Cancel**: Maker cancels and gets a refund

## Program Structure

### State
\\ust
struct EscrowState {
    maker: Pubkey,      // Who created the escrow
    taker: Pubkey,      // Who receives the funds
    amount: u64,        // Amount in lamports
    is_released: bool,  // Whether funds were released
}
\
### Instructions
\\ust
enum EscrowInstruction {
    InitEscrow { amount: u64 },
    Release,
    Cancel,
}
\
## Key Concepts

### Program Derived Addresses (PDAs)
The escrow account is a PDA derived from:
\\ust
seeds: [b
