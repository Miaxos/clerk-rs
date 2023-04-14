/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`create_session_token_from_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSessionTokenFromTemplateError {
	Status401(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_session`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSessionError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_session_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSessionListError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`revoke_session`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevokeSessionError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_session`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifySessionError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

pub struct Session;


impl Session {
	/// Creates a JSON Web Token(JWT) based on a session and a JWT Template name defined for your instance
	pub async fn create_session_token_from_template(
		clerk_configuration: &configuration::ClerkConfiguration,
		session_id: &str,
		template_name: &str,
	) -> Result<crate::models::CreateSessionTokenFromTemplate200Response, Error<CreateSessionTokenFromTemplateError>> {
		let local_var_configuration = clerk_configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/sessions/{session_id}/tokens/{template_name}",
			local_var_configuration.base_path,
			session_id = crate::apis::urlencode(session_id),
			template_name = crate::apis::urlencode(template_name)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateSessionTokenFromTemplateError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve the details of a session
	pub async fn get_session(
		clerk_configuration: &configuration::ClerkConfiguration,
		session_id: &str,
	) -> Result<crate::models::Session, Error<GetSessionError>> {
		let local_var_configuration = clerk_configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/sessions/{session_id}",
			local_var_configuration.base_path,
			session_id = crate::apis::urlencode(session_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetSessionError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Returns a list of all sessions. The sessions are returned sorted by creation date, with the newest sessions appearing first.
	pub async fn get_session_list(
		clerk_configuration: &configuration::ClerkConfiguration,
		client_id: Option<&str>,
		user_id: Option<&str>,
		status: Option<&str>,
		limit: Option<f32>,
		offset: Option<f32>,
	) -> Result<Vec<crate::models::Session>, Error<GetSessionListError>> {
		let local_var_configuration = clerk_configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!("{}/sessions", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = client_id {
			local_var_req_builder = local_var_req_builder.query(&[("client_id", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = user_id {
			local_var_req_builder = local_var_req_builder.query(&[("user_id", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = status {
			local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = limit {
			local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = offset {
			local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetSessionListError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Sets the status of a session as \"revoked\", which is an unauthenticated state. In multi-session mode, a revoked session will still be returned along with its client object, however the user will need to sign in again.
	pub async fn revoke_session(
		clerk_configuration: &configuration::ClerkConfiguration,
		session_id: &str,
	) -> Result<crate::models::Session, Error<RevokeSessionError>> {
		let local_var_configuration = clerk_configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/sessions/{session_id}/revoke",
			local_var_configuration.base_path,
			session_id = crate::apis::urlencode(session_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<RevokeSessionError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Returns the session if it is authenticated, otherwise returns an error.
	pub async fn verify_session(
		clerk_configuration: &configuration::ClerkConfiguration,
		session_id: &str,
		verify_session_request: Option<crate::models::VerifySessionRequest>,
	) -> Result<crate::models::Session, Error<VerifySessionError>> {
		let local_var_configuration = clerk_configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/sessions/{session_id}/verify",
			local_var_configuration.base_path,
			session_id = crate::apis::urlencode(session_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&verify_session_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<VerifySessionError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}

