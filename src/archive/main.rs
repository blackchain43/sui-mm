use std::env;

mod sui_client;
mod wallet;
mod utils;

use wallet::{WalletManipulator, WalletService};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a valid command.");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "generate_wallet" => {
            if args.len() < 3 {
                println!("Please provide the number of wallets to generate.");
                return;
            }
            let num_wallets = args[2].parse::<u64>().unwrap();
            let file_arg = args.iter().position(|arg| arg == "--file");
            let wallet_service = WalletService{};
            let wallets = wallet_service.generate_wallets(num_wallets).unwrap();
            if let Some(index) = file_arg {
                let file_path = args.get(index + 1).ok_or("Please provide a valid file path after --file.").unwrap();
                wallet_service.dump_wallets_to_csv(&wallets, file_path).unwrap();
                println!("Wallets generated and dumped to {}.", file_path);
            } else {
                println!("Wallets generated successfully.");
            }
        }
        "dispense_tokens" => {
            // if args.len() < 5 {
            //     println!("Please provide wallet address, token symbol, and amount.");
            //     return;
            // }
            // let wallet = &args[2];
            // let token = &args[3];
            // let amount = args[4].parse::<u64>().unwrap();

            // match wallet::dispense_tokens(wallet, token, amount) {
            //     Ok(_) => println!("Tokens dispensed successfully."),
            //     Err(err) => println!("Failed to dispense tokens: {:?}", err),
            // }
        }
        "swap" => {
            // if args.len() < 5 {
            //     println!("Please provide token X symbol, token Y symbol, and amount.");
            //     return;
            // }
            // let token_x = &args[2];
            // let token_y = &args[3];
            // let amount = args[4].parse::<u64>().unwrap();

            // match dex::swap(token_x, token_y, amount) {
            //     Ok(_) => println!("Swap executed successfully."),
            //     Err(err) => println!("Failed to execute swap: {:?}", err),
            // }
        }
        _ => println!("Invalid command."),
    }
}
