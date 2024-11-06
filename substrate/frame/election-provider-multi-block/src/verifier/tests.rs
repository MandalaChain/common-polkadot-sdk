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

use crate::{
	mock::*,
	verifier::{impls::pallet::*, *},
	Phase,
};
use frame_support::{assert_err, assert_noop, assert_ok};
use sp_npos_elections::ElectionScore;
use sp_runtime::Perbill;

#[test]
fn ensure_score_quality_works() {
	ExtBuilder::default()
		.solution_improvements_threshold(Perbill::from_percent(10))
		.build_and_execute(|| {
			assert_eq!(MinimumScore::<T>::get(), Default::default());
			assert!(<Pallet<T> as Verifier>::queued_score().is_none());

			// if minimum score is not set and there's no queued score, any score has quality.
			assert_ok!(Pallet::<T>::ensure_score_quality(ElectionScore {
				minimal_stake: 1,
				sum_stake: 1,
				sum_stake_squared: 1
			}));

			// if minimum score is set, the score being evaluated must be higher than the minimum
			// score.
			MinimumScore::<T>::set(
				ElectionScore { minimal_stake: 10, sum_stake: 20, sum_stake_squared: 300 }.into(),
			);

			// score is not higher than minimum score.
			assert_err!(
				Pallet::<T>::ensure_score_quality(ElectionScore {
					minimal_stake: 1,
					sum_stake: 1,
					sum_stake_squared: 1,
				}),
				FeasibilityError::ScoreTooLow
			);

			// if score improves the current one by the minimum solution improvement, we're gold.
			assert_ok!(Pallet::<T>::ensure_score_quality(ElectionScore {
				minimal_stake: 11,
				sum_stake: 22,
				sum_stake_squared: 300
			}));
		})
}

mod solution {
	use super::*;

	#[test]
	fn variant_flipping_works() {
		ExtBuilder::default().build_and_execute(|| {
			assert!(QueuedSolution::<T>::valid() != QueuedSolution::<T>::invalid());

			let valid_before = QueuedSolution::<T>::valid();
			let invalid_before = valid_before.other();

			let mock_score = ElectionScore { minimal_stake: 10, ..Default::default() };

			// queue solution and flip variant.
			QueuedSolution::<T>::finalize_solution(mock_score);

			// solution has been queued
			assert_eq!(QueuedSolution::<T>::queued_score().unwrap(), mock_score);
			// variant has flipped.
			assert_eq!(QueuedSolution::<T>::valid(), invalid_before);
			assert_eq!(QueuedSolution::<T>::invalid(), valid_before);
		})
	}
}

mod feasibility_check {
	use super::*;

	#[test]
	fn winner_indices_page_in_bounds() {
		ExtBuilder::default().pages(1).desired_targets(2).build_and_execute(|| {
			roll_to_phase(Phase::Signed);
			let mut solution = mine_full(1).unwrap();
			assert_eq!(crate::Snapshot::<Runtime>::targets().unwrap().len(), 8);

			// swap all votes from 3 to 4 to invalidate index 4.
			solution.solution_pages[0]
				.votes1
				.iter_mut()
				.filter(|(_, t)| *t == TargetIndex::from(3u16))
				.for_each(|(_, t)| *t += 1);

			assert_noop!(
				VerifierPallet::feasibility_check(solution.solution_pages[0].clone(), 0),
				FeasibilityError::InvalidVote,
			);
		})
	}

	#[test]
	fn targets_not_in_snapshot() {
		ExtBuilder::default().build_and_execute(|| {
			roll_to_phase(Phase::Off);

			crate::Snapshot::<Runtime>::kill();
			assert_eq!(crate::Snapshot::<Runtime>::targets(), None);

			assert_noop!(
				VerifierPallet::feasibility_check(TestNposSolution::default(), 0),
				FeasibilityError::SnapshotUnavailable,
			);
		})
	}

	#[test]
	fn voters_not_in_snapshot() {
		ExtBuilder::default().build_and_execute(|| {
			roll_to_phase(Phase::Signed);

			let _ = crate::PagedVoterSnapshot::<Runtime>::clear(u32::MAX, None);

			assert_eq!(crate::Snapshot::<Runtime>::targets().unwrap().len(), 8);
			assert_eq!(crate::Snapshot::<Runtime>::voters(0), None);

			assert_noop!(
				VerifierPallet::feasibility_check(TestNposSolution::default(), 0),
				FeasibilityError::SnapshotUnavailable,
			);
		})
	}

	#[test]
	fn desired_targets_not_in_snapshot() {
		ExtBuilder::default().no_desired_targets().build_and_execute(|| {
			roll_to_phase(Phase::Signed);

			assert_eq!(crate::Snapshot::<Runtime>::targets().unwrap().len(), 8);
			assert_ne!(crate::Snapshot::<Runtime>::voters(0).unwrap().len(), 0);
			assert_eq!(crate::Snapshot::<Runtime>::desired_targets(), None);

			assert_err!(
				VerifierPallet::feasibility_check(TestNposSolution::default(), 0),
				FeasibilityError::SnapshotUnavailable,
			);
		})
	}
}

mod sync_verifier {
	use super::*;

	mod verify_synchronous {
		use sp_runtime::traits::Bounded;

		use super::*;

		#[test]
		fn given_better_solution_stores_provided_page_as_valid_solution() {
			ExtBuilder::default().pages(1).build_and_execute(|| {
				roll_to_phase(Phase::Signed);
				let solution = mine_full(0).unwrap();

				// empty solution storage items before verification
				assert!(<VerifierPallet as Verifier>::next_missing_solution_page().is_some());
				assert!(QueuedSolutionBackings::<Runtime>::get(0).is_none());
				assert!(match QueuedSolution::<Runtime>::invalid() {
					SolutionPointer::X => QueuedSolutionX::<T>::get(0),
					SolutionPointer::Y => QueuedSolutionY::<T>::get(0),
				}
				.is_none());

				assert_ok!(<VerifierPallet as Verifier>::verify_synchronous(
					solution.solution_pages[0].clone(),
					solution.score,
					0,
				));

				// solution storage items filled after verification
				assert!(QueuedSolutionBackings::<Runtime>::get(0).is_some());
				assert_eq!(<VerifierPallet as Verifier>::next_missing_solution_page(), None);
				assert!(match QueuedSolution::<Runtime>::invalid() {
					SolutionPointer::X => QueuedSolutionX::<T>::get(0),
					SolutionPointer::Y => QueuedSolutionY::<T>::get(0),
				}
				.is_some());
			})
		}

		#[test]
		fn returns_error_if_score_quality_is_lower_than_expected() {
			ExtBuilder::default().pages(1).build_and_execute(|| {
				roll_to_phase(Phase::Signed);

				// a solution already stored
				let score =
					ElectionScore { minimal_stake: u128::max_value(), ..Default::default() };
				QueuedSolution::<T>::finalize_solution(score);

				let solution = mine_full(0).unwrap();
				assert_err!(
					<VerifierPallet as Verifier>::verify_synchronous(
						solution.solution_pages[0].clone(),
						solution.score,
						0,
					),
					FeasibilityError::ScoreTooLow
				);
			})
		}

		#[test]
		fn returns_error_if_solution_fails_feasibility_check() {
			ExtBuilder::default().build_and_execute(|| {
				roll_to_phase(Phase::Signed);

				let solution = mine_full(0).unwrap();
				let _ = crate::PagedVoterSnapshot::<Runtime>::clear(u32::MAX, None);
				assert_err!(
					<VerifierPallet as Verifier>::verify_synchronous(
						solution.solution_pages[0].clone(),
						solution.score,
						0,
					),
					FeasibilityError::SnapshotUnavailable
				);
			})
		}

		#[test]
		fn returns_error_if_computed_score_is_different_than_provided() {
			ExtBuilder::default().build_and_execute(|| {
				roll_to_phase(Phase::Signed);
				let solution = mine_full(0).unwrap();
				assert_err!(
					<VerifierPallet as Verifier>::verify_synchronous(
						solution.solution_pages[0].clone(),
						solution.score,
						0,
					),
					FeasibilityError::InvalidScore
				);
			})
		}
	}

	#[test]
	fn next_missing_solution_works() {
		ExtBuilder::default().build_and_execute(|| {
			let supports: SupportsOf<Pallet<T>> = Default::default();
			let msp = crate::Pallet::<T>::msp();
			assert!(msp == <T as crate::Config>::Pages::get() - 1 && msp == 2);

			// run to snapshot phase to reset `RemainingUnsignedPages`.
			roll_to_phase(Phase::Snapshot(crate::Pallet::<T>::lsp()));

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

mod async_verifier {
	use super::*;

	#[test]
	fn async_verifier_simple_works() {
		ExtBuilder::default().build_and_execute(|| {})
	}
}
