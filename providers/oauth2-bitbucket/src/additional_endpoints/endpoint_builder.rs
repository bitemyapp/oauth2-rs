use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenProvider, AccessTokenResponseSuccessfulBody, EndpointBuilder,
        UserInfoObtainOutput,
    },
    re_exports::Scope,
};

use super::BitbucketUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct BitbucketEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for BitbucketEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_provider: AccessTokenProvider<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        return Ok(UserInfoObtainOutput::Respond(Box::new(
            BitbucketUserInfoEndpoint::new(&access_token.access_token),
        )));
    }
}