#include <stdint.h>
void bech32_parse_run(const unsigned char* data, size_t data_len);
void chanmon_deser_run(const unsigned char* data, size_t data_len);
void chanmon_consistency_run(const unsigned char* data, size_t data_len);
void full_stack_run(const unsigned char* data, size_t data_len);
void invoice_deser_run(const unsigned char* data, size_t data_len);
void invoice_request_deser_run(const unsigned char* data, size_t data_len);
void offer_deser_run(const unsigned char* data, size_t data_len);
void bolt11_deser_run(const unsigned char* data, size_t data_len);
void onion_message_run(const unsigned char* data, size_t data_len);
void peer_crypt_run(const unsigned char* data, size_t data_len);
void process_network_graph_run(const unsigned char* data, size_t data_len);
void refund_deser_run(const unsigned char* data, size_t data_len);
void router_run(const unsigned char* data, size_t data_len);
void zbase32_run(const unsigned char* data, size_t data_len);
void indexedmap_run(const unsigned char* data, size_t data_len);
void onion_hop_data_run(const unsigned char* data, size_t data_len);
void base32_run(const unsigned char* data, size_t data_len);
void fromstr_to_netaddress_run(const unsigned char* data, size_t data_len);
void msg_accept_channel_run(const unsigned char* data, size_t data_len);
void msg_announcement_signatures_run(const unsigned char* data, size_t data_len);
void msg_channel_reestablish_run(const unsigned char* data, size_t data_len);
void msg_closing_signed_run(const unsigned char* data, size_t data_len);
void msg_commitment_signed_run(const unsigned char* data, size_t data_len);
void msg_decoded_onion_error_packet_run(const unsigned char* data, size_t data_len);
void msg_funding_created_run(const unsigned char* data, size_t data_len);
void msg_channel_ready_run(const unsigned char* data, size_t data_len);
void msg_funding_signed_run(const unsigned char* data, size_t data_len);
void msg_init_run(const unsigned char* data, size_t data_len);
void msg_open_channel_run(const unsigned char* data, size_t data_len);
void msg_revoke_and_ack_run(const unsigned char* data, size_t data_len);
void msg_shutdown_run(const unsigned char* data, size_t data_len);
void msg_update_fail_htlc_run(const unsigned char* data, size_t data_len);
void msg_update_fail_malformed_htlc_run(const unsigned char* data, size_t data_len);
void msg_update_fee_run(const unsigned char* data, size_t data_len);
void msg_update_fulfill_htlc_run(const unsigned char* data, size_t data_len);
void msg_channel_announcement_run(const unsigned char* data, size_t data_len);
void msg_node_announcement_run(const unsigned char* data, size_t data_len);
void msg_query_short_channel_ids_run(const unsigned char* data, size_t data_len);
void msg_reply_short_channel_ids_end_run(const unsigned char* data, size_t data_len);
void msg_query_channel_range_run(const unsigned char* data, size_t data_len);
void msg_reply_channel_range_run(const unsigned char* data, size_t data_len);
void msg_gossip_timestamp_filter_run(const unsigned char* data, size_t data_len);
void msg_update_add_htlc_run(const unsigned char* data, size_t data_len);
void msg_error_message_run(const unsigned char* data, size_t data_len);
void msg_channel_update_run(const unsigned char* data, size_t data_len);
void msg_ping_run(const unsigned char* data, size_t data_len);
void msg_pong_run(const unsigned char* data, size_t data_len);
void msg_channel_details_run(const unsigned char* data, size_t data_len);
void msg_open_channel_v2_run(const unsigned char* data, size_t data_len);
void msg_accept_channel_v2_run(const unsigned char* data, size_t data_len);
void msg_tx_add_input_run(const unsigned char* data, size_t data_len);
void msg_tx_add_output_run(const unsigned char* data, size_t data_len);
void msg_tx_remove_input_run(const unsigned char* data, size_t data_len);
void msg_tx_remove_output_run(const unsigned char* data, size_t data_len);
void msg_tx_complete_run(const unsigned char* data, size_t data_len);
void msg_tx_signatures_run(const unsigned char* data, size_t data_len);
void msg_tx_init_rbf_run(const unsigned char* data, size_t data_len);
void msg_tx_ack_rbf_run(const unsigned char* data, size_t data_len);
void msg_tx_abort_run(const unsigned char* data, size_t data_len);
void msg_stfu_run(const unsigned char* data, size_t data_len);
void msg_splice_run(const unsigned char* data, size_t data_len);
void msg_splice_ack_run(const unsigned char* data, size_t data_len);
void msg_splice_locked_run(const unsigned char* data, size_t data_len);
