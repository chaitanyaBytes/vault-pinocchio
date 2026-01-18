use litesvm::LiteSVM;

use solana_sdk::{
    message::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

#[test]
pub fn test_pinocchio_vault() {
    let mut svm = LiteSVM::new();

    let owner = Keypair::new();
    svm.airdrop(&owner.pubkey(), 2 * LAMPORTS_PER_SOL)
        .expect("failed to airdrop");

    let program_id_bytes: [u8; 32] = [
        0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07, 0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb,
        0xee, 0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07, 0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc,
        0x11, 0xf7,
    ];
    let program_id = Pubkey::from(program_id_bytes);
    let program_bytes = include_bytes!("../../target/deploy/vault_pinocchio.so");

    svm.add_program(program_id, program_bytes)
        .expect("failed to add program");

    let (vault, _) =
        Pubkey::find_program_address(&[b"vault", owner.pubkey().as_ref()], &program_id);

    // deposit instruction
    let amount = 1 * LAMPORTS_PER_SOL;

    let mut ix_data = vec![];
    ix_data.push(0);
    ix_data.extend_from_slice(&amount.to_le_bytes());

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(owner.pubkey(), true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
        data: ix_data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);

    if let Ok(response) = &result {
        let logs = response.pretty_logs();
        println!("Transaction logs:\n{}", logs);
    } else if let Err(e) = &result {
        // Try to get logs from error if available
        eprintln!("Transaction failed: {:?}", e);
    }

    let mut ix_data = vec![];
    ix_data.push(1);

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(owner.pubkey(), true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
        data: ix_data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);

    if let Ok(response) = &result {
        let logs = response.pretty_logs();
        println!("Transaction logs:\n{}", logs);
    } else if let Err(e) = &result {
        // Try to get logs from error if available
        eprintln!("Transaction failed: {:?}", e);
    }
}
