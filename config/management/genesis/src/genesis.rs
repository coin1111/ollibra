// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::layout::Layout;
use diem_global_constants::{OPERATOR_KEY, OWNER_KEY};
use diem_management::{config::ConfigPath, constants, error::Error, secure_backend::SharedBackend};
use diem_transaction_builder::stdlib as transaction_builder;
use diem_types::{
    account_address,
    chain_id::ChainId,
    transaction::{Transaction, TransactionPayload},
};
use std::{fs::File, io::{Read, Write}, path::PathBuf};
use structopt::StructOpt;
use vm_genesis::{OperatorAssignment, OperatorRegistration};

//////// 0L ////////
use ol_types::{account::ValConfigs, genesis_proof::GenesisMiningProof};

/// Note, it is implicitly expected that the storage supports
/// a namespace but one has not been set.
#[derive(Debug, StructOpt)]
pub struct Genesis {
    #[structopt(flatten)]
    pub config: ConfigPath,
    #[structopt(long, required_unless("config"))]
    pub chain_id: Option<ChainId>,
    #[structopt(flatten)]
    pub backend: SharedBackend,
    #[structopt(long)]
    pub path: Option<PathBuf>,
    #[structopt(long)]
    pub layout_path: Option<PathBuf>,
}

impl Genesis {
    fn config(&self) -> Result<diem_management::config::Config, Error> {
        self.config
            .load()?
            .override_chain_id(self.chain_id)
            .override_shared_backend(&self.backend.shared_backend)
    }

    pub fn execute(self) -> Result<Transaction, Error> {
        ///////// 0L ////////
        // for a decentralized genesis allow the participants to set their own layout file (will not have a central repo providing one).
        // for dev and testnets layouts can be found on genesis repo
        let layout: Layout = match &self.layout_path {
          Some(p) => {
            println!("Getting genesis validator set from file: {:?}\n", &self.layout_path);
            let mut file = File::open(p).expect("could not open layout file");
            let mut layout = String::new();
            file.read_to_string(&mut layout).expect("could not read");
            Layout::parse(&layout)
            .map_err(|e| Error::UnableToParse(constants::LAYOUT, e.to_string()))?
          },
          None => self.layout()?

        };
        //TODO(LG): get layout optionally from own file.
        // let layout = 
        //////// 0L ////////        
        // let diem_root_key = self.diem_root_key(&layout)?;
        // let treasury_compliance_key = self.treasury_compliance_key(&layout)?;
        let operator_assignments = self.operator_assignments(&layout)?;
        let operator_registrations = self.operator_registrations(&layout)?;

        let chain_id = self.config()?.chain_id;
        // Only have an allowlist of stdlib scripts
        let script_policy = None;

        let genesis = vm_genesis::encode_genesis_transaction(
            //////// 0L ////////
            // diem_root_key,
            // treasury_compliance_key,
            None,
            None,
            //////// 0L end ////////            
            &operator_assignments,
            &operator_registrations,
            script_policy,
            chain_id,
        );

        if let Some(path) = self.path {
            let mut file = File::create(path).map_err(|e| {
                Error::UnexpectedError(format!("Unable to create genesis file: {}", e.to_string()))
            })?;
            let bytes = bcs::to_bytes(&genesis).map_err(|e| {
                Error::UnexpectedError(format!("Unable to serialize genesis: {}", e.to_string()))
            })?;
            file.write_all(&bytes).map_err(|e| {
                Error::UnexpectedError(format!("Unable to write genesis file: {}", e.to_string()))
            })?;
        }

        Ok(genesis)
    }

    //////// 0L //////// commented out
    // /// Retrieves the diem root key from the remote storage. Note, at this point in time, genesis
    // /// only supports a single diem root key.
    // pub fn diem_root_key(&self, layout: &Layout) -> Result<Ed25519PublicKey, Error> {
    //     let config = self.config()?;
    //     let storage = config.shared_backend_with_namespace(layout.diem_root.clone());
    //     storage.ed25519_key(DIEM_ROOT_KEY)
    // }

    /// Retrieves a layout from the remote storage.
    pub fn layout(&self) -> Result<Layout, Error> {
        let config = self.config()?;
        let storage = config.shared_backend_with_namespace(constants::COMMON_NS.into());

        let layout = storage.string(constants::LAYOUT)?;
        Layout::parse(&layout).map_err(|e| Error::UnableToParse(constants::LAYOUT, e.to_string()))
    }

    /// Produces a set of OperatorAssignments from the remote storage.
    pub fn operator_assignments(&self, layout: &Layout) -> Result<Vec<OperatorAssignment>, Error> {
        let config = self.config()?;
        let mut operator_assignments = Vec::new();

        for owner in layout.owners.iter() {
            let owner_storage = config.shared_backend_with_namespace(owner.into());
            let owner_key = owner_storage.ed25519_key(OWNER_KEY).ok();

            let operator_name = owner_storage.string(constants::VALIDATOR_OPERATOR)?;
            let operator_storage = config.shared_backend_with_namespace(operator_name.clone());
            let operator_key = operator_storage.ed25519_key(OPERATOR_KEY)?;
            let operator_account = account_address::from_public_key(&operator_key);

            //////// 0L ////////
            //In genesis the owner will sign this script, which assigns an operator to thier profile.
            let set_operator_script =
                transaction_builder::encode_set_validator_operator_script_function(
                    operator_name.as_bytes().to_vec(),
                    operator_account,
                )
                .into_script_function();

            //////// 0L ////////
            let profile: Option<ValConfigs> = match owner_storage.string(diem_global_constants::ACCOUNT_PROFILE) {
              Ok(s) => {
                serde_json::from_str(&s).ok()
              }
              Err(_) => None
            };

            let pow = GenesisMiningProof {
                preimage: owner_storage.string(diem_global_constants::PROOF_OF_WORK_PREIMAGE).unwrap(),
                proof: owner_storage.string(diem_global_constants::PROOF_OF_WORK_PROOF).unwrap(),
                profile,
            };

            let owner_name_vec = owner.as_bytes().to_vec();
            operator_assignments.push(
                (
                    owner_key, 
                    owner_name_vec, 
                    set_operator_script,  
                    //////// 0L ////////
                    pow,
                    operator_account,
                )
            );
        }

        Ok(operator_assignments)
    }

    /// Produces a set of OperatorRegistrations from the remote storage.
    pub fn operator_registrations(
        &self,
        layout: &Layout,
    ) -> Result<Vec<OperatorRegistration>, Error> {
        let config = self.config()?;
        let mut registrations = Vec::new();

        //////// 0L ////////
        for operator_name in layout.operators.iter() {
            let operator_storage = config.shared_backend_with_namespace(operator_name.into());
            let operator_key = operator_storage.ed25519_key(OPERATOR_KEY)?;
            let validator_config_tx = operator_storage.transaction(constants::VALIDATOR_CONFIG)?;
            let validator_config_tx = validator_config_tx.as_signed_user_txn().unwrap().payload();
            let validator_config_tx =
                if let TransactionPayload::ScriptFunction(script_function) = validator_config_tx {
                    script_function.clone()
                } else {
                    return Err(Error::UnexpectedError("Found invalid registration".into()));
                };

            let operator_account = account_address::from_public_key(&operator_key);                
            registrations.push((
                operator_key,
                operator_name.as_bytes().to_vec(),
                validator_config_tx,
                operator_account,              
            ));
        }
        //////// 0L end ////////

        Ok(registrations)
    }

    //////// 0L ////////
    // /// Retrieves the treasury root key from the remote storage.
    // pub fn treasury_compliance_key(&self, layout: &Layout) -> Result<Ed25519PublicKey, Error> {
    //     let config = self.config()?;
    //     let storage = config.shared_backend_with_namespace(layout.diem_root.clone());
    //     storage.ed25519_key(diem_global_constants::TREASURY_COMPLIANCE_KEY)
    // }
}
