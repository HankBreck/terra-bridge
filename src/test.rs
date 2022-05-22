#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        DepsMut,
    };

    use crate::{contract::instantiate, msg::InstantiateMsg};

    // Static variables for testing
    const CREATOR: &str = "creator";

    /// Helper function to instantiate our contract for other tests
    fn do_instantiate(deps: DepsMut, admins: Vec<String>, operators: Vec<String>) {
        let info = mock_info(&CREATOR, &[]);
        let env = mock_env();
        let msg = InstantiateMsg { admins, operators };
        instantiate(deps, env, info, msg).unwrap();
    }

    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies(&[]);
        do_instantiate(deps.as_mut(), vec!["Hank".into(), "Ellemer".into()], vec![]);
        // Confirm the admins and operators are stored correctly
    }
}
