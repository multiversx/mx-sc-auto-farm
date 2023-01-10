use pair::AddLiquidityResultType;

elrond_wasm::imports!();

const MIN_AMOUNT_OUT: u32 = 1;

pub struct DoubleSwapResult<M: ManagedTypeApi> {
    pub first_swap_tokens: EsdtTokenPayment<M>,
    pub second_swap_tokens: EsdtTokenPayment<M>,
}

pub struct PairAddLiqResult<M: ManagedTypeApi> {
    pub lp_tokens: EsdtTokenPayment<M>,
    pub first_tokens_remaining: EsdtTokenPayment<M>,
    pub second_tokens_remaining: EsdtTokenPayment<M>,
}

#[elrond_wasm::module]
pub trait PairActionsModule:
    crate::configs::pairs_config::PairsConfigModule + utils::UtilsModule
{
    fn buy_half_each_token(
        &self,
        input_tokens: EsdtTokenPayment,
        dest_pair: &ManagedAddress,
    ) -> DoubleSwapResult<Self::Api> {
        require!(input_tokens.token_nonce == 0, "Only fungible ESDT accepted");
        self.require_sc_address(dest_pair);

        let dest_pair_config = self.get_pair_config(dest_pair);
        let tokens_to_pair_mapper = self.pair_address_for_tokens(
            &dest_pair_config.first_token_id,
            &dest_pair_config.second_token_id,
        );
        require!(!tokens_to_pair_mapper.is_empty(), "Unknown pair SC");

        let first_amount = &input_tokens.amount / 2u32;
        let second_amount = &input_tokens.amount - &first_amount;

        let first_swap_tokens = self.perform_tokens_swap(
            input_tokens.token_identifier.clone(),
            first_amount,
            dest_pair_config.first_token_id,
        );
        let second_swap_tokens = self.perform_tokens_swap(
            input_tokens.token_identifier,
            second_amount,
            dest_pair_config.second_token_id,
        );

        DoubleSwapResult {
            first_swap_tokens,
            second_swap_tokens,
        }
    }

    fn perform_tokens_swap(
        &self,
        from_tokens: TokenIdentifier,
        from_amount: BigUint,
        to_tokens: TokenIdentifier,
    ) -> EsdtTokenPayment {
        if from_tokens == to_tokens {
            return EsdtTokenPayment::new(from_tokens, 0, from_amount);
        }

        let pair_address = self.get_pair_address_for_tokens(&from_tokens, &to_tokens);
        let payment = EsdtTokenPayment::new(from_tokens, 0, from_amount);

        self.call_pair_swap(pair_address, payment, to_tokens)
    }

    fn call_pair_swap(
        &self,
        pair_address: ManagedAddress,
        input_tokens: EsdtTokenPayment,
        requested_token_id: TokenIdentifier,
    ) -> EsdtTokenPayment {
        self.pair_proxy(pair_address)
            .swap_tokens_fixed_input(requested_token_id, MIN_AMOUNT_OUT)
            .with_esdt_transfer(input_tokens)
            .execute_on_dest_context()
    }

    fn call_pair_add_liquidity(
        &self,
        pair_address: ManagedAddress,
        first_tokens: EsdtTokenPayment,
        second_tokens: EsdtTokenPayment,
    ) -> PairAddLiqResult<Self::Api> {
        let first_token_full_amount = first_tokens.amount.clone();
        let second_token_full_amount = second_tokens.amount.clone();
        let raw_results: AddLiquidityResultType<Self::Api> = self
            .pair_proxy(pair_address)
            .add_liquidity(MIN_AMOUNT_OUT, MIN_AMOUNT_OUT)
            .with_esdt_transfer(first_tokens)
            .with_esdt_transfer(second_tokens)
            .execute_on_dest_context();

        let (lp_tokens, first_tokens_used, second_tokens_used) = raw_results.into_tuple();
        let first_tokens_remaining_amount = first_token_full_amount - first_tokens_used.amount;
        let second_tokens_remaining_amount = second_token_full_amount - second_tokens_used.amount;

        let first_tokens_remaining = EsdtTokenPayment::new(
            first_tokens_used.token_identifier,
            0,
            first_tokens_remaining_amount,
        );
        let second_tokens_remaining = EsdtTokenPayment::new(
            second_tokens_used.token_identifier,
            0,
            second_tokens_remaining_amount,
        );

        PairAddLiqResult {
            lp_tokens,
            first_tokens_remaining,
            second_tokens_remaining,
        }
    }

    #[proxy]
    fn pair_proxy(&self, sc_address: ManagedAddress) -> pair::Proxy<Self::Api>;
}