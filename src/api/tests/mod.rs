use crate::api::versions::{available_versions, versions};
use crate::api::Api;
use crate::models::version::Version;

mod items;
mod recipes;
mod versions;

fn get_api(version: Version) -> Api {
    Api::new(version)
}

fn get_test_versions() -> Vec<Version> {
    let available = available_versions().unwrap();
    versions()
        .unwrap()
        .into_iter()
        .filter(|v| available.contains(&v.minecraft_version))
        .collect()
}
