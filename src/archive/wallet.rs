use fastcrypto::{traits::{KeyPair, EncodeDecodeBase64}};
use rand::thread_rng;
use fastcrypto::ed25519::*;
use std::fs::File;
use std::io::prelude::*;
use sui_types::{
  base_types::SuiAddress,
  crypto::{SuiKeyPair}
};
pub struct Wallet {
  public_key: SuiAddress,
  keypair_b64: String,
}
pub trait WalletManipulator {
  fn generate_wallets(&self, num_wallets: u64) -> Result<Vec<Wallet>, anyhow::Error>;

  fn dump_wallets_to_csv(&self, wallets: &Vec<Wallet>, filename: &str) -> Result<(), anyhow::Error>;

  fn dispense_tokens(&self, wallet: &str, token: &str, amount: u64) -> Result<(), anyhow::Error>;
}

pub struct WalletService;

impl WalletManipulator for WalletService {
  fn generate_wallets(&self, num_wallets: u64) -> Result<Vec<Wallet>, anyhow::Error> {
    let mut wallets = Vec::new();
    for _ in 0..num_wallets {
      // create Ed25519 keypair
      let kp = Ed25519KeyPair::generate(&mut thread_rng());
      let public_key: SuiAddress = kp.public().into();
      let keypair = SuiKeyPair::Ed25519(kp);
      let wallet = Wallet {
          public_key,
          keypair_b64: keypair.encode_base64(),
      };
      wallets.push(wallet);
    }
    Ok(wallets)
  }
  fn dump_wallets_to_csv(&self, wallets: &Vec<Wallet>, filename: &str) -> Result<(), anyhow::Error> {
    // check file exists and create if not
    let mut file =  if !std::path::Path::new(filename).exists() {
      File::create(filename)?
    } else {
      File::open(filename)?
    };

    file.write_all(b"Public Key,Private Key\n")?;
    for wallet in wallets {
      let line = format!("{},{}\n", wallet.public_key, wallet.keypair_b64);
      file.write_all(line.as_bytes())?;
    }
    Ok(())
  }

  fn dispense_tokens(&self, wallet: &str, token: &str, amount: u64) -> Result<(), anyhow::Error> {
    Ok(())
  }
}






