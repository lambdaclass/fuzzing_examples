#[macro_use]
extern crate honggfuzz;
use std::{collections::HashMap, path::PathBuf};

extern crate starknet_rs;
use felt::Felt;
use std::io::{BufReader, Read};
use num_traits::Zero;
use starknet_rs::services::api::contract_class_errors::ContractClassError;


fn main() {
    println!("Starting fuzzer");
    loop {
        fuzz!(|data: &[u8]| {
        // ---------------------------------------------------------
        //  Create program and entry point types for contract class
        // ---------------------------------------------------------
        // let reader = BufReader::new(data);
        let contract_class = starknet_rs::services::api::contract_class::ContractClass::try_from(data).unwrap();

        let entry_points_by_type = contract_class.entry_points_by_type.clone();

        let fib_entrypoint_selector = entry_points_by_type
            .get(&starknet_rs::services::api::contract_class::EntryPointType::External)
            .unwrap()
            .get(0)
            .unwrap()
            .selector
            .clone();
        

        //* --------------------------------------------
        //*    Create state reader with class hash data
        //* --------------------------------------------

        let ffc = starknet_rs::starknet_storage::dict_storage::DictStorage::new();
        let contract_class_storage = starknet_rs::starknet_storage::dict_storage::DictStorage::new();
        let mut contract_class_cache = HashMap::new();

        //  ------------ contract data --------------------

        let address = starknet_rs::utils::Address(1111.into());
        let class_hash = [1; 32];
        let contract_state = starknet_rs::business_logic::fact_state::contract_state::ContractState::new(class_hash, 3.into(), HashMap::new());

        contract_class_cache.insert(class_hash, contract_class);
        let mut state_reader = starknet_rs::business_logic::fact_state::in_memory_state_reader::InMemoryStateReader::new(ffc, contract_class_storage);
        state_reader
            .contract_states
            .insert(address.clone(), contract_state);

        //* ---------------------------------------
        //*    Create state with previous data
        //* ---------------------------------------

        let mut state = starknet_rs::business_logic::state::cached_state::CachedState::new(state_reader, Some(contract_class_cache));

        //* ------------------------------------
        //*    Create execution entry point
        //* ------------------------------------

        let calldata = [1.into(), 1.into(), 10.into()].to_vec();
        let caller_address = starknet_rs::utils::Address(0000.into());
        let entry_point_type = starknet_rs::services::api::contract_class::EntryPointType::External;
        let entrypoint = starknet_rs::business_logic::execution::execution_entry_point::ExecutionEntryPoint::new(
            address,
            calldata.clone(),
            fib_entrypoint_selector.clone(),
            caller_address,
            entry_point_type,
            Some(starknet_rs::business_logic::execution::objects::CallType::Delegate),
            Some(class_hash),
        );

        //* --------------------
        //*   Execute contract
        //* ---------------------
        let general_config = starknet_rs::definitions::general_config::StarknetGeneralConfig::default();
        let tx_execution_context = starknet_rs::business_logic::execution::objects::TransactionExecutionContext::new(
            starknet_rs::utils::Address(0.into()),
            Felt::zero(),
            Vec::new(),
            0,
            10.into(),
            general_config.invoke_tx_max_n_steps,
            starknet_rs::definitions::constants::TRANSACTION_VERSION,
        );
        let mut resources_manager = starknet_rs::business_logic::fact_state::state::ExecutionResourcesManager::default();

        entrypoint.execute(
            &mut state,
            &general_config,
            &mut resources_manager,
            &tx_execution_context
        )
        .unwrap();
        });
    }
}
