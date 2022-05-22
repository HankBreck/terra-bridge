#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        DepsMut, Response,
    };

    use crate::{contract::instantiate, msg::InstantiateMsg, error::ContractError, query::query_admins};

    // Static variables for testing
    const CREATOR: &str = "creator";

    /// Helper function to instantiate our contract for other tests
    fn do_instantiate(deps: DepsMut, admins: Vec<String>, operators: Vec<String>) -> Result<Response, ContractError> {
        let info = mock_info(&CREATOR, &[]);
        let env = mock_env();
        let msg = InstantiateMsg { admins, operators };
        instantiate(deps, env, info, msg)
    }

    #[test]
    fn proper_instantiation() {
        // Perform instantiation with invalid addrs
        let mut deps = mock_dependencies(&[]);
        let admins = vec![CREATOR.to_string(), "champ".to_string(), "bobcat".to_string()];
        let opers = vec!["tommy".to_string(), "titan".to_string()];
        let result = do_instantiate(deps.as_mut(), admins, opers);

        // we can just call .unwrap() to assert successful instantiation
        let response = result.unwrap();
        assert_eq!(response.attributes.len(), 2);
        assert_eq!(response.messages.len(), 0);

        // confirm sucessful state initialization
        let response = query_admins(deps.as_ref()).unwrap();
        // check admins match
        // check opers match
        // check history_pk is 0
    }

    // add test for instantiation with info.sender not in admins
        // check admins from state includes info.sender
    
    // add test for updating admins & operators
        // check for required permission (fails with not an admin)

    // add test for cw721receive working as intended
    // add test for cw721receive failing if the collection is not mapped
}
