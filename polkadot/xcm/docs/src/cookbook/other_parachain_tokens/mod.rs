// Copyright Parity Technologies (UK) Ltd.
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

//! # Supporting other parachain tokens
//!
//! This example shows how to configure a parachain to be able to handle other parachain tokens.
//!
//! The most important configuration is the `AssetTransactor`.
//! We need to reference other parachain tokens coming in XCMs.
//! The assets coming in have an [`xcm::latest::asset::AssetId`] which is just a wrapper around a
//! [`xcm::latest::location::Location`].
//! We could map these locations to integers and reference the assets this way internally.
//! However, a simpler way is just using the locations themselves as ids.
//!
//! 

/// The parachain runtime for this example.
pub mod parachain;

/// The relay chain runtime for this example.
pub mod relay_chain;

/// The network for this example.
pub mod network;

/// Tests for this example
#[cfg(test)]
pub mod tests;
