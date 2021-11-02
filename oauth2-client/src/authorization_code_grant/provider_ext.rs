use std::{error, fmt, str};

pub use oauth2_core::{
    access_token_request::{
        BodyWithAuthorizationCodeGrant as AccessTokenRequestBody,
        GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
    },
    access_token_response::ErrorBodyError as AccessTokenResponseErrorBodyError,
    authorization_code_grant::{
        access_token_response::{
            ErrorBody as AccessTokenResponseErrorBody,
            SuccessfulBody as AccessTokenResponseSuccessfulBody,
        },
        authorization_request::Query as AuthorizationRequestQuery,
    },
};
pub use serde_qs::{self, Error as SerdeQsError};

use crate::{
    provider::{Map, RedirectUri, Request, Response, Url, Value},
    Provider,
};

pub trait ProviderExtAuthorizationCodeGrant: Provider
where
    <<Self as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    fn redirect_uri(&self) -> Option<&RedirectUri>;

    fn authorization_endpoint_url(&self) -> &Url;

    fn authorization_request_query_extensions(&self) -> Option<Map<String, Value>> {
        None
    }

    fn authorization_request_query_serializing(
        &self,
        _query: &AuthorizationRequestQuery<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn error::Error>>> {
        None
    }

    fn access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        None
    }

    fn access_token_request_rendering(
        &self,
        _body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Vec<u8>>, Box<dyn error::Error>>> {
        None
    }

    fn access_token_response_parsing(
        &self,
        _response: &Response<Vec<u8>>,
    ) -> Option<
        Result<
            Result<
                AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
                AccessTokenResponseErrorBody,
            >,
            Box<dyn error::Error>,
        >,
    > {
        None
    }
}