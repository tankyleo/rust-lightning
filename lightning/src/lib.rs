// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

#![crate_name = "lightning"]

//! Rust-Lightning, not Rusty's Lightning!
//!
//! A full-featured but also flexible lightning implementation, in library form. This allows the
//! user (you) to decide how they wish to use it instead of being a fully self-contained daemon.
//! This means there is no built-in threading/execution environment and it's up to the user to
//! figure out how best to make networking happen/timers fire/things get written to disk/keys get
//! generated/etc. This makes it a good candidate for tight integration into an existing wallet
//! instead of having a rather-separate lightning appendage to a wallet.
//!
//! `default` features are:
//!
//! * `std` - enables functionalities which require `std`, including `std::io` trait implementations and things which utilize time
//! * `grind_signatures` - enables generation of [low-r bitcoin signatures](https://bitcoin.stackexchange.com/questions/111660/what-is-signature-grinding),
//! which saves 1 byte per signature in 50% of the cases (see [bitcoin PR #13666](https://github.com/bitcoin/bitcoin/pull/13666))
//!
//! Available features are:
//!
//! * `std`
//! * `grind_signatures`

#![cfg_attr(not(any(test, fuzzing, feature = "_test_utils")), deny(missing_docs))]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
// In general, rust is absolutely horrid at supporting users doing things like,
// for example, compiling Rust code for real environments. Disable useless lints
// that don't do anything but annoy us and cant actually ever be resolved.
#![allow(bare_trait_objects)]
#![allow(ellipsis_inclusive_range_patterns)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#[cfg(all(fuzzing, test))]
compile_error!("Tests will always fail with cfg=fuzzing");

#[macro_use]
extern crate alloc;

pub extern crate lightning_types as types;

pub extern crate bitcoin;

pub extern crate lightning_invoice as bolt11_invoice;

#[cfg(any(test, feature = "std"))]
extern crate core;

#[cfg(any(test, feature = "_test_utils"))]
extern crate regex;

#[cfg(not(feature = "std"))]
extern crate libm;

#[cfg(ldk_bench)]
extern crate criterion;

#[cfg(all(feature = "std", test))]
extern crate parking_lot;

#[macro_use]
pub mod util;

pub mod blinded_path;
pub mod chain;
pub mod events;
pub mod ln;
pub mod offers;
pub mod onion_message;
pub mod routing;
pub mod sign;

pub(crate) mod crypto;

/// Extension of the bitcoin::io module
pub mod io;

#[doc(hidden)]
/// IO utilities public only for use by in-crate macros. These should not be used externally
///
/// This is not exported to bindings users as it is not intended for public consumption.
pub mod io_extras {
	use bitcoin::io::{self, Read, Write};

	/// Creates an instance of a writer which will successfully consume all data.
	pub use bitcoin::io::sink;

	pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> Result<u64, io::Error>
	where
		R: Read,
		W: Write,
	{
		let mut count = 0;
		let mut buf = [0u8; 64];

		loop {
			match reader.read(&mut buf) {
				Ok(0) => break,
				Ok(n) => {
					writer.write_all(&buf[0..n])?;
					count += n as u64;
				},
				Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
				Err(e) => return Err(e.into()),
			};
		}
		Ok(count)
	}

	pub fn read_to_end<D: Read>(d: &mut D) -> Result<alloc::vec::Vec<u8>, io::Error> {
		let mut result = vec![];
		let mut buf = [0u8; 64];
		loop {
			match d.read(&mut buf) {
				Ok(0) => break,
				Ok(n) => result.extend_from_slice(&buf[0..n]),
				Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
				Err(e) => return Err(e.into()),
			};
		}
		Ok(result)
	}
}

mod prelude {
	#![allow(unused_imports)]
	#![allow(dead_code)]

	pub use alloc::{boxed::Box, collections::VecDeque, string::String, vec, vec::Vec};

	pub use alloc::borrow::ToOwned;
	pub use alloc::string::ToString;

	pub use core::convert::{AsMut, AsRef, TryFrom, TryInto};
	pub use core::default::Default;
	pub use core::marker::Sized;

	pub use bitcoin::ext::*;

	pub(crate) trait LegacyAllZeros {
		fn all_zeros() -> Self;
	}

	impl LegacyAllZeros for bitcoin::BlockHash {
		fn all_zeros() -> Self {
			Self::from_byte_array([0; 32])
		}
	}

	impl LegacyAllZeros for bitcoin::TxMerkleNode {
		fn all_zeros() -> Self {
			Self::from_byte_array([0; 32])
		}
	}

	impl LegacyAllZeros for bitcoin::script::WScriptHash {
		fn all_zeros() -> Self {
			Self::from_byte_array([0; 32])
		}
	}

	impl LegacyAllZeros for bitcoin::key::PubkeyHash {
		fn all_zeros() -> Self {
			Self::from_byte_array([0; 20])
		}
	}

	impl LegacyAllZeros for bitcoin::key::WPubkeyHash {
		fn all_zeros() -> Self {
			Self::from_byte_array([0; 20])
		}
	}

	pub(crate) trait LegacySecretKeyExt {
		fn from_slice(data: &[u8]) -> Result<Self, bitcoin::secp256k1::Error>
		where
			Self: Sized;
	}

	impl LegacySecretKeyExt for bitcoin::secp256k1::SecretKey {
		fn from_slice(data: &[u8]) -> Result<Self, bitcoin::secp256k1::Error> {
			let data = <[u8; 32]>::try_from(data)
				.map_err(|_| bitcoin::secp256k1::Error::InvalidSecretKey)?;
			Self::from_secret_bytes(data)
		}
	}

	pub(crate) fn secret_key_from_slice(
		data: &[u8],
	) -> Result<bitcoin::secp256k1::SecretKey, bitcoin::secp256k1::Error> {
		let data =
			<[u8; 32]>::try_from(data).map_err(|_| bitcoin::secp256k1::Error::InvalidSecretKey)?;
		bitcoin::secp256k1::SecretKey::from_secret_bytes(data)
	}

	pub(crate) trait LegacyFromSlice {
		fn from_slice(data: &[u8]) -> Result<Self, ()>
		where
			Self: Sized;
	}

	impl LegacyFromSlice for bitcoin::Txid {
		fn from_slice(data: &[u8]) -> Result<Self, ()> {
			let data = <[u8; 32]>::try_from(data).map_err(|_| ())?;
			Ok(Self::from_byte_array(data))
		}
	}

	impl LegacyFromSlice for bitcoin::BlockHash {
		fn from_slice(data: &[u8]) -> Result<Self, ()> {
			let data = <[u8; 32]>::try_from(data).map_err(|_| ())?;
			Ok(Self::from_byte_array(data))
		}
	}

	impl LegacyFromSlice for bitcoin::hashes::sha256::Hash {
		fn from_slice(data: &[u8]) -> Result<Self, ()> {
			let data = <[u8; 32]>::try_from(data).map_err(|_| ())?;
			Ok(Self::from_byte_array(data))
		}
	}

	impl LegacyFromSlice for bitcoin::XOnlyPublicKey {
		fn from_slice(data: &[u8]) -> Result<Self, ()> {
			let data = <[u8; 32]>::try_from(data).map_err(|_| ())?;
			Self::from_byte_array(&data).map_err(|_| ())
		}
	}

	pub(crate) trait LegacyTxidExt {
		fn from_raw_hash(hash: bitcoin::hashes::sha256d::Hash) -> Self;
	}

	impl LegacyTxidExt for bitcoin::Txid {
		fn from_raw_hash(hash: bitcoin::hashes::sha256d::Hash) -> Self {
			Self::from_byte_array(hash.to_byte_array())
		}
	}

	pub(crate) trait LegacyBlockHashExt {
		fn hash(data: &[u8]) -> Self;
	}

	impl LegacyBlockHashExt for bitcoin::BlockHash {
		fn hash(data: &[u8]) -> Self {
			let hash = bitcoin::hashes::sha256d::Hash::hash(data);
			Self::from_byte_array(hash.to_byte_array())
		}
	}

	pub(crate) trait LegacyOutPointExt {
		fn null() -> Self;
	}

	impl LegacyOutPointExt for bitcoin::OutPoint {
		fn null() -> Self {
			Self::COINBASE_PREVOUT
		}
	}

	pub(crate) trait LegacyScriptPubKeyBufDustExt {
		fn minimal_non_dust(&self) -> bitcoin::Amount;
	}

	impl LegacyScriptPubKeyBufDustExt for bitcoin::ScriptPubKeyBuf {
		fn minimal_non_dust(&self) -> bitcoin::Amount {
			bitcoin::script::ScriptPubKeyExt::minimal_non_dust(self.as_script())
		}
	}

	pub(crate) trait LegacyScriptBufExt {
		fn to_p2sh(&self) -> bitcoin::ScriptPubKeyBuf;
		fn to_p2wsh(&self) -> bitcoin::ScriptPubKeyBuf;
		fn wscript_hash(&self) -> bitcoin::script::WScriptHash;
	}

	impl LegacyScriptBufExt for bitcoin::ScriptPubKeyBuf {
		fn to_p2sh(&self) -> bitcoin::ScriptPubKeyBuf {
			let redeem_script = bitcoin::RedeemScriptBuf::from_bytes(self.as_bytes().to_vec());
			bitcoin::script::ScriptExt::to_p2sh(redeem_script.as_script())
				.expect("LDK redeem scripts fit within the standard redeem script size")
		}

		fn to_p2wsh(&self) -> bitcoin::ScriptPubKeyBuf {
			let witness_script = bitcoin::WitnessScriptBuf::from_bytes(self.as_bytes().to_vec());
			bitcoin::script::WitnessScriptExt::to_p2wsh(witness_script.as_script())
				.expect("LDK witness scripts fit within the standard witness script size")
		}

		fn wscript_hash(&self) -> bitcoin::script::WScriptHash {
			let witness_script = bitcoin::WitnessScriptBuf::from_bytes(self.as_bytes().to_vec());
			bitcoin::script::WitnessScriptExt::wscript_hash(witness_script.as_script())
				.expect("LDK witness scripts fit within the standard witness script size")
		}
	}

	impl LegacyScriptBufExt for bitcoin::ScriptPubKey {
		fn to_p2sh(&self) -> bitcoin::ScriptPubKeyBuf {
			let redeem_script = bitcoin::RedeemScriptBuf::from_bytes(self.as_bytes().to_vec());
			bitcoin::script::ScriptExt::to_p2sh(redeem_script.as_script())
				.expect("LDK redeem scripts fit within the standard redeem script size")
		}

		fn to_p2wsh(&self) -> bitcoin::ScriptPubKeyBuf {
			let witness_script = bitcoin::WitnessScriptBuf::from_bytes(self.as_bytes().to_vec());
			bitcoin::script::WitnessScriptExt::to_p2wsh(witness_script.as_script())
				.expect("LDK witness scripts fit within the standard witness script size")
		}

		fn wscript_hash(&self) -> bitcoin::script::WScriptHash {
			let witness_script = bitcoin::WitnessScriptBuf::from_bytes(self.as_bytes().to_vec());
			bitcoin::script::WitnessScriptExt::wscript_hash(witness_script.as_script())
				.expect("LDK witness scripts fit within the standard witness script size")
		}
	}

	pub(crate) use crate::util::hash_tables::*;
}

#[cfg(all(not(ldk_bench), feature = "backtrace", feature = "std", test))]
extern crate backtrace;

mod sync;

#[cfg(feature = "_externalize_tests")]
lightning_macros::xtest_inventory!();
