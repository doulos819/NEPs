mod burning_impl;

pub use burning_impl::*;

use crate::non_fungible_token::token::{Token, TokenId};
use near_sdk::AccountId;

pub trait NonFungibleTokenBurning {
    pub fn burn(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
    ) -> Token;

    pub fn utility_burn(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
    ) -> Token;
}