// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn create_voters_snapshot_paged(t: u32) -> Weight;
	fn create_targets_snapshot_paged(v: u32) -> Weight;
	fn on_initialize_start_signed() -> Weight;
	fn on_initialize_do_nothing() -> Weight;
	fn on_phase_transition() -> Weight;
	fn on_initialize_start_export() -> Weight;
}

/// Weight functions for `pallet_epm_core`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: `Staking::CounterForValidators` (r:1 w:0)
	/// Proof: `Staking::CounterForValidators` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::TargetSnapshotStatus` (r:1 w:1)
	/// Proof: `Staking::TargetSnapshotStatus` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Validators` (r:2049 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `ElectionProviderMultiBlock::PagedTargetSnapshot` (r:0 w:1)
	/// Proof: `ElectionProviderMultiBlock::PagedTargetSnapshot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[512, 2048]`.
	fn create_targets_snapshot_paged(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1041 + t * (46 ±0)`
		//  Estimated: `3510 + t * (2520 ±0)`
		// Minimum execution time: 47_198_000_000 picoseconds.
		Weight::from_parts(3_209_333_333, 0)
			.saturating_add(Weight::from_parts(0, 3510))
			// Standard Error: 1_207_323
			.saturating_add(Weight::from_parts(86_960_937, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 2520).saturating_mul(t.into()))
	}
	/// Storage: `VoterList::CounterForListNodes` (r:1 w:0)
	/// Proof: `VoterList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::VoterSnapshotStatus` (r:1 w:1)
	/// Proof: `Staking::VoterSnapshotStatus` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `VoterList::ListBags` (r:200 w:0)
	/// Proof: `VoterList::ListBags` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `VoterList::ListNodes` (r:1025 w:0)
	/// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Bonded` (r:1024 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Ledger` (r:1024 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1091), added: 3566, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Nominators` (r:1024 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Validators` (r:1000 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `Staking::MinimumActiveStake` (r:0 w:1)
	/// Proof: `Staking::MinimumActiveStake` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `ElectionProviderMultiBlock::PagedVoterSnapshot` (r:0 w:1)
	/// Proof: `ElectionProviderMultiBlock::PagedVoterSnapshot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `v` is `[32, 1024]`.
	fn create_voters_snapshot_paged(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `73175 + v * (946 ±0)`
		//  Estimated: `512390 + v * (3566 ±0)`
		// Minimum execution time: 13_398_000_000 picoseconds.
		Weight::from_parts(4_906_354_838, 0)
			.saturating_add(Weight::from_parts(0, 512390))
			// Standard Error: 534_281
			.saturating_add(Weight::from_parts(260_582_661, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(208))
			.saturating_add(T::DbWeight::get().reads((5_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 3566).saturating_mul(v.into()))
	}
	/// Storage: `ElectionProviderMultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `ElectionProviderMultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionProviderMultiBlock::Round` (r:1 w:0)
	/// Proof: `ElectionProviderMultiBlock::Round` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ElectionDataLock` (r:0 w:1)
	/// Proof: `Staking::ElectionDataLock` (`max_values`: Some(1), `max_size`: Some(0), added: 495, mode: `MaxEncodedLen`)
	fn on_initialize_start_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1561`
		// Minimum execution time: 66_000_000 picoseconds.
		Weight::from_parts(66_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1561))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `ElectionProviderMultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `ElectionProviderMultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionProviderMultiBlock::Round` (r:1 w:0)
	/// Proof: `ElectionProviderMultiBlock::Round` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_phase_transition() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1561`
		// Minimum execution time: 62_000_000 picoseconds.
		Weight::from_parts(62_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1561))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn on_initialize_start_export() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::CurrentPlannedSession` (r:1 w:0)
	/// Proof: `Staking::CurrentPlannedSession` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ErasStartSessionIndex` (r:1 w:0)
	/// Proof: `Staking::ErasStartSessionIndex` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ForceEra` (r:1 w:0)
	/// Proof: `Staking::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `ElectionProviderMultiBlock::CurrentPhase` (r:1 w:0)
	/// Proof: `ElectionProviderMultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize_do_nothing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `502`
		//  Estimated: `3481`
		// Minimum execution time: 111_000_000 picoseconds.
		Weight::from_parts(111_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3481))
			.saturating_add(T::DbWeight::get().reads(5))
	}
}

impl WeightInfo for () {
	fn create_voters_snapshot_paged(_v: u32) -> Weight {
	    Default::default()
	}

	fn create_targets_snapshot_paged(_t: u32) -> Weight {
		Default::default()
	}

	fn on_initialize_start_signed() -> Weight {
		Default::default()
	}

	fn on_initialize_do_nothing() -> Weight {
		Default::default()
	}

	fn on_phase_transition() -> Weight {
		Default::default()
	}

	fn on_initialize_start_export() -> Weight {
		Default::default()
	}
}