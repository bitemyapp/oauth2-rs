use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfoObtainOutput,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::GoogleScope;

use super::GoogleUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct GoogleEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for GoogleEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_provider: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(UserInfoObtainOutput::Respond(Box::new(
            GoogleUserInfoEndpoint::new(
                &access_token.access_token,
                access_token.scope.to_owned().map(|x| {
                    ScopeParameter::<String>::from(&x)
                        .0
                        .contains(&GoogleScope::Openid.to_string())
                }) == Some(true),
            ),
        )))
    }
}
