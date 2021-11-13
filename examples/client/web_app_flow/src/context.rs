use std::{collections::HashMap, error};

use http_api_isahc_client::IsahcClient;
use oauth2_amazon::{
    AmazonEndpointBuilder, AmazonProviderWithWebServices, AmazonScope, AmazonTokenUrlRegion,
};
use oauth2_apple::AppleProviderWithAppleJs;
use oauth2_facebook::{FacebookEndpointBuilder, FacebookProviderForWebApp, FacebookScope};
use oauth2_github::{GithubEndpointBuilder, GithubProviderWithWebApplication, GithubScope};
use oauth2_google::{
    GoogleEndpointBuilder, GoogleProviderForWebServerApps,
    GoogleProviderForWebServerAppsAccessType, GoogleScope,
};
use oauth2_instagram::{
    InstagramEndpointBuilder, InstagramProviderForBasicDisplayApi, InstagramScope,
};
use oauth2_mastodon::{
    MastodonEndpointBuilder, MastodonProviderForEndUsers, MastodonScope, BASE_URL_MASTODON_SOCIAL,
};
use oauth2_signin::{
    oauth2_client::additional_endpoints::DefaultEndpointBuilder, web_app::SigninFlow,
};
use oauth2_twitch::{TwitchEndpointBuilder, TwitchProviderForWebServerApps, TwitchScope};

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
                GithubEndpointBuilder,
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
                vec![
                    GoogleScope::Email,
                    GoogleScope::Profile,
                    GoogleScope::Openid,
                ],
                GoogleEndpointBuilder,
            ),
        );
        signin_flow_map.insert(
            "twitch",
            SigninFlow::new(
                IsahcClient::new()?,
                TwitchProviderForWebServerApps::new(
                    clients_config.twitch.client_id.to_owned(),
                    clients_config.twitch.client_secret.to_owned(),
                    clients_config.twitch.redirect_uri.to_owned(),
                )?,
                vec![TwitchScope::UserReadEmail],
                TwitchEndpointBuilder,
            ),
        );
        signin_flow_map.insert(
            "mastodon-social",
            SigninFlow::new(
                IsahcClient::new()?,
                MastodonProviderForEndUsers::new(
                    BASE_URL_MASTODON_SOCIAL,
                    clients_config.mastodon_social.client_id.to_owned(),
                    clients_config.mastodon_social.client_secret.to_owned(),
                    clients_config.mastodon_social.redirect_uri.to_owned(),
                )?,
                vec![MastodonScope::Read, MastodonScope::Write],
                MastodonEndpointBuilder,
            ),
        );
        signin_flow_map.insert(
            "apple",
            SigninFlow::new(
                IsahcClient::new()?,
                AppleProviderWithAppleJs::new(
                    clients_config.apple.client_id.to_owned(),
                    clients_config.apple.client_secret.to_owned(),
                    clients_config.apple.redirect_uri.to_owned(),
                )?,
                None,
                DefaultEndpointBuilder,
            ),
        );
        signin_flow_map.insert(
            "instagram",
            SigninFlow::new(
                IsahcClient::new()?,
                InstagramProviderForBasicDisplayApi::new(
                    clients_config.instagram.client_id.to_owned(),
                    clients_config.instagram.client_secret.to_owned(),
                    clients_config.instagram.redirect_uri.to_owned(),
                )?,
                vec![InstagramScope::UserMedia, InstagramScope::UserProfile],
                InstagramEndpointBuilder,
            ),
        );
        signin_flow_map.insert(
            "facebook",
            SigninFlow::new(
                IsahcClient::new()?,
                FacebookProviderForWebApp::new(
                    clients_config.facebook.client_id.to_owned(),
                    clients_config.facebook.client_secret.to_owned(),
                    clients_config.facebook.redirect_uri.to_owned(),
                )?,
                vec![FacebookScope::Email, FacebookScope::PublicProfile],
                FacebookEndpointBuilder,
            ),
        );
        signin_flow_map.insert(
            "amazon",
            SigninFlow::new(
                IsahcClient::new()?,
                AmazonProviderWithWebServices::new(
                    clients_config.amazon.client_id.to_owned(),
                    clients_config.amazon.client_secret.to_owned(),
                    clients_config.amazon.redirect_uri.to_owned(),
                    AmazonTokenUrlRegion::NA,
                )?,
                vec![AmazonScope::Profile, AmazonScope::PostalCode],
                AmazonEndpointBuilder,
            ),
        );

        Ok(Self {
            config,
            signin_flow_map,
        })
    }
}
