// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_indices`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-vcatxqpx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("rococo-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_indices
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_indices`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_indices::WeightInfo for WeightInfo<T> {
	/// Storage: `Indices::Accounts` (r:1 w:1)
	/// Proof: `Indices::Accounts` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3534`
		// Minimum execution time: 22_085_000 picoseconds.
		Weight::from_parts(22_982_000, 0)
			.saturating_add(Weight::from_parts(0, 3534))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Indices::Accounts` (r:1 w:1)
	/// Proof: `Indices::Accounts` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `203`
		//  Estimated: `3593`
		// Minimum execution time: 38_568_000 picoseconds.
		Weight::from_parts(39_557_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Indices::Accounts` (r:1 w:1)
	/// Proof: `Indices::Accounts` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn free() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `3534`
		// Minimum execution time: 22_958_000 picoseconds.
		Weight::from_parts(23_613_000, 0)
			.saturating_add(Weight::from_parts(0, 3534))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Indices::Accounts` (r:1 w:1)
	/// Proof: `Indices::Accounts` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `203`
		//  Estimated: `3593`
		// Minimum execution time: 25_899_000 picoseconds.
		Weight::from_parts(26_826_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Indices::Accounts` (r:1 w:1)
	/// Proof: `Indices::Accounts` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `3534`
		// Minimum execution time: 26_296_000 picoseconds.
		Weight::from_parts(26_820_000, 0)
			.saturating_add(Weight::from_parts(0, 3534))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
