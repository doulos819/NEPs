
use crate::non_fungible_token::burning::NonFungibleTokenBurning;
use crate::non_fungible_token::token::{Token, TokenId};
use crate::non_fungible_token::NonFungibleToken;
use near_sdk::{env, AccountId};

impl NonFungibleTokenBurning for NonFungibleToken {
    fn burn(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
    ) -> Token {
        let token = self.internal_burn(
            token_id,
            token_owner_id,
        );
        NftBurn { owner_id: &token.owner_id, token_ids: &[&token.token_id], authorized_id: None, memo: None }.emit();
        token
    }

    fn internal_burn(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
    ) -> Token {

        let owner_id = self.owner_by_id.get(&token_id).unwrap_or_else(|| {
            env::panic_str("token_id not found");
        });
        if token_owner_id != owner_id {
            env::panic_str("token_owner_id is not the token owner");
        }

        self.owner_by_id.remove(&token_id);

        let metadata = self.token_metadata_by_id
            .as_mut()
            .and_then(| by_id | by_id.remove(&token_id));

        Token { token_id, owner_id, metadata, None }

    }

    fn utility_burn(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
    ) -> Token {
        let token = self.internal_burn(
            token_id,
            token_owner_id,
        );
        let next_token_id = &token.token_id.parse::<u64>().unwrap() + 1;

        let token = self.internal_mint_with_refund(
            next_token_id.to_string(),
            token.owner_id,
            token.metadata,
            Some(env::predecessor_account_id()),
        );

        NftBurn { owner_id: &token.owner_id, token_ids: &[&token.token_id], authorized_id: None, memo: None }.emit();
        token
    }
}