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

// Helper that takes two DID Documents (identities) for issuer and subject, and
// creates a credential with claims about subject by issuer.
fn issue_vaccination(issuer: &IotaDocument, subject: &IotaDocument) -> Result<Credential> {
  // Create VC "subject" field containing subject ID and claims about it.
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject.id().as_str(),
    "vaccine": {
      "vaccine_type": "Pfizer-BioNTech",
      "batch_id": "6EQUJ5",
      "clinic_id": "LV-426"
    }
  }))?;

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .issuer(Url::parse(issuer.id().as_str())?)
    .type_("VaccinationCertificate")
    .subject(subject)
    .build()?;

  Ok(credential)
}

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize a `Client` to interact with the IOTA Tangle.
  let client: Client = Client::new().await?;

  // Create a signed DID Document/KeyPair for the credential issuer (see previous example).
  let (doc_iss, key_iss): (IotaDocument, KeyPair) = common::create_did_document(&client).await?;

  // Create a signed DID Document/KeyPair for the credential subject (see previous example).
  let (doc_sub, _key_sub): (IotaDocument, KeyPair) = common::create_did_document(&client).await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = issue_vaccination(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuer secret key - the result is a Verifiable Credential.
  doc_iss.sign_data(&mut credential, key_iss.secret())?;

  println!("Credential > {:#}", credential);
  println!();

  // Convert the Verifiable Credential to JSON and "exchange" with a verifier
  let message: String = credential.to_json()?;

  fs::write("/tmp/foo", &message).expect("Unable to write file");

  Ok(())
}
