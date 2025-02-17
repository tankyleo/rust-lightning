//! Defines Witness Builder Methods.

use core::borrow::Borrow;

use crate::sign::tx_builder::ChannelParameters;
use crate::sign::EcdsaChannelSigner;

use bitcoin::secp256k1::{self, PublicKey, Secp256k1, SecretKey};
use bitcoin::{Transaction, Witness};

use crate::ln::chan_utils::{self, TxCreationKeys};
use crate::sign::{EcdsaSignature, HTLCOutputInCommitment};
use crate::types::payment::PaymentPreimage;

pub(crate) trait WitnessBuilder: ChannelParameters {
	fn spend_justice_revoked_output(
		&self, justice_tx: &Transaction, input: usize, amount: u64, per_commitment_key: &SecretKey,
		secp_ctx: &Secp256k1<secp256k1::All>,
	) -> Result<Witness, ()>;
	fn spend_justice_revoked_htlc(
		&self, justice_tx: &Transaction, input: usize, amount: u64, per_commitment_key: &SecretKey,
		htlc: &HTLCOutputInCommitment, secp_ctx: &Secp256k1<secp256k1::All>,
	) -> Result<Witness, ()>;
	fn spend_counterparty_htlc_output(&self, sweep_tx: &Transaction, input: usize, amount: u64, secp_ctx: &Secp256k1<secp256k1::All>, per_commitment_point: &PublicKey, htlc: &HTLCOutputInCommitment, preimage: Option<&PaymentPreimage>) -> Result<Witness, ()>;
}

impl<T> WitnessBuilder for T
where
	T: ChannelParameters + EcdsaChannelSigner,
{
	fn spend_justice_revoked_output(
		&self, justice_tx: &Transaction, input: usize, amount: u64, per_commitment_key: &SecretKey,
		secp_ctx: &Secp256k1<secp256k1::All>
	) -> Result<Witness, ()> {
		let per_commitment_point = PublicKey::from_secret_key(secp_ctx, per_commitment_key);
		let sig = self.sign_justice_revoked_output(
			justice_tx,
			input,
			amount,
			per_commitment_key,
			secp_ctx,
		)?;
		let ecdsa_sig = EcdsaSignature::sighash_all(sig);

		let params = self.get_populated_parameters().as_counterparty_broadcastable();
		let contest_delay = params.contest_delay();
		let keys = TxCreationKeys::from_channel_static_keys(
			&per_commitment_point,
			params.broadcaster_pubkeys(),
			params.countersignatory_pubkeys(),
			secp_ctx,
		);
		let witness_script = chan_utils::get_revokeable_redeemscript(
			&keys.revocation_key,
			contest_delay,
			&keys.broadcaster_delayed_payment_key,
		);

		Ok(Witness::from_slice(
			&[ecdsa_sig.serialize().as_ref(), &[1][..], witness_script.as_bytes()][..],
		))
	}

	fn spend_justice_revoked_htlc(
		&self, justice_tx: &Transaction, input: usize, amount: u64, per_commitment_key: &SecretKey,
		htlc: &HTLCOutputInCommitment, secp_ctx: &Secp256k1<secp256k1::All>,
	) -> Result<Witness, ()> {
		let per_commitment_point = PublicKey::from_secret_key(secp_ctx, per_commitment_key);
		let sig = self.sign_justice_revoked_htlc(justice_tx, input, amount, per_commitment_key, htlc, secp_ctx)?;
		let ecdsa_sig = EcdsaSignature::sighash_all(sig);

		let params = self.get_populated_parameters().as_counterparty_broadcastable();
		let keys = TxCreationKeys::from_channel_static_keys(
			&per_commitment_point,
			params.broadcaster_pubkeys(),
			params.countersignatory_pubkeys(),
			secp_ctx,
		);
		let witness_script = chan_utils::get_htlc_redeemscript(htlc, params.channel_type_features(), &keys);

		Ok(Witness::from_slice(&[ecdsa_sig.serialize().as_ref(), &keys.revocation_key.to_public_key().serialize()[..], witness_script.as_bytes(),][..]))
	}

	fn spend_counterparty_htlc_output(&self, sweep_tx: &Transaction, input: usize, amount: u64, secp_ctx: &Secp256k1<secp256k1::All>, per_commitment_point: &PublicKey, htlc: &HTLCOutputInCommitment, preimage: Option<&PaymentPreimage>) -> Result<Witness, ()> {
		let sig = self.sign_counterparty_htlc_transaction(sweep_tx, input, amount, per_commitment_point, htlc, secp_ctx)?;
		let ecdsa_sig = EcdsaSignature::sighash_all(sig);
		let witness_item = match preimage {
			Some(p) => p.borrow(),
			None => &[][..],
		};

		let params = self.get_populated_parameters().as_counterparty_broadcastable();
		let keys = TxCreationKeys::from_channel_static_keys(
			&per_commitment_point,
			params.broadcaster_pubkeys(),
			params.countersignatory_pubkeys(),
			secp_ctx,
		);
		let witness_script = chan_utils::get_htlc_redeemscript(htlc, params.channel_type_features(), &keys);

		Ok(Witness::from_slice(&[ecdsa_sig.serialize().as_ref(), witness_item, witness_script.as_bytes()][..]))
	}
}
