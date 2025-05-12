//! Defines the `TxBuilder` trait, and the `SpecTxBuilder` type

use crate::ln::chan_utils::{self, htlc_success_tx_weight, htlc_timeout_tx_weight};
use crate::ln::channel::{self, CommitmentStats};
use crate::prelude::*;
use crate::types::features::ChannelTypeFeatures;

pub(crate) struct HTLCDirectionAmount(pub(crate) bool, pub(crate) u64);

impl HTLCDirectionAmount {
	/// Checks if this HTLC is dust
	fn is_dust(
		&self, feerate_per_kw: u32, broadcaster_dust_limit_sat: u64, features: &ChannelTypeFeatures,
	) -> bool {
		let htlc_tx_fee_sat = if features.supports_anchors_zero_fee_htlc_tx() {
			0
		} else {
			let htlc_tx_weight = if self.0 {
				htlc_timeout_tx_weight(features)
			} else {
				htlc_success_tx_weight(features)
			};
			// As required by the spec, round down
			feerate_per_kw as u64 * htlc_tx_weight / 1000
		};
		self.1 / 1000 < broadcaster_dust_limit_sat + htlc_tx_fee_sat
	}
}

pub(crate) trait TxBuilder {
	fn commit_tx_fee_sat(&self, feerate_per_kw: u32, num_nondust_htlcs: usize, channel_type_features: &ChannelTypeFeatures) -> u64;
	fn build_commitment_stats(
		&self, local: bool, is_outbound_from_holder: bool, channel_features: &ChannelTypeFeatures,
		channel_value_msat: u64, value_to_self_msat: u64, htlcs_in_tx: Vec<HTLCDirectionAmount>,
		feerate_per_kw: u32, broadcaster_dust_limit_sat: u64, fee_buffer_nondust_htlcs: usize,
	) -> CommitmentStats;
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SpecTxBuilder {}

impl TxBuilder for SpecTxBuilder {
	fn commit_tx_fee_sat(&self, feerate_per_kw: u32, num_nondust_htlcs: usize, channel_type_features: &ChannelTypeFeatures) -> u64 {
		chan_utils::commit_tx_fee_sat(feerate_per_kw, num_nondust_htlcs, channel_type_features)
	}
	fn build_commitment_stats(
		&self, local: bool, is_outbound_from_holder: bool, channel_type: &ChannelTypeFeatures,
		channel_value_msat: u64, value_to_self_msat: u64, htlcs_in_tx: Vec<HTLCDirectionAmount>,
		feerate_per_kw: u32, broadcaster_dust_limit_sat: u64, fee_buffer_nondust_htlcs: usize,
	) -> CommitmentStats {
		let mut local_htlc_total_msat = 0;
		let mut remote_htlc_total_msat = 0;
		let mut nondust_htlc_count = fee_buffer_nondust_htlcs;

		for htlc in htlcs_in_tx {
			if htlc.0 == local {
				local_htlc_total_msat += htlc.1;
			} else {
				remote_htlc_total_msat += htlc.1;
			}
			if !htlc.is_dust(feerate_per_kw, broadcaster_dust_limit_sat, channel_type) {
				nondust_htlc_count += 1;
			}
		}

		// # Panics
		//
		// The value going to each party MUST be 0 or positive, even if all HTLCs pending in the
		// commitment clear by failure.

		let mut value_to_remote_msat = channel_value_msat - value_to_self_msat;
		let mut value_to_self_msat = value_to_self_msat.checked_sub(local_htlc_total_msat).unwrap();
		value_to_remote_msat = value_to_remote_msat.checked_sub(remote_htlc_total_msat).unwrap();

		let total_fee_sat =
			chan_utils::commit_tx_fee_sat(feerate_per_kw, nondust_htlc_count, channel_type);
		let total_anchors_sat = if channel_type.supports_anchors_zero_fee_htlc_tx() {
			channel::ANCHOR_OUTPUT_VALUE_SATOSHI * 2
		} else {
			0
		};

		if is_outbound_from_holder {
			value_to_self_msat = value_to_self_msat.saturating_sub(total_anchors_sat * 1000);
		} else {
			value_to_remote_msat = value_to_remote_msat.saturating_sub(total_anchors_sat * 1000);
		}

		CommitmentStats {
			total_fee_sat,
			local_balance_before_fee_msat: value_to_self_msat,
			remote_balance_before_fee_msat: value_to_remote_msat,
		}
	}
}
