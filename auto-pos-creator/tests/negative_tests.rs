#![allow(deprecated)]

use auto_pos_creator::{
    external_sc_interactions::router_actions::SwapOperationType,
    multi_contract_interactions::create_pos_endpoints::CreatePosEndpointsModule,
};
use multiversx_sc::types::{BigUint, ManagedBuffer, MultiValueEncoded};
use multiversx_sc_scenario::{managed_address, managed_token_id, rust_biguint, DebugApi};
use pos_creator_setup::{PosCreatorSetup, TOKEN_IDS};

pub mod metastaking_setup;
pub mod pair_setup;
pub mod pos_creator_setup;
pub mod router_setup;

pub static SWAP_TOKENS_FIXED_INPUT_FUNC_NAME: &[u8] = b"swapTokensFixedInput";

#[test]
fn try_create_lp_impossible_swap_path() {
    let pos_creator_setup = PosCreatorSetup::new(
        farm_with_locked_rewards::contract_obj,
        energy_factory::contract_obj,
        pair::contract_obj,
        router::contract_obj,
        farm_staking::contract_obj,
        farm_staking_proxy::contract_obj,
        auto_pos_creator::contract_obj,
    );
    let b_mock = pos_creator_setup.farm_setup.b_mock;

    let user_addr = pos_creator_setup.farm_setup.first_user;
    let user_first_token_balance = 200_000_000u64;
    b_mock.borrow_mut().set_esdt_balance(
        &user_addr,
        TOKEN_IDS[0],
        &rust_biguint!(user_first_token_balance),
    );

    // user enter (B, C) pair with token A
    let first_pair_addr = pos_creator_setup.pair_setups[0]
        .pair_wrapper
        .address_ref()
        .clone();
    let third_pair_addr = pos_creator_setup.pair_setups[2]
        .pair_wrapper
        .address_ref()
        .clone();
    b_mock
        .borrow_mut()
        .execute_esdt_transfer(
            &user_addr,
            &pos_creator_setup.pos_creator_wrapper,
            TOKEN_IDS[0],
            0,
            &rust_biguint!(user_first_token_balance),
            |sc| {
                let mut swap_operations = MultiValueEncoded::new();
                let swap_operation: SwapOperationType<DebugApi> = (
                    managed_address!(&first_pair_addr),
                    ManagedBuffer::from(SWAP_TOKENS_FIXED_INPUT_FUNC_NAME),
                    managed_token_id!("RAND-123456"),
                    BigUint::from(1u64),
                )
                    .into();
                swap_operations.push(swap_operation);
                let _ = sc.create_lp_pos_from_single_token(
                    managed_address!(&third_pair_addr),
                    1u32.into(),
                    1u32.into(),
                    swap_operations,
                );
            },
        )
        .assert_user_error("Invalid tokens");
}