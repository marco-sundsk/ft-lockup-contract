use crate::*;
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};

const FT_WASM: &str = "res/fungible_token.wasm";
const LOCKUP_WASM: &str = "res/ft_lockup.wasm";


pub async fn deploy_lockup(
    root: &Account,
    token_id: &AccountId,
    owner_id: &AccountId,
    draft_operator_id: &AccountId,
) -> Result<LockupContract> {
    let lockup = root
        .create_subaccount("ft-lockup")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .unwrap();
    let lockup = lockup
        .deploy(&std::fs::read(LOCKUP_WASM).unwrap())
        .await?
        .unwrap();
    assert!(lockup.call("new")
        .args_json(json!({
            "token_account_id": token_id,
            "deposit_whitelist": [owner_id,],
            "draft_operators_whitelist": [draft_operator_id,],
        }))
        .max_gas()
        .transact()
        .await?
        .is_success());
    Ok(LockupContract(lockup))
}


pub async fn deploy_ft(
    root: &Account,
    owner: &Account,
    symbol: &str,
    decimal: u8,
) -> Result<FtContract> {

    let mock_ft = root
        .create_subaccount(symbol)
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    let mock_ft = mock_ft
        .deploy(&std::fs::read(FT_WASM).unwrap())
        .await?
        .unwrap();
    assert!(mock_ft
        .call("new")
        .args_json(json!({
            "owner_id": owner.id(),
            "total_supply": U128::from(d(1_000_000, decimal)),
            "metadata": FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: symbol.to_string(),
                symbol: symbol.to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: decimal,
            }
        }))
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .is_success());
    Ok(FtContract(mock_ft))
}
