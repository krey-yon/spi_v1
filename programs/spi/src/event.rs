use anchor_lang::prelude::*;

#[event]
pub struct SpiTransferEvent {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub fee_collector: Pubkey,
    pub total_amount: u64,
    pub fee_amount: u64,
    pub recipient_amount: u64,
    pub timestamp: i64,
}
