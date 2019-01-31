use std::fmt;
use url::Url;

use crate::auth::token::Token;

/// OAuthHeader stores the OAuth1A header for authentication purposes with the Twitter API.
pub struct OAuthHeader {
    /// The string contents of the header.
    pub contents: String
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
    method: &'a str,
    /// The parameters of the HTTP request 
    params: Vec<(&'a str, &'a str)>,
    /// The user authentication token
    token: Token
}

impl<'a> OAuthHeaderBuilder<'a> {
    /// Create a new OAuthHeaderBuilder instance.
    pub fn new(method: &'a str, token: Token) -> Self {
        OAuthHeaderBuilder {
            method,
            params: Vec::new(),
            token
        }
    }

    /// Add a set of request parameters to the header.
    pub fn parameters<P>(&mut self, parameters: P) -> &mut Self
    where P: IntoIterator<Item=(&'a str, &'a str)>
    {
        self.params.extend(parameters.into_iter());
        self
    }

    /// Add a new parameter to the header.
    pub fn add_param(&mut self, key: &'a str, value: &'a str) {
        self.params.push((key, value));
    }

    /// Finalize the creation of the OAuthHeader.
    pub fn finalize() -> OAuthHeader {
        unimplemented!();
    }
}

