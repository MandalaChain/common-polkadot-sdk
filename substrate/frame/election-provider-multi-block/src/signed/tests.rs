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

use super::*;
use crate::{mock::*, verifier::SolutionDataProvider, Phase, Verifier};
use frame_support::{assert_noop, assert_ok, testing_prelude::*};
use sp_npos_elections::ElectionScore;
use sp_runtime::traits::Convert;

#[test]
fn clear_submission_of_works() {
	ExtBuilder::default().build_and_execute(|| {});
}

mod calls {
	use super::*;
	use sp_core::bounded_vec;

	#[test]
	fn register_works() {
		ExtBuilder::default().build_and_execute(|| {
			roll_to_phase(Phase::Signed);
			assert_ok!(assert_snapshots());

			assert_eq!(balances(99), (100, 0));
			let score = ElectionScore { minimal_stake: 100, ..Default::default() };

			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(99), score));
			assert_eq!(balances(99), (90, 10));

			assert_eq!(
				Submissions::<T>::metadata_for(current_round(), &99).unwrap(),
				SubmissionMetadata {
					claimed_score: score,
					deposit: 10,
					pages: bounded_vec![false, false, false],
					release_strategy: Default::default(),
				}
			);

			assert_eq!(
				signed_events(),
				vec![Event::Registered { round: 0, who: 99, claimed_score: score }],
			);

			// duplicate submission for the same round fails.
			assert_noop!(
				SignedPallet::register(RuntimeOrigin::signed(99), score),
				Error::<T>::DuplicateRegister,
			);

			// if claimed score if below the minimum score, submission will fail.
			<VerifierPallet as Verifier>::set_minimum_score(ElectionScore {
				minimal_stake: 20,
				..Default::default()
			});

			let low_score = ElectionScore { minimal_stake: 10, ..Default::default() };
			assert_noop!(
				SignedPallet::register(RuntimeOrigin::signed(97), low_score),
				Error::<T>::SubmissionScoreTooLow,
			);
		})
	}

	#[test]
	fn register_sorted_works() {
		ExtBuilder::default().signed_max_submissions(3).build_and_execute(|| {
			// try register 5 submissions:
			// - 3 are stored.
			// - one submission is registered after queue is full while the score improves current
			// submission in the queue; other submission is discarded.
			// - one submission is registered after queue is full while the score does not improve
			// the current submission in the queue; submission is discarded.

			roll_to_phase(Phase::Signed);

			let score = ElectionScore { minimal_stake: 40, ..Default::default() };
			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(40), score));

			let score = ElectionScore { minimal_stake: 30, ..Default::default() };
			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(30), score));

			let score = ElectionScore { minimal_stake: 20, ..Default::default() };
			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(20), score));

			// submission queue is full, next submissions will only be accepted if the submitted
			// score improves the current lower score.

			// registration discarded.
			let score = ElectionScore { minimal_stake: 10, ..Default::default() };
			assert_noop!(
				SignedPallet::register(RuntimeOrigin::signed(10), score),
				Error::<T>::SubmissionsQueueFull
			);

			// higher score is successfully registered.
			let higher_score = ElectionScore { minimal_stake: 50, ..Default::default() };
			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(50), higher_score));

			assert_eq!(Submissions::<T>::leader(current_round()).unwrap(), (50, higher_score),);

			assert_eq!(
				signed_events(),
				vec![
					Event::Registered {
						round: 0,
						who: 40,
						claimed_score: ElectionScore {
							minimal_stake: 40,
							sum_stake: 0,
							sum_stake_squared: 0
						}
					},
					Event::Registered {
						round: 0,
						who: 30,
						claimed_score: ElectionScore {
							minimal_stake: 30,
							sum_stake: 0,
							sum_stake_squared: 0
						}
					},
					Event::Registered {
						round: 0,
						who: 20,
						claimed_score: ElectionScore {
							minimal_stake: 20,
							sum_stake: 0,
							sum_stake_squared: 0
						}
					},
					Event::Registered {
						round: 0,
						who: 50,
						claimed_score: ElectionScore {
							minimal_stake: 50,
							sum_stake: 0,
							sum_stake_squared: 0
						}
					},
				],
			);
		})
	}

	#[test]
	fn submit_page_works() {
		ExtBuilder::default().build_and_execute(|| {
			// bad timing.
			assert_noop!(
				SignedPallet::submit_page(RuntimeOrigin::signed(40), 0, None),
				Error::<T>::NotAcceptingSubmissions
			);

			roll_to_phase(Phase::Signed);

			// submission not registered before.
			assert_noop!(
				SignedPallet::submit_page(RuntimeOrigin::signed(10), 0, None),
				Error::<T>::SubmissionNotRegistered
			);

			let score = ElectionScore { minimal_stake: 10, ..Default::default() };
			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(10), score));

			// 0 pages submitted so far.
			assert_eq!(Submissions::<T>::page_count_submission_for(current_round(), &10), 0);

			// now submission works since there is a registered commitment.
			assert_ok!(SignedPallet::submit_page(
				RuntimeOrigin::signed(10),
				0,
				Some(Default::default())
			));

			assert_eq!(
				Submissions::<T>::page_submission_for(current_round(), 10, 0),
				Some(Default::default()),
			);

			// 1 page submitted so far.
			assert_eq!(Submissions::<T>::page_count_submission_for(current_round(), &10), 1);

			// tries to submit a page out of bounds.
			assert_noop!(
				SignedPallet::submit_page(RuntimeOrigin::signed(10), 10, Some(Default::default())),
				Error::<T>::BadPageIndex,
			);

			// 1 successful page submitted so far.
			assert_eq!(Submissions::<T>::page_count_submission_for(current_round(), &10), 1);

			assert_eq!(
				signed_events(),
				vec![
					Event::Registered {
						round: 0,
						who: 10,
						claimed_score: ElectionScore {
							minimal_stake: 10,
							sum_stake: 0,
							sum_stake_squared: 0
						}
					},
					Event::PageStored { round: 0, who: 10, page: 0 }
				],
			);
		})
	}

	#[test]
	fn bail_works() {
		ExtBuilder::default().build_and_execute(|| {
			// TODO
		})
	}

	#[test]
	fn force_clear_submission_works() {
		ExtBuilder::default().build_and_execute(|| {
			// TODO
		})
	}
}

mod deposit {
	use super::*;

	#[test]
	fn register_submit_bail_deposit_works() {
		ExtBuilder::default().build_and_execute(|| {
			assert_eq!(<Runtime as crate::Config>::Pages::get(), 3);

			roll_to_phase(Phase::Signed);
			assert_ok!(assert_snapshots());

			// expected base deposit with 0 submissions in the queue.
			let base_deposit = <Runtime as Config>::DepositBase::convert(0);
			let page_deposit = <Runtime as Config>::DepositPerPage::get();
			assert!(base_deposit != 0 && page_deposit != 0 && base_deposit != page_deposit);

			// 99 has 100 free balance and 0 held balance for elections.
			assert_eq!(balances(99), (100, 0));

			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(99), Default::default()));

			// free balance and held deposit updated as expected.
			assert_eq!(balances(99), (100 - base_deposit, base_deposit));

			// submit page 2.
			assert_ok!(SignedPallet::submit_page(
				RuntimeOrigin::signed(99),
				2,
				Some(Default::default())
			));

			// free balance and held deposit updated as expected.
			assert_eq!(
				balances(99),
				(100 - base_deposit - page_deposit, base_deposit + page_deposit)
			);

			// submit remaining pages.
			assert_ok!(SignedPallet::submit_page(
				RuntimeOrigin::signed(99),
				1,
				Some(Default::default())
			));
			assert_ok!(SignedPallet::submit_page(
				RuntimeOrigin::signed(99),
				0,
				Some(Default::default())
			));

			// free balance and held deposit updated as expected (ie. base_deposit + Pages *
			// page_deposit)
			assert_eq!(
				balances(99),
				(100 - base_deposit - (3 * page_deposit), base_deposit + (3 * page_deposit))
			);

			// now if 99 bails, all the deposits are released.
			assert_ok!(SignedPallet::bail(RuntimeOrigin::signed(99)));

			// the base deposit was burned after bail and all the pages deposit were released.
			assert_eq!(balances(99), (100 - base_deposit, 0));
		})
	}
}

mod solution_data_provider {
	use super::*;

	#[test]
	fn higher_score_works() {
		ExtBuilder::default().build_and_execute(|| {
			roll_to_phase(Phase::Signed);

			assert_eq!(<SignedPallet as SolutionDataProvider>::get_score(), None);

			let higher_score = ElectionScore { minimal_stake: 40, ..Default::default() };
			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(40), higher_score));

			let score = ElectionScore { minimal_stake: 30, ..Default::default() };
			assert_ok!(SignedPallet::register(RuntimeOrigin::signed(30), score));

			assert_eq!(<SignedPallet as SolutionDataProvider>::get_score(), Some(higher_score));
		})
	}

	#[test]
	fn get_page_works() {
		ExtBuilder::default().build_and_execute(|| {
			roll_to_phase(Phase::Signed);
			assert_eq!(<SignedPallet as SolutionDataProvider>::get_score(), None);
		})
	}
}

mod e2e {
	use super::*;

	type MaxSubmissions = <Runtime as Config>::MaxSubmissions;

	mod simple_e2e_works {
		use super::*;

		#[test]
		fn submit_solution_happy_path_works() {
			ExtBuilder::default().build_and_execute(|| {
				roll_to_phase(Phase::Signed);

				let current_round = MultiPhase::current_round();
				assert!(Submissions::<Runtime>::metadata_for(current_round, &10).is_none());

				let claimed_score = ElectionScore { minimal_stake: 100, ..Default::default() };

				// register submission
				assert_ok!(SignedPallet::register(RuntimeOrigin::signed(10), claimed_score,));

				// metadata and claimed scores have been stored as expected.
				assert_eq!(
					Submissions::<Runtime>::metadata_for(current_round, &10),
					Some(SubmissionMetadata {
						claimed_score,
						deposit: 10,
						pages: bounded_vec![false, false, false],
						release_strategy: Default::default(),
					})
				);
				let expected_scores: BoundedVec<(AccountId, ElectionScore), MaxSubmissions> =
					bounded_vec![(10, claimed_score)];
				assert_eq!(Submissions::<Runtime>::scores_for(current_round), expected_scores);

				// submit all pages of a noop solution;
				let solution = TestNposSolution::default();
				for page in (0..=MultiPhase::msp()).into_iter().rev() {
					assert_ok!(SignedPallet::submit_page(
						RuntimeOrigin::signed(10),
						page,
						Some(solution.clone())
					));

					assert_eq!(
						Submissions::<Runtime>::page_submission_for(current_round, 10, page),
						Some(solution.clone())
					);
				}

				assert_eq!(
					signed_events(),
					vec![
						Event::Registered {
							round: 0,
							who: 10,
							claimed_score: ElectionScore {
								minimal_stake: 100,
								sum_stake: 0,
								sum_stake_squared: 0
							}
						},
						Event::PageStored { round: 0, who: 10, page: 2 },
						Event::PageStored { round: 0, who: 10, page: 1 },
						Event::PageStored { round: 0, who: 10, page: 0 },
					]
				);
			})
		}
	}
}