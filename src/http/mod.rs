//! Pieces pertaining to the HTTP message protocol.
use std::borrow::Cow;

use header::Connection;
use header::ConnectionOption::{KeepAlive, Close};
use header::Headers;
use version::HttpVersion;
use version::HttpVersion::{Http10, Http11};

pub use self::message::{HttpMessage, RequestHead, ResponseHead, Protocol};

pub mod h1;
pub mod h2;
pub mod message;

/// The raw status code and reason-phrase.
#[derive(Clone, PartialEq, Debug)]
pub struct RawStatus(pub u16, pub Cow<'static, str>);

/// Checks if a connection should be kept alive.
#[inline]
pub fn should_keep_alive(version: HttpVersion, headers: &Headers) -> bool {
    match (version, headers.get::<Connection>()) {
        (Http10, Some(conn)) if !conn.contains(&KeepAlive) => false,
        (Http11, Some(conn)) if conn.contains(&Close)  => false,
        _ => true
    }
}
