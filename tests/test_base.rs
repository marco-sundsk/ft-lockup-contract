pub use workspaces::{network::Sandbox, Account, AccountId, Contract, Worker, result::{Result, ExecutionFinalResult}};

mod common;

use crate::common::*;

pub async fn init(root: &Account, deposit_whitelist: Option<Vec<AccountId>>) -> Result<(Account, Account, FtContract, LockupContract)> {
    let owner = tool_create_account(&root, "owner", None).await;
    let draft_operator = tool_create_account(&root, "draft_operator", None).await;
    let token_contract = deploy_ft(&root, &owner, "token", 18).await?;   
    let lockup_contract = deploy_lockup(&root, token_contract.0.id(), owner.id(), draft_operator.id()).await?;
    check!(token_contract.ft_storage_deposit(lockup_contract.0.id()));
    Ok((owner, draft_operator, token_contract, lockup_contract))
}

#[tokio::test]
async fn wtest_base() -> Result<()> {
    let worker = workspaces::sandbox().await?;
    let root = worker.root_account()?;
    let (owner, draft_operator, token_contract, lockup_contract) = init(&root, None).await?;
    check!(view token_contract.ft_balance_of(&owner));

    // let owner = tool_create_account(&root, "owner", None).await;
    // let draft_operator = tool_create_account(&root, "draft_operator", None).await;

    // let token_contract = deploy_ft(&root, &owner, "token", 18).await?;
    // check!(view token_contract.ft_balance_of(&owner));
    // let lockup_contract = deploy_lockup(&root, token_contract.0.id(), owner.id(), draft_operator.id()).await?;
    // check!(token_contract.ft_storage_deposit(lockup_contract.0.id()));
    Ok(())
}
