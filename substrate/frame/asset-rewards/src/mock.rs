// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test environment for Staking Rewards pallet.

use super::*;
use crate as pallet_staking_rewards;
use core::default::Default;
use frame_support::{
	construct_runtime, derive_impl,
	instances::Instance1,
	ord_parameter_types, parameter_types,
	traits::{
		tokens::fungible::{NativeFromLeft, NativeOrWithId, UnionOf},
		AsEnsureOriginWithArg, ConstU128, ConstU32, EnsureOrigin,
	},
	PalletId,
};
use frame_system::{ensure_signed, EnsureSigned};
use sp_runtime::{
	traits::{AccountIdConversion, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<MockRuntime>;

construct_runtime!(
	pub enum MockRuntime
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets::<Instance1>,
		StakingRewards: pallet_staking_rewards,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for MockRuntime {
	type AccountId = u128;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u128>;
}

impl pallet_balances::Config for MockRuntime {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<100>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
}

impl pallet_assets::Config<Instance1> for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU128<1>;
	type AssetAccountDeposit = ConstU128<10>;
	type MetadataDepositBase = ConstU128<1>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	pallet_assets::runtime_benchmarks_enabled! {
		type BenchmarkHelper = ();
	}
}

parameter_types! {
	pub const StakingRewardsPalletId: PalletId = PalletId(*b"py/stkrd");
	pub const Native: NativeOrWithId<u32> = NativeOrWithId::Native;
	pub const PermissionedAccountId: u128 = 1;
}
ord_parameter_types! {
	pub const AssetConversionOrigin: u128 = AccountIdConversion::<u128>::into_account_truncating(&StakingRewardsPalletId::get());
}

// Set account id 1 to the permissioned creator
pub struct MockPermissionedPoolCreator;
impl EnsureOrigin<RuntimeOrigin> for MockPermissionedPoolCreator {
	type Success = ();

	fn try_origin(origin: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		// Set account 1 to admin in tests
		if ensure_signed(origin.clone()).map_or(false, |acc| acc == 1) {
			return Ok(());
		}

		return Err(origin);
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		todo!()
	}
}

pub type NativeAndAssets = UnionOf<Balances, Assets, NativeFromLeft, NativeOrWithId<u32>, u128>;

impl Config for MockRuntime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = NativeOrWithId<u32>;
	type Balance = <Self as pallet_balances::Config>::Balance;
	type Assets = NativeAndAssets;
	type PalletId = StakingRewardsPalletId;
	type PermissionedPoolCreator = MockPermissionedPoolCreator;
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<MockRuntime>::default().build_storage().unwrap();

	pallet_assets::GenesisConfig::<MockRuntime, Instance1> {
		// Genesis assets: id, owner, is_sufficient, min_balance
		// pub assets: Vec<(T::AssetId, T::AccountId, bool, T::Balance)>,
		assets: vec![(1, 1, true, 10000)],
		// Genesis metadata: id, name, symbol, decimals
		// pub metadata: Vec<(T::AssetId, Vec<u8>, Vec<u8>, u8)>,
		metadata: vec![(1, b"test".to_vec(), b"TST".to_vec(), 18)],
		// Genesis accounts: id, account_id, balance
		// pub accounts: Vec<(T::AssetId, T::AccountId, T::Balance)>,
		accounts: vec![
			(1, 1, 10000),
			(1, 2, 20000),
			(1, 3, 30000),
			(1, 4, 40000),
			(1, 10, 40000),
			(1, 20, 40000),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let pool_zero_account_id = 31086825966906540362769395565;
	pallet_balances::GenesisConfig::<MockRuntime> {
		balances: vec![
			(1, 10000),
			(2, 20000),
			(3, 30000),
			(4, 40000),
			(10, 40000),
			(20, 40000),
			(pool_zero_account_id, 100_000), // Top up the default pool account id
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
