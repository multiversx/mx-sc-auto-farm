// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           26
// Async Callback:                       1
// Total number of exported functions:  28

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    energy_dao
    (
        registerWrappedFarmToken
        registerUnstakeFarmToken
        addFarms
        removeFarms
        getFarmState
        getFarmingTokenId
        getFarmTokenId
        getDivisionSafetyConstant
        getWrappedFarmTokenId
        getUnstakeFarmTokenId
        getUnbondPeriod
        getPenaltyPercent
        enterFarm
        claimUserRewards
        unstakeFarm
        unbondFarm
        lockEnergyTokens
        claimFeesCollectorRewards
        setEnergyFactoryAddress
        getEnergyFactoryAddress
        issueWrappedToken
        setTransferRoleWrappedToken
        unsetTransferRoleWrappedToken
        getLockedTokenId
        getOldLockedTokenId
        getWrappedTokenId
        callBack
    )
}
