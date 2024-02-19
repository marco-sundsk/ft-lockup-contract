use std::str::FromStr;

use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DraftGroupFunding {
    pub draft_group_id: DraftGroupIndex,
    // use remaining gas to try converting drafts
    pub try_convert: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum FtMessage {
    LockupCreate(LockupCreate),
    DraftGroupFunding(DraftGroupFunding),
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(
            env::predecessor_account_id(),
            self.token_account_id,
            "Invalid token ID"
        );
        let amount = amount.into();
        self.assert_deposit_whitelist(&sender_id);

        let ft_message: FtMessage = serde_json::from_str(&msg).unwrap();
        match ft_message {
            FtMessage::LockupCreate(lockup_create) => {
                let lockup = lockup_create.into_lockup(&sender_id);
                lockup.assert_new_valid(amount);
                let index = self.internal_add_lockup(&lockup);
                log!(
                    "Created new lockup for {} with index {}",
                    lockup.account_id,
                    index
                );
                let event: FtLockupCreateLockup = (index, lockup, None).into();
                emit(EventKind::FtLockupCreateLockup(vec![event]));
            }
            FtMessage::DraftGroupFunding(funding) => {
                let draft_group_id = funding.draft_group_id;
                let mut draft_group = self
                    .draft_groups
                    .get(&draft_group_id as _)
                    .expect("draft group not found");
                assert_eq!(
                    draft_group.total_amount, amount,
                    "The draft group total balance doesn't match the transferred balance",
                );
                draft_group.fund(&sender_id);
                self.draft_groups.insert(&draft_group_id as _, &draft_group);
                log!("Funded draft group {}", draft_group_id);

                if funding.try_convert.unwrap_or(false) {
                    // Using remaining gas to try convert drafts, not waiting for results
                    if let Some(remaining_gas) =
                        env::prepaid_gas().0.checked_sub(env::used_gas().0 + GAS_EXT_CALL_COST.0)
                    {
                        if remaining_gas > GAS_MIN_FOR_CONVERT.0 {
                            Self::ext(env::current_account_id())
                                .with_static_gas(remaining_gas.into())
                                .convert_drafts(
                                draft_group.draft_indices.into_iter().collect(),
                            );
                        }
                    }
                }
                let event = FtLockupFundDraftGroup {
                    id: draft_group_id,
                    amount: amount.into(),
                };
                emit(EventKind::FtLockupFundDraftGroup(vec![event]));
            }
        }

        PromiseOrValue::Value(0.into())
    }
}

#[test]
fn do_serial_and_unserial() {
    let lockup_create = LockupCreate {
        account_id: AccountId::from_str("alice.near").unwrap(),
        schedule: Schedule(vec![
            Checkpoint {
                timestamp: 1000,
                balance: 0,
            },
            Checkpoint {
                timestamp: 2000,
                balance: 500,
            },
        ]),
        vesting_schedule: None,
    };
    println!(
        "{}",
        near_sdk::serde_json::to_string(&lockup_create).unwrap()
    );
}