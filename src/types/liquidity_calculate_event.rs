//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Emitted when liquidity decreased or increase.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidityCalculateEvent {
    /// The pool liquidity before decrease or increase
    pub pool_liquidity: u128,
    /// The pool price when decrease or increase in liquidity
    pub pool_sqrt_price_x64: u128,
    /// The pool tick when decrease or increase in liquidity
    pub pool_tick: i32,
    /// The amount of token_0 that was calculated for the decrease or increase in liquidity
    pub calc_amount0: u64,
    /// The amount of token_1 that was calculated for the decrease or increase in liquidity
    pub calc_amount1: u64,
    pub trade_fee_owed0: u64,
    /// The amount of token_1 fee
    pub trade_fee_owed1: u64,
    /// The amount of token_0 transfer fee without trade_fee_amount_0
    pub transfer_fee0: u64,
    /// The amount of token_1 transfer fee without trade_fee_amount_0
    pub transfer_fee1: u64,
}
