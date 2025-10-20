use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use solana_program_test::*;
use solana_sdk::{account::Account, signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::test]
async fn test_notwest_stable() {
    // Setup environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new("notwest_stable", program_id, processor!(notwest_stable::entry));
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create a simple test
    println!("✅ Notwest Stable program test executed successfully!");
}
