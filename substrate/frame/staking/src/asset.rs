//! Facade of currency implementation. Useful while migrating from old to new currency system.

use frame_support::{
	defensive, ensure,
	traits::{Defensive, InspectLockableCurrency, LockableCurrency},
};
use sp_staking::{StakingAccount, StakingInterface};

use crate::{
	BalanceOf, Bonded, Config, Error, Ledger, Pallet, Payee, RewardDestination, StakingLedger,
	VirtualStakers, STAKING_ID,
};

/// Balance that is staked and at stake.
pub fn staked<T: Config>(who: &T::AccountId) -> BalanceOf<T> {
	T::Currency::balance_locked(crate::STAKING_ID, who)
}