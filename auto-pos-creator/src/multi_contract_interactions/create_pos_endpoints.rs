use common_structs::PaymentsVec;

use crate::external_sc_interactions::pair_actions::PairTokenPayments;

use super::create_pos::{CreatePosArgs, StepsToPerform};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CreatePosEndpointsModule:
    crate::external_sc_interactions::pair_actions::PairActionsModule
    + crate::configs::pairs_config::PairsConfigModule
    + utils::UtilsModule
    + auto_farm::whitelists::farms_whitelist::FarmsWhitelistModule
    + auto_farm::whitelists::metastaking_whitelist::MetastakingWhitelistModule
    + auto_farm::external_storage_read::farm_storage_read::FarmStorageReadModule
    + auto_farm::external_storage_read::metastaking_storage_read::MetastakingStorageReadModule
    + crate::external_sc_interactions::farm_actions::FarmActionsModule
    + crate::external_sc_interactions::metastaking_actions::MetastakingActionsModule
    + super::create_pos::CreatePosModule
{
    #[payable("*")]
    #[endpoint(createPosFromSingleToken)]
    fn create_pos_from_single_token(
        &self,
        dest_pair_address: ManagedAddress,
        steps: StepsToPerform,
        buy_token_first_token_min_amount_out: BigUint,
        buy_token_second_token_min_amount_out: BigUint,
        add_liq_first_token_min_amount_out: BigUint,
        add_liq_second_token_min_amount_out: BigUint,
    ) -> PaymentsVec<Self::Api> {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        let double_swap_result = self.buy_half_each_token(
            payment,
            &dest_pair_address,
            buy_token_first_token_min_amount_out,
            buy_token_second_token_min_amount_out,
        );
        let args = CreatePosArgs {
            caller,
            dest_pair_address,
            pair_input_tokens: double_swap_result,
            steps,
            first_token_min_amount_out: add_liq_first_token_min_amount_out,
            second_token_min_amount_out: add_liq_second_token_min_amount_out,
        };

        self.create_pos_common(args)
    }

    /// Create pos from two payments, entering the pair for the two tokens
    /// It will try doing this with the optimal amounts,
    /// performing swaps before adding liqudity if necessary
    #[payable("*")]
    #[endpoint(createPosFromTwoTokens)]
    fn create_pos_from_two_tokens(
        &self,
        steps: StepsToPerform,
        swap_min_amount_out_first_token: BigUint,
        swap_min_amount_out_second_token: BigUint,
        add_liq_first_token_min_amount_out: BigUint,
        add_liq_second_token_min_amount_out: BigUint,
    ) -> PaymentsVec<Self::Api> {
        let caller = self.blockchain().get_caller();
        let [mut first_payment, mut second_payment] = self.call_value().multi_esdt();
        let wrapped_dest_pair_address = self.get_pair_address_for_tokens(
            &first_payment.token_identifier,
            &second_payment.token_identifier,
        );

        if wrapped_dest_pair_address.is_reverse() {
            core::mem::swap(&mut first_payment, &mut second_payment);
        }

        let dest_pair_address = wrapped_dest_pair_address.unwrap_address();
        let mut pair_input_tokens = PairTokenPayments {
            first_tokens: first_payment,
            second_tokens: second_payment,
        };
        self.balance_token_amounts_through_swaps(
            dest_pair_address.clone(),
            &mut pair_input_tokens,
            swap_min_amount_out_first_token,
            swap_min_amount_out_second_token,
        );

        let args = CreatePosArgs {
            caller,
            dest_pair_address,
            pair_input_tokens,
            steps,
            first_token_min_amount_out: add_liq_first_token_min_amount_out,
            second_token_min_amount_out: add_liq_second_token_min_amount_out,
        };

        self.create_pos_common(args)
    }
}
