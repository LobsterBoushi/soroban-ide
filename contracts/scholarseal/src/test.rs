#![cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};
    use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};

    fn setup_env<'a>() -> (Env, VaultScholarsContractClient<'a>, Address, Address, TokenClient<'a>, StellarAssetClient<'a>) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, VaultScholarsContract);
        let client = VaultScholarsContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let student = Address::generate(&env);

        // Setup mock USDC token
        let token_admin = Address::generate(&env);
        let token_contract = env.register_stellar_asset_contract(token_admin.clone());
        let token_client = TokenClient::new(&env, &token_contract);
        let token_admin_client = StellarAssetClient::new(&env, &token_contract);

        (env, client, admin, student, token_client, token_admin_client)
    }

    // Test 1 (Happy path): The MVP transaction executes successfully end-to-end
    #[test]
    fn test_request_document_success() {
        let (env, client, admin, student, token_client, token_admin) = setup_env();
        let fee = 20000000; // 2 USDC (7 decimals)
        
        client.initialize(&admin, &token_client.address, &fee);
        token_admin.mint(&student, &50000000); // Give student 5 USDC

        let doc_hash = BytesN::from_array(&env, &[1; 32]);
        client.request_document(&student, &doc_hash);

        assert_eq!(token_client.balance(&student), 30000000);
        assert_eq!(token_client.balance(&admin), 20000000);
    }

    // Test 2 (Edge case): Insufficient balance prevents document storage
    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn test_insufficient_balance() {
        let (env, client, admin, student, token_client, _token_admin) = setup_env();
        let fee = 20000000;
        
        client.initialize(&admin, &token_client.address, &fee);
        // Student has 0 balance. This should panic during transfer.
        let doc_hash = BytesN::from_array(&env, &[2; 32]);
        client.request_document(&student, &doc_hash);
    }

    // Test 3 (State verification): Assert that contract storage reflects the correct state
    #[test]
    fn test_verify_document_state() {
        let (env, client, admin, student, token_client, token_admin) = setup_env();
        let fee = 20000000;
        
        client.initialize(&admin, &token_client.address, &fee);
        token_admin.mint(&student, &50000000);

        let doc_hash = BytesN::from_array(&env, &[3; 32]);
        client.request_document(&student, &doc_hash);

        // State verify: Document exists and belongs to student
        let owner = client.verify_document(&doc_hash).unwrap();
        assert_eq!(owner, student);

        // State verify: Unknown document returns None
        let fake_hash = BytesN::from_array(&env, &[9; 32]);
        assert!(client.verify_document(&fake_hash).is_none());
    }

    // Test 4 (Edge case): Prevent duplicate document requests
    #[test]
    #[should_panic(expected = "Document already exists")]
    fn test_duplicate_document_hash() {
        let (env, client, admin, student, token_client, token_admin) = setup_env();
        let fee = 20000000;
        
        client.initialize(&admin, &token_client.address, &fee);
        token_admin.mint(&student, &50000000);

        let doc_hash = BytesN::from_array(&env, &[4; 32]);
        
        client.request_document(&student, &doc_hash); // First call succeeds
        client.request_document(&student, &doc_hash); // Second call should panic
    }

    // Test 5 (Edge case): Uninitialized contract
    #[test]
    #[should_panic(expected = "Not initialized")]
    fn test_uninitialized_contract() {
        let (env, client, _admin, student, _token_client, _token_admin) = setup_env();
        
        // Calling request_document without initializing the contract first
        let doc_hash = BytesN::from_array(&env, &[5; 32]);
        client.request_document(&student, &doc_hash);
    }
}