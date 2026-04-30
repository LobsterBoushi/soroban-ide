# VaultScholars

**A decentralized school document management system combining credential anchoring with automated micro-payments.**

## Problem & Solution
**Problem:** Graduates in developing nations wait weeks and pay manual cash fees to get verifiable transcripts, while employers struggle with credential fraud. 
**Solution:** VaultScholars allows students to pay a tiny USDC micro-fee via a Soroban smart contract, instantly routing payment to the school and anchoring a tamper-proof digital transcript hash on the Stellar blockchain for instant employer verification.

## Timeline
Can be successfully built, tested, and demoed within a 48-hour hackathon timeframe.

## Stellar Features Used
* Soroban Smart Contracts (escrow/logic execution)
* USDC Transfers (fast, low-cost B2B/B2C micro-payments)
* Stellar Network Storage (immutable document hashes)

## Vision and Purpose
To eradicate credential fraud and administrative friction in Southeast Asia's education sector by leveraging Stellar's low fees and instant settlement.

## Deployed Contract Details
[1] https://stellar.expert/explorer/testnet/tx/31c098bae0a2e5b71219c35224e8b6082de129d05f354165645638a19ba6d2d8
[2] https://stellar.expert/explorer/testnet/tx/31c098bae0a2e5b71219c35224e8b6082de129d05f354165645638a19ba6d2d8

## Prerequisites
* Rust (latest stable version)
* `soroban-cli` v20.0.0 or higher
* Target `wasm32-unknown-unknown` installed (`rustup target add wasm32-unknown-unknown`)

## Commands

**How to build:**
```bash
soroban contract build