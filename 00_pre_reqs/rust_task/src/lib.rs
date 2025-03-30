
#[allow(unused_imports)]
use solana_client::rpc_client::RpcClient;
mod programs;
use solana_program::instruction::{AccountMeta, Instruction};
use std::io::BufRead;

\use solana_program::{
    hash::hash,
    pubkey::Pubkey,
    system_instruction::transfer,
    system_program,
};
const PROGRAM_ID: &str = "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa";

use solana_sdk::{
    message::Message,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;  use solana_client::rpc_client::RpcClient;
    use solana_program::system_program;
    use solana_sdk::{
        signature::{read_keypair_file, Signer},
        transaction::Transaction
    };

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn check_balance() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        let balance = client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
        println!("Balance right now isssssss: {} lamports", balance);
    }

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = std::io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = std::io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap() 
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap()) 
            .collect::<Vec<u8>>(); 
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        println!("Loaded Public Key: {}", keypair.pubkey());
    
        let to_pubkey_result = Pubkey::from_str("2yTTSEX7vSj3a218SSmyk3eg9GQN7e8UbwyEJ77SdkvV");
        let to_pubkey = match to_pubkey_result {
            Ok(pubkey) => pubkey,
            Err(_) => panic!("Invalid public key. Please check the format and try again."),
        };
    
        let rpc_client = RpcClient::new(RPC_URL);
    
        let message_bytes = b"I verify my solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        match sig.verify(&keypair.pubkey().to_bytes(), message_bytes) {
            true => println!("Signature verified"),
            false => panic!("Verification failed"),
        }
    
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
    
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
    
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
    
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn submit_completion() {
        // Load wallet
        let signer = read_keypair_file("Turbin3-wallet.json")
            .expect("Couldn't find wallet file");
        
        // Derive PDA
        let (prereq, _bump) = Pubkey::find_program_address(
            &[b"prereq", signer.pubkey().as_ref()],
            &Pubkey::from_str(PROGRAM_ID).unwrap()
        );
    
        // Instruction data:
        // 1. 8-byte discriminator (from IDL)
        // 2. GitHub username (length-prefixed)
        let mut data = vec![0, 77, 224, 147, 136, 25, 88, 76]; // Discriminator
        let github = b"IshaanXCoder";
        data.extend_from_slice(&(github.len() as u32).to_le_bytes());
        data.extend_from_slice(github);
    
        let instruction = Instruction {
            program_id: Pubkey::from_str(PROGRAM_ID).unwrap(),
            accounts: vec![
                AccountMeta::new(signer.pubkey(), true),  // signer
                AccountMeta::new(prereq, false),          // prereq account
                AccountMeta::new_readonly(system_program::id(), false), // system program
            ],
            data,
        };
    
        // Create transaction
        let rpc_client = RpcClient::new(RPC_URL);
        let blockhash = rpc_client.get_latest_blockhash().unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );
    
        // Send transaction
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(sig) => println!("✅ Success! TX: https://explorer.solana.com/tx/{}?cluster=devnet", sig),
            Err(e) => {
                let balance = rpc_client.get_balance(&signer.pubkey()).unwrap_or(0);
                panic!("❌ Failed: {:?}\nCheck:\n1. Wallet balance (current: {} lamports)\n2. Program ID: {}\n3. GitHub username: 'IshaanXCoder'\n4. PDA derivation", 
                    e, balance, PROGRAM_ID);
            }
        };
    }
}    