#[cfg(test)]
mod tests {
    // Solana Client and SDK imports
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signer::{keypair::Keypair, Signer},
        transaction::Transaction,
    };
    // Import for reading keypair files
    use solana_sdk::signer::keypair::read_keypair_file;
    
    // Solana Program imports (for system_program::id())
    use solana_program::system_program; 
    
    // Standard library imports
    use std::str::FromStr;

    #[test]
    fn submit_ts() {
        let rpc_client = RpcClient::new("https://api.devnet.solana.com");
        let signer = read_keypair_file("personal_wallet.json").expect("Couldn't find wallet file");
        let mint = Keypair::new();
        // Corrected from_Str to from_str
        let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        // Corrected fromStr to from_str
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program_id = system_program::id(); // Use system_program::id()

        // Declare the 'authority' Pubkey.
        // Based on the provided JSON IDL snippet, the PDA for 'authority'
        // is derived using the string "collection" and the `collection` Pubkey itself as seeds.
        let (authority, _bump) = Pubkey::find_program_address(&[b"collection", collection.as_ref()], &turbin3_prereq_program);
    
        // the PDA steps here
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
        let data = vec![ 77, 124, 82, 163, 21, 133, 181, 206];
        let accounts = vec![
            AccountMeta::new(signer_pubkey, true),
            AccountMeta::new(prereq_pda, false),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program_id, false), // Use system_program_id
        ];

        // Get blockhash
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        // the instruction building
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        // tXn and signing it
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer_pubkey),
            &[&signer, &mint],
            blockhash,
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here: \nhttps://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }
}
