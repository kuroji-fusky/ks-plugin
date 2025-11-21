#![allow(non_snake_case)]

pub mod Workspace {
    pub fn add_legal_shit() {}
}

pub struct UserDetails {
    pub email: String,
    pub roles: Vec<String>,
}

pub struct UserInfraction {
    pub reason: String,
    pub duration: u32,
    pub enactedBy: UserDetails,
}

pub mod User {
    use super::{UserDetails, UserInfraction};

    pub fn search(user: &str, filters: Option<&str>) {}
    pub fn add(user: &str, perms: Option<&UserDetails>) {}

    pub fn remove(user: &str, perms: Option<&UserDetails>) {}

    pub fn kick(user: &str, infraction: Option<&UserInfraction>) {}
    pub fn ban(user: &str, infraction: Option<&UserInfraction>) {}

    pub fn update(user: &str, perms: Option<&UserDetails>) {}
}
