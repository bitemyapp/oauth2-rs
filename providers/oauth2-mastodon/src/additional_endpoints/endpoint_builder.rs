use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfoObtainOutput,
    },
    re_exports::Scope,
};

use super::MastodonUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct MastodonEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for MastodonEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        access_token_provider: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        let extensions = match access_token_provider {
            GrantInfo::AuthorizationCodeGrant {
                provider,
                authorization_request_scopes: _,
            } => provider.extensions(),
            GrantInfo::DeviceAuthorizationGrant {
                provider,
                authorization_request_scopes: _,
            } => provider.extensions(),
        };

        let base_url = extensions
            .map(|x| x.get("base_url").cloned())
            .ok_or("Missing base_url")?
            .ok_or("Missing base_url")?
            .as_str()
            .ok_or("Mismatch base_url")?
            .to_owned();

        Ok(UserInfoObtainOutput::Respond(Box::new(
            MastodonUserInfoEndpoint::new(base_url, &access_token.access_token)?,
        )))
    }
}
