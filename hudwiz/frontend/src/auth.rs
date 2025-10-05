use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
}
