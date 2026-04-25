// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

macro_rules! hash_to_message {
	($slice: expr) => {{
		#[cfg(not(fuzzing))]
		{
			let digest = <[u8; 32] as ::core::convert::TryFrom<&[u8]>>::try_from($slice).unwrap();
			::bitcoin::secp256k1::Message::from_digest(digest)
		}
		#[cfg(fuzzing)]
		{
			match <[u8; 32] as ::core::convert::TryFrom<&[u8]>>::try_from($slice) {
				Ok(digest) => ::bitcoin::secp256k1::Message::from_digest(digest),
				Err(_) => ::bitcoin::secp256k1::Message::from_digest([1; 32]),
			}
		}
	}};
}
