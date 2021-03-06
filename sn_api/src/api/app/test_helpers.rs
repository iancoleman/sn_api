// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crate::Safe;
use anyhow::{Context, Result};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env::var;

// Environment variable which can be set with the auth credentials
// to be used for all sn_api tests
const TEST_AUTH_CREDENTIALS: &str = "TEST_AUTH_CREDENTIALS";

// Instantiate a Safe instance
pub async fn new_safe_instance() -> Result<Safe> {
    let mut safe = Safe::default();
    let credentials = match var(TEST_AUTH_CREDENTIALS) {
        Ok(val) => {
            let keypair = serde_json::from_str(&val).with_context(|| {
                format!(
                    "Failed to parse credentials read from {} env var",
                    TEST_AUTH_CREDENTIALS
                )
            })?;
            Some(keypair)
        }
        Err(_) => None,
    };

    safe.connect(credentials, None).await?;
    Ok(safe)
}

// Create a random NRS name
pub fn random_nrs_name() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(15).collect()
}
