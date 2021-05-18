// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, then creates a Verifiable Credential (vc) specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example verifiable_credential

mod common;

use identity::core::json;
use identity::core::FromJson;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::CredentialValidation;
use identity::iota::CredentialValidator;
use identity::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize a `Client` to interact with the IOTA Tangle.
  let client: Client = Client::new().await?;

  let message = fs::read_to_string("/tmp/foo").expect("Unable to read file");

  // Create a `CredentialValidator` instance that will fetch
  // and validate all associated documents from the IOTA Tangle.
  let validator: CredentialValidator = CredentialValidator::new(&client);

  // Perform the validation operation.
  let validation: CredentialValidation = validator.check(&message).await?;

  println!("Credential Validation > {:#?}", validation);
  println!();

  Ok(())
}
