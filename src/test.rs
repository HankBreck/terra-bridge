#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Api, DepsMut, Response,
    };

    use crate::{
        contract::instantiate,
        error::ContractError,
        execute::{try_update_super_users, try_update_collection_mappings, try_receive_nft},
        msg::{AdminsResponse, InstantiateMsg, OperatorsResponse, CollectionMapping, CollectionMappingResponse, HistoryResponse, BridgeRecordResponse},
        query::{query_admins, query_operators, query_collection_mappings, query_history}, state::BridgeRecord,
    };

    // Static variables for testing
    const CREATOR: &str = "creator";

    fn get_admins() -> Vec<String> {
        vec![
            CREATOR.to_string(),
            "champ".to_string(),
            "bobcat".to_string(),
        ]
    }

    fn get_admins_no_creator() -> Vec<String> {
        vec!["champ".to_string(), "bobcat".to_string()]
    }

    fn get_opers() -> Vec<String> {
        vec!["tommy".to_string(), "titan".to_string()]
    }

    /// Helper function to instantiate our contract for other tests
    fn do_instantiate(
        deps: DepsMut,
        admins: Vec<String>,
        operators: Vec<String>,
    ) -> Result<Response, ContractError> {
        let info = mock_info(&CREATOR, &[]);
        let env = mock_env();
        let msg = InstantiateMsg { admins, operators };
        instantiate(deps, env, info, msg)
    }

    #[test]
    fn proper_instantiation() {
        // Instantiate including creator in admins
        let mut deps = mock_dependencies(&[]);
        let admins = get_admins();
        let opers = get_opers();
        let result = do_instantiate(deps.as_mut(), admins.clone(), opers.clone());

        // we can just call .unwrap() to assert successful instantiation
        let response = result.unwrap();
        assert_eq!(response.attributes.len(), 2);
        assert_eq!(response.messages.len(), 0);

        // check admins match
        let response = query_admins(deps.as_ref()).unwrap();
        let success_response = AdminsResponse {
            admins: admins
                .iter()
                .map(|admin| deps.api.addr_validate(admin).unwrap())
                .collect::<Vec<Addr>>(),
        };
        assert_eq!(
            from_binary::<AdminsResponse>(&response).unwrap(),
            success_response
        );

        // check opers match
        let response = query_operators(deps.as_ref()).unwrap();
        let success_response = OperatorsResponse {
            operators: opers
                .iter()
                .map(|op| deps.api.addr_validate(op).unwrap())
                .collect::<Vec<Addr>>(),
        };
        assert_eq!(
            from_binary::<OperatorsResponse>(&response).unwrap(),
            success_response
        );

        // TODO: check history_pk is 0
        // Add config to state
        // Add config query
        // Add update for pause
    }

    #[test]
    fn omit_sender_instantiation() {
        // Instantiate including creator in admins
        let mut deps = mock_dependencies(&[]);
        let mut admins = get_admins_no_creator();
        let opers = get_opers();
        let result = do_instantiate(deps.as_mut(), admins.clone(), opers.clone());

        // we can just call .unwrap() to assert successful instantiation
        result.unwrap();

        admins.push(CREATOR.to_string());

        // check admins match
        let response = query_admins(deps.as_ref()).unwrap();
        let success_response = AdminsResponse {
            admins: admins
                .iter()
                .map(|admin| deps.api.addr_validate(admin).unwrap())
                .collect::<Vec<Addr>>(),
        };
        assert_eq!(
            from_binary::<AdminsResponse>(&response).unwrap(),
            success_response
        );
    }

    // add test for updating admins & operators
    // check for required permission (fails with not an admin)
    #[test]
    fn update_admins() {
        // Instantiate contract
        let mut deps = mock_dependencies(&[]);
        let initial_admins = get_admins();
        let initial_opers = get_opers();
        do_instantiate(deps.as_mut(), initial_admins, initial_opers).unwrap();

        /*
         * Verify admins were correctly updated when an admin sender is used
         */

        let info_success = mock_info(&CREATOR, &[]);
        let admins_add = vec!["willie".to_string()];
        let admins_rem = vec!["champ".to_string()];

        // Ensure TX succeeds
        let _ = try_update_super_users(
            deps.as_mut(),
            info_success,
            true,
            Some(admins_add.clone()),
            Some(admins_rem.clone()),
        )
        .unwrap();

        // Ensure admins vec does not contain "champ"
        let admins: AdminsResponse = from_binary(&query_admins(deps.as_ref()).unwrap()).unwrap();
        let success_res = AdminsResponse {
            admins: vec![
                deps.api.addr_validate(CREATOR).unwrap(),
                deps.api.addr_validate("bobcat").unwrap(),
                deps.api.addr_validate("willie").unwrap(),
            ],
        };
        assert_eq!(admins, success_res);

        /*
         * Verify admins are not updated when an non-admin sender is used
         */

        let info_fail = mock_info(&"nonadmin", &[]);
        // Try to reset admins back to initial_admins
        // Flipping admins_rem & admins_add from the previous test would reset the the contract's admin state
        // back to the initial value of initial_admins
        let err = try_update_super_users(
            deps.as_mut(),
            info_fail,
            true,
            Some(admins_rem),
            Some(admins_add),
        )
        .unwrap_err();
        assert_eq!(err.to_string(), String::from("Unauthorized"));

        // Ensure admins vec is unchanged
        let admins: AdminsResponse = from_binary(&query_admins(deps.as_ref()).unwrap()).unwrap();
        let fail_res = success_res;
        assert_eq!(admins, fail_res);
    }

    #[test]
    fn update_opers() {
        // Instantiate contract
        let mut deps = mock_dependencies(&[]);
        let initial_admins = get_admins();
        let initial_opers = get_opers();
        do_instantiate(deps.as_mut(), initial_admins.clone(), initial_opers).unwrap();

        /*
         * Verify opers were correctly updated when an admin sender is used
         */

        let info_success = mock_info(&CREATOR, &[]);
        let opers_add = vec!["willie".to_string()];
        let opers_rem = vec!["tommy".to_string()];

        // Ensure TX succeeds
        let _ = try_update_super_users(
            deps.as_mut(),
            info_success,
            false,
            Some(opers_add.clone()),
            Some(opers_rem.clone()),
        )
        .unwrap();

        // Ensure opers vec does not contain "tommy"
        let opers: OperatorsResponse =
            from_binary(&query_operators(deps.as_ref()).unwrap()).unwrap();
        let success_res = OperatorsResponse {
            operators: vec![
                deps.api.addr_validate("titan").unwrap(),
                deps.api.addr_validate("willie").unwrap(),
            ],
        };
        assert_eq!(opers, success_res);

        /*
         * Verify opers are not updated when an non-admin sender (including operators) is used
         */

        let info_fail = mock_info(&"nonadmin", &[]);
        // Try to reset operators back to initial_opers
        // Flipping opers_rem & opers_add from the previous test would reset the the contract's operator state
        // back to the initial value of initial_opers
        let err = try_update_super_users(
            deps.as_mut(),
            info_fail,
            false,
            Some(opers_rem),
            Some(opers_add),
        )
        .unwrap_err();
        assert_eq!(err.to_string(), String::from("Unauthorized"));

        // Ensure admins vec is unchanged
        let opers: OperatorsResponse =
            from_binary(&query_operators(deps.as_ref()).unwrap()).unwrap();
        let fail_res = success_res;
        assert_eq!(opers, fail_res);
    }

    #[test]
    fn update_collection_mappings() {
        // Instantiate contract
        let mut deps = mock_dependencies(&[]);
        let initial_admins = get_admins();
        let initial_opers = get_opers();
        do_instantiate(deps.as_mut(), initial_admins.clone(), initial_opers).unwrap();

        // Verify mappings count is 0
            // TODO: Add ContractInfo to hold this data
        
        /*
         * Non-admin user cannot update collection mappings 
         */

        let info_fail = mock_info("tommy", &[]);
        let add_list = vec![
            CollectionMapping { source: "terra contract 1".to_string(), destination: "secret contract 1".to_string() },
            CollectionMapping { source: "terra contract 2".to_string(), destination: "secret contract 2".to_string() },
        ];
        let err = try_update_collection_mappings(deps.as_mut(), info_fail, None, Some(add_list.clone())).unwrap_err();
        assert_eq!(err.to_string(), "Unauthorized");


        /*
         * Admin can add items to the collection mappings
         */

        let info_success = mock_info(&CREATOR, &[]);
        try_update_collection_mappings(deps.as_mut(), info_success.clone(), None, Some(add_list.clone())).unwrap();
        
        let sources = vec!["terra contract 1".into(), "terra contract 2".into()];
        let dest_bin = query_collection_mappings(deps.as_ref(), sources).unwrap();
        let CollectionMappingResponse { destinations } = from_binary(&dest_bin).unwrap();
        
        let res_success = vec![
            deps.api.addr_validate("secret contract 1").unwrap(),
            deps.api.addr_validate("secret contract 2").unwrap(),
        ];
        assert_eq!(destinations, res_success);

        /*
         * Admin can remove items from the collection mappings
         */

        let rem_list = vec!["terra contract 1".to_string()];
        try_update_collection_mappings(deps.as_mut(), info_success.clone(), Some(rem_list), None).unwrap();

        let sources = vec!["terra contract 2".to_string()];
        let dest_bin = query_collection_mappings(deps.as_ref(), sources).unwrap();
        let CollectionMappingResponse { destinations } = from_binary(&dest_bin).unwrap();
        let res_success = vec![
            deps.api.addr_validate("secret contract 2").unwrap(),
        ];
        assert_eq!(destinations, res_success);

        /*
         * Removing and adding mappings for the same source collection removes the existing mapping
         * and replaces it with the new mapping
         */
        
        let rem_list = vec!["terra contract 2".to_string()];
        let add_list = vec![
            CollectionMapping { source: "terra contract 2".to_string(), destination: "secret contract 2.0".to_string() },
        ];
        try_update_collection_mappings(deps.as_mut(), info_success.clone(), Some(rem_list), Some(add_list)).unwrap();

        let sources = vec!["terra contract 2".to_string()];
        let dest_bin = query_collection_mappings(deps.as_ref(), sources).unwrap();
        let CollectionMappingResponse { destinations } = from_binary(&dest_bin).unwrap();
        assert_eq!(destinations.len(), 1);
        
        let res_success = vec![
            deps.api.addr_validate("secret contract 2.0").unwrap(),
        ];
        assert_eq!(destinations, res_success);
    }

    #[test]
    fn receive_nft() {
        // Instantiate contract
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(CREATOR, &[]);
        let env = mock_env();
        let initial_admins = get_admins();
        let initial_opers = get_opers();
        do_instantiate(deps.as_mut(), initial_admins.clone(), initial_opers).unwrap();

        /*
         * Message fails if the terra contract is not mapped to a secret contract
         */

        let terra_coll_addr = "terra contract";
        let info_contract = mock_info(terra_coll_addr, &[]);
        let sender = "terra wallet".to_string();
        let token_id = "0".to_string();
        let err = try_receive_nft(deps.as_mut(), env.clone(), info_contract.clone(), sender.clone(), token_id.clone()).unwrap_err();
        assert_eq!(err.to_string(), "Unauthorized collection");

        /*
         * Receives the NFT and saves a record of the transaction
         */

        // Add collection mapping for sender
        let add_list = vec![
            CollectionMapping { source: terra_coll_addr.into(), destination: "secret contract".into() }
        ];
        try_update_collection_mappings(deps.as_mut(), info.clone(), None, Some(add_list.clone())).unwrap();

        // Send NFT to the contract
        try_receive_nft(deps.as_mut(), env.clone(), info_contract, sender, token_id.clone()).unwrap();
        let response_bin = &query_history(deps.as_ref(), terra_coll_addr.into(), token_id.clone(), None, Some(1u8)).unwrap();
        let response: HistoryResponse = from_binary(&response_bin).unwrap();

        // Verify success
        let res_success = HistoryResponse {
            history: vec![ BridgeRecordResponse {
                is_bridged: false,
                is_released: false,
                token_id: token_id,
                source_address: "terra wallet".into(),
                source_collection: terra_coll_addr.into(),
                destination_collection: "secret contract".into(),
                block_height: env.block.height,
                block_time: env.block.time.seconds(),
            }]
        };
        assert_eq!(response, res_success);


    }

    // add test for cw721receive failing if the collection is not mapped
}
