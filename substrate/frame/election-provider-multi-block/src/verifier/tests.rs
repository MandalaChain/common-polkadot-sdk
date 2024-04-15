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

use crate::mock::*;

mod solution_queued {
	use crate::verifier::impls::pallet::*;

	use super::*;

	#[test]
	fn variant_flipping_works() {
		ExtBuilder::verifier().build_and_execute(|| {
			assert!(QueuedSolution::<T>::valid() != QueuedSolution::<T>::invalid());
		})
	}
}

mod verifier {
	use super::*;
	use crate::{
		verifier::{impls::pallet::QueuedSolution, SolutionPointer},
		SupportsOf, Verifier,
	};

	#[test]
	fn sync_verification_works() {
		ExtBuilder::verifier().build_and_execute(|| {
			// no queued score or solution at the beginning.
			assert!(<VerifierPallet as Verifier>::queued_score().is_none());
			assert!(<VerifierPallet as Verifier>::get_queued_solution(0).is_none());

			//assert_ok!(<VerifierPallet as Verifier>::verify_synchronous(mine_solution(),
			// ElectionScore::default(), 0));
		})
	}

	#[test]
	fn next_missing_solution_works() {
		ExtBuilder::verifier().build_and_execute(|| {
			let supports: SupportsOf<VerifierPallet> = Default::default();
			let msp = crate::Pallet::<T>::msp();
			assert!(msp == <T as crate::Config>::Pages::get() - 1 && msp == 2);

			// msp page is the next missing.
			assert_eq!(<VerifierPallet as Verifier>::next_missing_solution_page(), Some(msp));

			// X is the current valid solution, let's work with it.
			assert_eq!(QueuedSolution::<T>::valid(), SolutionPointer::X);

			// set msp and check the next missing page again.
			QueuedSolution::<T>::set_page(msp, supports.clone());
			assert_eq!(<VerifierPallet as Verifier>::next_missing_solution_page(), Some(msp - 1));

			QueuedSolution::<T>::set_page(msp - 1, supports.clone());
			assert_eq!(<VerifierPallet as Verifier>::next_missing_solution_page(), Some(0));

			// set last page, missing page after is None as solution is complete.
			QueuedSolution::<T>::set_page(0, supports.clone());
			assert_eq!(<VerifierPallet as Verifier>::next_missing_solution_page(), None);
		})
	}
}

mod feasibility_check {
	//use super::*;

	#[test]
	fn something() {
		//ExtBuilder::verifier().execute_with(|| {
		//	assert!(true);
		//})
	}
}