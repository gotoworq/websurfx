//! This module provides the functionality to generate random user agent string.

use fake_useragent::{Browsers, UserAgentsBuilder};

/// A function to generate random user agent to improve privacy of the user.
///
/// # Returns
///
/// A randomly generated user agent string.
pub fn random_user_agent() -> String {
    UserAgentsBuilder::new()
        .cache(false)
        .dir("/tmp")
        .thread(1)
        .set_browsers(
            Browsers::new()
                .set_chrome()
                .set_safari()
                .set_edge()
                .set_firefox()
                .set_mozilla(),
        )
        .build()
        .random()
        .to_string()
}
