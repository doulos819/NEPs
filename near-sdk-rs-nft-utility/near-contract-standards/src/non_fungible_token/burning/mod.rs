mod burning_impl;

pub use burning_impl::*;

use crate::non_fungible_token::token::{Token, TokenId};
use near_sdk::AccountId;

/// Trait used when it's desired to have a non-fungible token that has a
/// a function for burning and utility burning. Burning a token will remove 
/// remove the existence of the token without any receipt. Utility burning 
/// will burn the token and mint a receipt NFT out of the burned NFT's metadata.
pub trait NonFungibleTokenBurning {
    /// Burn a NFT.
    ///
    /// Requirements
    /// * Token must exists.
    /// * Token owner should be the caller for burning the token. 
    ///
    /// Arguments:
    /// * `token_id`: the token for which needs to be burned
    /// * `token_owner_id`: the account owner id for burning the token
    pub fn burn(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
    ) -> Token;

    /// Utility Burn a NFT.
    ///
    /// Requirements
    /// * Token must exists.
    /// * Token owner should be the caller for burning the token. 
    ///
    /// Arguments:
    /// * `token_id`: the token for which needs to be burned
    /// * `token_owner_id`: the account owner id for burning the token
    pub fn utility_burn(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
    ) -> Token;
}