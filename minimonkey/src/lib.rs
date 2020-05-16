#![crate_type = "lib"]

pub mod minimonkey;
pub mod provision;

pub use crate::minimonkey::read;
pub use crate::minimonkey::Response;
pub use crate::minimonkey::{add_admin_permission, revoke_admin_permission};
pub use crate::minimonkey::{add_login_permission, revoke_login_permission};
pub use crate::minimonkey::{add_publish_permission, revoke_publish_permission};
pub use crate::minimonkey::{add_subscribe_permission, revoke_subscribe_permission};
pub use crate::minimonkey::{authenticate, enter, publish, subscribe};
pub use crate::provision::{ProvisionInfo, Room};
