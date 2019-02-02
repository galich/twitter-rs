use rand::Rng;
use std::fmt;
use url::Url;

use crate::auth::token::Token;

/// This implementation was adopted from https://github.com/azyobuzin/rust-oauthcli.

/// OAuthHeader stores the OAuth1A header for authentication purposes with the Twitter API.
pub struct OAuthHeader {
    /// The string contents of the header.
    pub contents: String,
}

impl OAuthHeader {
    /// Returns a reference to the contents of the header.
    pub fn get_header(&self) -> &str {
        &self.contents
    }

    /// Returns the contents of the header as owned.
    pub fn get_owned_header(self) -> String {
        self.contents
    }
}

impl fmt::Display for OAuthHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.contents)
    }
}

/// OAuthHeaderBuilder enables the user to construct an OAuthHeader for the OAuth1A Protocol.
pub struct OAuthHeaderBuilder<'a> {
    /// The HTTP method to use
    method: String,
    /// The parameters of the HTTP request
    params: Vec<(String, String)>,
    /// The user authentication token
    token: &'a Token,
}

impl<'a> OAuthHeaderBuilder<'a> {
    /// Create a new OAuthHeaderBuilder instance.
    pub fn new(method: String, token: &'a Token) -> Self {
        OAuthHeaderBuilder {
            method,
            params: Vec::new(),
            token,
        }
    }

    /// Add a set of request parameters to the header.
    pub fn parameters<P>(&mut self, parameters: P) -> &mut Self
    where
        P: IntoIterator<Item = (String, String)>,
    {
        self.params.extend(parameters.into_iter());
        self
    }

    /// Add a new parameter to the header.
    pub fn add_param(&mut self, key: String, value: String) {
        self.params.push((key, value));
    }

    /// Finalize the creation of the OAuthHeader.
    pub fn finalize(&mut self) -> OAuthHeader {
        self.add_param(
            "oauth_consumer_key".to_string(),
            self.token.consumer_key.clone(),
        );
        self.add_param("oauth_token".to_string(), self.token.access_token.clone());
        self.add_param("oauth_signature_method".to_string(), "HmacSha1".to_string());
        self.add_param(
            "oauth_timestamp".to_string(),
            time::now_utc().to_timespec().sec.to_string(),
        );

        unimplemented!();
    }
}
