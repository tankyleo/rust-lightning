//! Defines Claim Sweeper Methods.

use crate::sign::tx_builder::ChannelParameters;
use crate::chain::package::PackageSolvingData;
use crate::sign::witness_builder::WitnessBuilder;

use bitcoin::Transaction;
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;

trait ClaimsSweeper: ChannelParameters {
	fn finalize_input(&self, claim: &PackageSolvingData, bumped_tx: &mut Transaction, i: usize, secp_ctx: &Secp256k1<secp256k1::All>) -> bool;
}

impl<T> ClaimsSweeper for T where T: WitnessBuilder {
	fn finalize_input(&self, claim: &PackageSolvingData, bumped_tx: &mut Transaction, i: usize, secp_ctx: &Secp256k1<secp256k1::All>) -> bool {
		match claim {
			PackageSolvingData::RevokedOutput(ref outp) => {
				//TODO: should we panic on signer failure ?
				if let Ok(witness) = self.spend_justice_revoked_output(&bumped_tx, i, claim.amount(), &outp.per_commitment_key, secp_ctx) {
					bumped_tx.input[i].witness = witness;
				} else { return false; }
			},
			PackageSolvingData::RevokedHTLCOutput(ref outp) => {
				//TODO: should we panic on signer failure ?
				if let Ok(witness) = self.spend_justice_revoked_htlc(&bumped_tx, i, claim.amount(), &outp.per_commitment_key, &outp.htlc, secp_ctx) {
					bumped_tx.input[i].witness = witness;
				} else { return false; }
			},
			PackageSolvingData::CounterpartyOfferedHTLCOutput(ref outp) => {
				if let Ok(witness) = self.spend_counterparty_htlc_output(&bumped_tx, i, claim.amount(), secp_ctx, &outp.per_commitment_point, &outp.htlc, Some(&outp.preimage)) {
					bumped_tx.input[i].witness = witness;
				}
			},
			PackageSolvingData::CounterpartyReceivedHTLCOutput(ref outp) => {
				if let Ok(witness) = self.spend_counterparty_htlc_output(&bumped_tx, i, claim.amount(), secp_ctx, &outp.per_commitment_point, &outp.htlc, None) {
					bumped_tx.input[i].witness = witness;
				}
			},
			_ => { panic!("API Error!"); }
		}
		true
	}
}
