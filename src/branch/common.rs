use anyhow::Result;
use surrealdb::{engine::any::Any, Surreal};

use crate::{input::SurrealdbConfiguration, surrealdb::create_surrealdb_client};

use super::constants::{BRANCH_NS, BRANCH_TABLE};

#[allow(deprecated)]
pub async fn create_branch_data_client(
    db_configuration: &SurrealdbConfiguration,
) -> Result<Surreal<Any>> {
    const BRANCH_DATA_NS: &str = "database";
    const BRANCH_DATA_DB: &str = "branching";

    let branch_data_db_configuration = SurrealdbConfiguration {
        address: db_configuration.address.clone(),
        url: db_configuration.url.clone(),
        username: db_configuration.username.clone(),
        password: db_configuration.password.clone(),
        ns: Some(BRANCH_DATA_NS.to_owned()),
        db: Some(BRANCH_DATA_DB.to_owned()),
    };

    let client = create_surrealdb_client(&branch_data_db_configuration).await?;
    Ok(client)
}

#[allow(deprecated)]
pub async fn create_branch_client(
    branch_name: &String,
    db_configuration: &SurrealdbConfiguration,
) -> Result<Surreal<Any>> {
    let branch_db_configuration = SurrealdbConfiguration {
        address: db_configuration.address.clone(),
        url: db_configuration.url.clone(),
        username: db_configuration.username.clone(),
        password: db_configuration.password.clone(),
        ns: Some(BRANCH_NS.to_owned()),
        db: Some(branch_name.to_owned()),
    };

    let client = create_surrealdb_client(&branch_db_configuration).await?;
    Ok(client)
}

pub async fn retrieve_existing_branch_names(
    branch_data_client: &Surreal<Any>,
) -> Result<Vec<String>> {
    let existing_branch_names: Vec<String> = branch_data_client
        .query(format!("SELECT VALUE name FROM {}", BRANCH_TABLE))
        .await?
        .take(0)?;

    Ok(existing_branch_names)
}
