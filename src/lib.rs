/*!
This crate contains client bindings for the [Twitter API](https://developer.twitter.com/content/developer-twitter/en.html). It is meant to be an easy bridge from Rust to Twitter, and allow the easy creation of applications involving the Twitter API. 
 */
#![deny(missing_docs)]

extern crate url;

/// The authentication module handles all user authentication, including interfacing with the Twitter OAuth1A standard.
pub mod auth;

