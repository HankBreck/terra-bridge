#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        DepsMut, Response, from_binary, Api, StdResult, Addr, CanonicalAddr,
    };

    use crate::{contract::instantiate, msg::{InstantiateMsg, AdminsResponse, OperatorsResponse}, error::ContractError, query::{query_admins, query_operators}, execute::try_update_super_user};

    // Static variables for testing
    const CREATOR: &str = "creator";

    fn get_admins() -> Vec<String> {
        vec![CREATOR.to_string(), "champ".to_string(), "bobcat".to_string()]
    }

    fn get_admins_no_creator() -> Vec<String> {
        vec!["champ".to_string(), "bobcat".to_string()]
    }

    fn get_opers() -> Vec<String> {
        vec!["tommy".to_string(), "titan".to_string()]
    }

    /// Helper function to instantiate our contract for other tests
    fn do_instantiate(deps: DepsMut, admins: Vec<String>, operators: Vec<String>) -> Result<Response, ContractError> {
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
                .collect::<Vec<Addr>>()
        };
        assert_eq!(from_binary::<AdminsResponse>(&response).unwrap(), success_response);

        // check opers match
        let response = query_operators(deps.as_ref()).unwrap();
        let success_response = OperatorsResponse { 
            operators: opers
                .iter()
                .map(|op| deps.api.addr_validate(op).unwrap())
                .collect::<Vec<Addr>>()
        };
        assert_eq!(from_binary::<OperatorsResponse>(&response).unwrap(), success_response);
        
        // TODO: check history_pk is 0
            // Add config to state
            // Add config query
            // Add update for pause



    }

    // add test for instantiation with info.sender not in admins
        // check admins from state includes info.sender
    #[test]
    fn omit_sender_instantiation() {
        // Instantiate including creator in admins
        let mut deps = mock_dependencies(&[]);
        let mut admins = vec!["champ".to_string(), "bobcat".to_string()];
        let opers = vec!["tommy".to_string(), "titan".to_string()];
        let result = do_instantiate(deps.as_mut(), admins.clone(), opers.clone());

        // we can just call .unwrap() to assert successful instantiation
        let _ = result.unwrap();

        admins.push(CREATOR.to_string());

        // check admins match
        let response = query_admins(deps.as_ref()).unwrap();
        let success_response = AdminsResponse { 
            admins: admins
                .iter()
                .map(|admin| deps.api.addr_validate(admin).unwrap())
                .collect::<Vec<Addr>>()
        };
        assert_eq!(from_binary::<AdminsResponse>(&response).unwrap(), success_response);
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
        let _ = try_update_super_user(deps.as_mut(), info_success, true, Some(admins_add.clone()), Some(admins_rem.clone())).unwrap();
        
        // Ensure admins vec does not contain "champ"
        let admins: AdminsResponse = from_binary(&query_admins(deps.as_ref()).unwrap()).unwrap();
        let success_res = AdminsResponse {
            admins: vec![
                deps.api.addr_validate(CREATOR).unwrap(), 
                deps.api.addr_validate("bobcat").unwrap(),
                deps.api.addr_validate("willie").unwrap(),
            ]
        };
        assert_eq!(admins, success_res);

        /*
         * Verify admins are not updated when an non-admin sender is used
         */

        let info_fail = mock_info(&"nonadmin", &[]);
        // Try to reset admins back to initial_admins
            // Flipping admins_rem & admins_add from the previous test would reset the the contract's admin state
            // back to the initial value of initial_admins
        let err = try_update_super_user(deps.as_mut(), info_fail, true, Some(admins_rem), Some(admins_add)).unwrap_err();
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
        let _ = try_update_super_user(deps.as_mut(), info_success, false, Some(opers_add.clone()), Some(opers_rem.clone())).unwrap();
        
        // Ensure opers vec does not contain "tommy"
        let opers: OperatorsResponse = from_binary(&query_operators(deps.as_ref()).unwrap()).unwrap();
        let success_res = OperatorsResponse {
            operators: vec![
                deps.api.addr_validate("titan").unwrap(),
                deps.api.addr_validate("willie").unwrap(),
            ]
        };
        assert_eq!(opers, success_res);

        /*
         * Verify opers are not updated when an non-admin sender (including operators) is used
         */

        let info_fail = mock_info(&"nonadmin", &[]);
        // Try to reset operators back to initial_opers
            // Flipping opers_rem & opers_add from the previous test would reset the the contract's operator state
            // back to the initial value of initial_opers
        let err = try_update_super_user(deps.as_mut(), info_fail, false, Some(opers_rem), Some(opers_add)).unwrap_err();
        assert_eq!(err.to_string(), String::from("Unauthorized"));

        // Ensure admins vec is unchanged
        let opers: OperatorsResponse = from_binary(&query_operators(deps.as_ref()).unwrap()).unwrap();
        let fail_res = success_res;
        print!("Opers: {:?}", opers);
        print!("Response: {:?}", fail_res);
        assert_eq!(opers, fail_res);
    }

    // add test for cw721receive working as intended
    // add test for cw721receive failing if the collection is not mapped
}
