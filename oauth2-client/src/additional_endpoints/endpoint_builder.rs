use std::{error, fmt};

use dyn_clone::{clone_trait_object, DynClone};

use crate::re_exports::Scope;

use super::{builder::BuilderObtainUserInfoOutput, AccessTokenResponseSuccessfulBody, GrantInfo};

//
//
//
pub trait EndpointBuilder<SCOPE>: DynClone
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>>;
}

clone_trait_object!(<SCOPE> EndpointBuilder<SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> fmt::Debug for dyn EndpointBuilder<SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EndpointBuilder").finish()
    }
}

//
//
//
#[derive(Debug, Clone)]
pub struct DefaultEndpointBuilder;
impl<SCOPE> EndpointBuilder<SCOPE> for DefaultEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(BuilderObtainUserInfoOutput::None)
    }
}
