#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, BytesN, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Admin,           // Address of the school/registrar
    FeeToken,        // Address of the USDC token
    FeeAmount,       // i128 amount required to process a document
    Doc(BytesN<32>), // Maps document hash to the Student's Address
}

#[contract]
pub struct VaultScholarsContract;

#[contractimpl]
impl VaultScholarsContract {
    /// Initializes the contract with the school's admin address, the accepted token (e.g., USDC), and the fee.
    pub fn initialize(env: Env, admin: Address, fee_token: Address, fee_amount: i128) {
        assert!(!env.storage().instance().has(&DataKey::Admin), "Already initialized");
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::FeeToken, &fee_token);
        env.storage().instance().set(&DataKey::FeeAmount, &fee_amount);
    }

    /// MVP Feature: Student pays the fee and records their document hash on-chain.
    pub fn request_document(env: Env, student: Address, document_hash: BytesN<32>) {
        // 1. Require the student to authorize this transaction
        student.require_auth();

        // 2. Load contract configuration
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        let fee_token: Address = env.storage().instance().get(&DataKey::FeeToken).unwrap();
        let fee_amount: i128 = env.storage().instance().get(&DataKey::FeeAmount).unwrap();

        // 3. Prevent duplicate anchoring to avoid double-charging for the exact same document
        assert!(!env.storage().persistent().has(&DataKey::Doc(document_hash.clone())), "Document already exists");

        // 4. Execute the USDC payment from Student to School Admin
        let client = token::Client::new(&env, &fee_token);
        client.transfer(&student, &admin, &fee_amount);

        // 5. Store the document hash permanently, linked to the student
        env.storage().persistent().set(&DataKey::Doc(document_hash), &student);
    }

    /// Verifies if a document hash exists on-chain and returns the owner's address.
    pub fn verify_document(env: Env, document_hash: BytesN<32>) -> Option<Address> {
        env.storage().persistent().get(&DataKey::Doc(document_hash))
    }
}

mod test;