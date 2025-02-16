//! Defines Tx Builder Methods.

use bitcoin::secp256k1::{self, PublicKey, Secp256k1};
use bitcoin::{Amount, Transaction};

use crate::chain::transaction::OutPoint;
use crate::sign::chan_utils::{self, TxCreationKeys};
use crate::sign::{ChannelTransactionParameters, HTLCOutputInCommitment};

pub(crate) trait TxBuilder {
	fn provide_populated_parameters(&mut self, channel_parameters: &ChannelTransactionParameters);
	fn provide_channel_parameters(&mut self, channel_parameters: &ChannelTransactionParameters);
	fn get_populated_parameters(&self) -> &ChannelTransactionParameters;
	fn provide_funding_outpoint(&mut self, outpoint: OutPoint);
	fn build_commitment_transaction(
		&self, is_holder_tx: bool, commitment_number: u64, per_commitment_point: &PublicKey,
		to_broadcaster_value_sat: Amount, to_countersignatory_value_sat: Amount,
		htlcs: Vec<&mut HTLCOutputInCommitment>, secp_ctx: &Secp256k1<secp256k1::All>,
	) -> (Transaction, Vec<HTLCOutputInCommitment>) {
		let params = if is_holder_tx {
			self.get_populated_parameters().as_holder_broadcastable()
		} else {
			self.get_populated_parameters().as_counterparty_broadcastable()
		};
		let keys = TxCreationKeys::from_channel_static_keys(
			per_commitment_point,
			params.broadcaster_pubkeys(),
			params.countersignatory_pubkeys(),
			secp_ctx,
		);
		let (obscured_commitment_transaction_number, txins) =
			chan_utils::internal_build_inputs(commitment_number, &params);
		let (txouts, sorted_htlcs) = chan_utils::internal_build_outputs(
			&keys,
			to_broadcaster_value_sat,
			to_countersignatory_value_sat,
			htlcs,
			&params,
		);

		(
			chan_utils::make_transaction(obscured_commitment_transaction_number, txins, txouts),
			sorted_htlcs,
		)
	}
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SpecTxBuilder {
	channel_parameters: Option<ChannelTransactionParameters>,
}

impl TxBuilder for SpecTxBuilder {
	fn provide_populated_parameters(&mut self, channel_parameters: &ChannelTransactionParameters) {
		assert!(
			self.channel_parameters.is_none()
				|| self.channel_parameters.as_ref().unwrap() == channel_parameters
		);
		if self.channel_parameters.is_some() {
			// The channel parameters were already set and they match, return early.
			return;
		}
		assert!(channel_parameters.is_populated(), "Channel parameters must be fully populated");
		self.channel_parameters = Some(channel_parameters.clone());
	}

	fn provide_channel_parameters(&mut self, channel_parameters: &ChannelTransactionParameters) {
		assert!(
			self.channel_parameters.is_none()
				|| self.channel_parameters.as_ref().unwrap() == channel_parameters
		);
		if self.channel_parameters.is_some() {
			// The channel parameters were already set and they match, return early.
			return;
		}
		assert!(
			channel_parameters.counterparty_parameters.is_some()
				&& channel_parameters.funding_outpoint.is_none()
		);
		self.channel_parameters = Some(channel_parameters.clone());
	}

	fn provide_funding_outpoint(&mut self, outpoint: OutPoint) {
		// all parameters should be set already, except the funding outpoint
		let params = self.channel_parameters.as_ref().unwrap();
		assert!(params.counterparty_parameters.is_some());

		assert!(params.funding_outpoint.is_none() || params.funding_outpoint == Some(outpoint));
		if params.funding_outpoint.is_some() {
			// The funding outpoint was already set and it matches, return early.
			return;
		}
		self.channel_parameters.as_mut().unwrap().funding_outpoint = Some(outpoint);
		// channel parameters should now be fully populated
		assert!(self.channel_parameters.as_ref().unwrap().is_populated());
	}

	fn get_populated_parameters(&self) -> &ChannelTransactionParameters {
		let params = self.channel_parameters.as_ref().unwrap();
		assert!(params.is_populated());
		params
	}
}
