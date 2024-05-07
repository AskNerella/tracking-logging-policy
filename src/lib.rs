// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;

use anyhow::{anyhow, Result};

use chrono::{Utc};
use pdk::hl::*;
use pdk::logger;

use crate::generated::config::Config;

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter(request_state: RequestState, _config: &Config) {

    let headers_state = request_state.into_headers_state().await;
    let header_name = _config.start_header_name.as_str();
    let path = headers_state.path();

    logger::info!("Request Path: {}", path);

    // get current time stamp
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    headers_state.handler().set_header(header_name, timestamp.as_str());
    logger::info!("Setting Request Header: {} - {}", header_name, timestamp);
}

async fn response_filter(response_state: ResponseState, _config: &Config) {
    let headers_state = response_state.into_headers_state().await;
    let header_name = _config.end_header_name.as_str();

    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    headers_state.handler().set_header(header_name, timestamp.as_str());
    logger::info!("Setting Response Header: {} - {}", header_name, timestamp);
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;

    // This filter will be called for every request and response
    let filter = on_request(|rs| request_filter(rs, &config)).on_response(|rs| response_filter(rs, &config));

    launcher.launch(filter).await?;

    Ok(())
}
