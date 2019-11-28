use super::check;
use super::utils::{check_same_length, clean_0x, extract_to_u32, get_latest_key};

use cita_types::{Address, H256};
use cita_vm::evm::{InterpreterParams, InterpreterResult, Log};
use common_types::context::Context;
use common_types::errors::ContractError;

use super::contract::Contract;
use crate::rs_contracts::contracts::build_in_perm;
use crate::rs_contracts::contracts::group::Group;
use crate::rs_contracts::storage::db_contracts::ContractsDB;
use crate::rs_contracts::storage::db_trait::DataBase;
use crate::rs_contracts::storage::db_trait::DataCategory;

use cita_trie::DB;
use cita_vm::state::State;
use ethabi::param_type::ParamType;
use ethabi::Token;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::sync::Arc;
use tiny_keccak::keccak256;

use crate::cita_executive::create_address_from_address_and_nonce;
use cita_vm::state::StateObjectInfo;
use common_types::reserved_addresses;
use ethabi::token::LenientTokenizer;
use ethabi::token::Tokenizer;

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupStore {
    // key -> height, value -> json(PermissionManager)
    contracts: BTreeMap<u64, Option<String>>,
}

impl Default for GroupStore {
    fn default() -> GroupStore {
        PermStore {
            contracts: BTreeMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GroupManager {
    pub groups: HashMap<Address, Group>,
}

pub fn new_group<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - new group, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn delete_group<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - delete_group, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn update_group_name<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - update_group_name, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn update_group_name<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - update_group_name, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn add_accounts<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - add_accounts, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn delete_accounts<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - delete_accounts, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn check_scope<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - check_scope, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn query_groups<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - query_groups, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn delete_child<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - delete_child, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}

pub fn add_child<B: DB>(
    &mut self,
    params: &InterpreterParams,
    changed: &mut bool,
    _context: &Context,
    _contracts_db: Arc<ContractsDB>,
    state: Arc<RefCell<State<B>>>,
) -> Result<InterpreterResult, ContractError> {
    trace!("System contract - group - add_child, input {:?}", params.input);

    return Ok(InterpreterResult::Normal(
        H256::from(0).0.to_vec(),
        params.gas_limit,
        vec![],
    ));
}
