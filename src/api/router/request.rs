use std::str;

use axum::{RequestExt, RequestPartsExt, extract::Path};
use bytes::Bytes;
use http::request::Parts;
use serde::Deserialize;
use tuwunel_core::{Result, err};
use tuwunel_service::Services;

#[derive(Deserialize)]
pub(super) struct QueryParams {
	pub(super) access_token: Option<String>,
	pub(super) user_id: Option<String>,
}

pub(super) struct Request {
	pub(super) path: Path<Vec<String>>,
	pub(super) query: QueryParams,
	pub(super) body: Bytes,
	pub(super) parts: Parts,
}

pub(super) async fn from(
	services: &Services,
	request: hyper::Request<axum::body::Body>,
) -> Result<Request> {
	let limited = request.with_limited_body();
	let (mut parts, body) = limited.into_parts();

	let path: Path<Vec<String>> = parts.extract().await?;
	let query = parts.uri.query().unwrap_or_default();
	let query = serde_html_form::from_str(query)
		.map_err(|e| err!(Request(Unknown("Failed to read query parameters: {e}"))))?;

	let max_body_size = services.server.config.max_request_size;

	let body = axum::body::to_bytes(body, max_body_size)
		.await
		.map_err(|e| err!(Request(TooLarge("Request body too large: {e}"))))?;

	Ok(Request { path, query, body, parts })
}
