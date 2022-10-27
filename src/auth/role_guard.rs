use std::str::FromStr;

use async_graphql::{async_trait, Context, Error as GraphError, Guard};

use crate::{constants::messages, models::custom_error::CustomError};

use super::jwt::Role;

pub type AuthRole = Role;

pub struct RoleGuard {
    role: AuthRole,
}

impl RoleGuard {
    pub(crate) fn new(role: AuthRole) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), GraphError> {
        let maybe_getting_role_result = ctx.data_opt::<Result<Option<AuthRole>, CustomError>>();
        match maybe_getting_role_result {
            Some(getting_role_result) => {
                let check_role_result = check_user_role_is_allowed(getting_role_result, &self.role);
                match check_role_result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(GraphError::new(e.message)),
                }
            }
            None => Err(messages::MESSAGE_INVALID_ROLE.into()),
        }
    }
}

fn check_user_role_is_allowed(
    getting_role_result: &Result<Option<Role>, CustomError>,
    allowed_role: &Role,
) -> Result<(), CustomError> {
    let maybe_role = match getting_role_result {
        Ok(maybe_role) => maybe_role,
        Err(e) => {
            return Err(format!("Error while getting a user's role: {}", e.message)
                .as_str()
                .into())
        }
    };

    match maybe_role {
        Some(role) => {
            if role == allowed_role {
                Ok(())
            } else {
                Err(messages::MESSAGE_INVALID_ROLE.into())
            }
        }
        None => Err(messages::MESSAGE_INVALID_ROLE.into()),
    }
}

pub fn get_role(role: String) -> Result<Option<Role>, CustomError> {
    let mut role_header_value = Option::<String>::default();

    if !String::is_empty(&role) {
        role_header_value = Option::<String>::Some(role);
    }

    match role_header_value {
        Some(header_value) => Ok(Some(
            Role::from_str(header_value.as_str()).unwrap_or(Role::User),
        )),
        None => Ok(None),
    }
}
