use jwtk::ecdsa::EcdsaPrivateKey;
use log::info;
use std::{env::var, error::Error};

use crate::utils::get_pkey;

pub(crate) fn exec() ->  Result<(bool, String, EcdsaPrivateKey), Box<dyn Error>> {
    //
    //
    // load API key for selected environment
    //
    let apikey: String = var("PT_API_KEY").expect("PT_API_KEY must be set in the environment or .env file");
    info!("Using API Key: {apikey}");

    //
    // retrieve private key from local pem file
    //
    let pkey: EcdsaPrivateKey = get_pkey();

    Ok((true, apikey, pkey))
}