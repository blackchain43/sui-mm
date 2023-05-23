use sui_sdk::{
  SuiClient,
  SuiClientBuilder,
  wallet_context::WalletContext,
};
use sui_config::{sui_config_dir, SUI_CLIENT_CONFIG};

pub struct SuiService {
  wallet: WalletContext,
  sui_provider: SuiClient,
}

impl SuiService {
  pub async fn new() -> Self {
    let sui_provider = SuiClientBuilder::default().build(
      "https://fullnode.devnet.sui.io:443",
    ).await.unwrap();
    // Instantiate wallet context
    let mut config_path = sui_config_dir().unwrap();
    config_path.push(SUI_CLIENT_CONFIG);
    let wallet = WalletContext::new(&config_path, Option::None, Option::None).await.unwrap();
    Self { sui_provider, wallet }
  }


  pub async fn perform_contract_call(&self) {
      // New SuiService
      let sui_service = SuiService::new().await;
      // Perform other contract calls or operations
      
  }
}
