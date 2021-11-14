use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfo,
        UserInfoObtainOutput,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::{authorization_code_grant::KEY_OPENID, WechatScope};

use super::WechatUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct WechatEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for WechatEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        let has_snsapi_login_scope = access_token.scope.as_ref().map(|x| {
            ScopeParameter::<String>::from(x)
                .0
                .contains(&WechatScope::SnsapiLogin.to_string())
        }) == Some(true);

        if has_snsapi_login_scope {
            let openid = access_token
                .extensions()
                .ok_or("extensions missing")?
                .get(KEY_OPENID)
                .ok_or("openid missing")?
                .as_str()
                .ok_or("openid mismatch")?
                .to_owned();

            return Ok(UserInfoObtainOutput::Respond(Box::new(
                WechatUserInfoEndpoint::new(&access_token.access_token, openid),
            )));
        }

        match grant_info {
            GrantInfo::AuthorizationCodeGrant {
                provider: _,
                authorization_request_scopes: _,
            } => {
                let uid = access_token
                    .extensions()
                    .ok_or("extensions missing")?
                    .get(KEY_OPENID)
                    .ok_or("openid missing")?
                    .as_str()
                    .ok_or("openid mismatch")?
                    .to_owned();

                return Ok(UserInfoObtainOutput::Static(UserInfo {
                    uid,
                    name: None,
                    email: None,
                    raw: Default::default(),
                }));
            }
            GrantInfo::DeviceAuthorizationGrant {
                provider: _,
                authorization_request_scopes: _,
            } => {
                // unknown
            }
        }

        Ok(UserInfoObtainOutput::None)
    }
}
