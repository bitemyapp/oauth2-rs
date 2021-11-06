use std::{collections::HashMap, error};

use http_api_isahc_client::IsahcClient;
use oauth2_github::{GithubProviderWithWebApplication, GithubScope, GithubUserInfoEndpoint};
use oauth2_google::{
    GoogleProviderForWebServerApps, GoogleProviderForWebServerAppsAccessType, GoogleScope,
    GoogleUserInfoEndpoint,
};
use oauth2_signin::web_app::SigninFlow;

use crate::config::Config;

pub struct Context {
    pub config: Config,
    pub signin_flow_map: HashMap<&'static str, SigninFlow<IsahcClient>>,
}

impl Context {
    pub fn new(config: Config) -> Result<Self, Box<dyn error::Error>> {
        let clients_config = config.clients_config.to_owned();

        let mut signin_flow_map = HashMap::new();
        signin_flow_map.insert(
            "github",
            SigninFlow::new(
                IsahcClient::new()?,
                GithubProviderWithWebApplication::new(
                    clients_config.github.client_id.to_owned(),
                    clients_config.github.client_secret.to_owned(),
                    clients_config.github.redirect_uri.to_owned(),
                )?,
                vec![GithubScope::PublicRepo, GithubScope::UserEmail],
                GithubUserInfoEndpoint,
            ),
        );
        signin_flow_map.insert(
            "google",
            SigninFlow::new(
                IsahcClient::new()?,
                GoogleProviderForWebServerApps::new(
                    clients_config.google.client_id.to_owned(),
                    clients_config.google.client_secret.to_owned(),
                    clients_config.google.redirect_uri.to_owned(),
                )?
                .configure(|mut x| {
                    x.access_type = Some(GoogleProviderForWebServerAppsAccessType::Offline);
                    x.include_granted_scopes = Some(true);
                }),
                vec![GoogleScope::Email, GoogleScope::DriveFile],
                GoogleUserInfoEndpoint,
            ),
        );

        Ok(Self {
            config,
            signin_flow_map,
        })
    }
}