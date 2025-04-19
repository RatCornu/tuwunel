use std::path::PathBuf;

use serde::Deserialize;
use url::Url;

/// ## Examples:
///
/// - No LDAP login (default):
///
/// ```
/// [ldap]
/// enable = false
/// ```
#[derive(Clone, Debug, Deserialize)]
pub struct LdapConfig {
	/// Whether to enable LDAP login.
	///
	/// example: "true"
	#[serde(default)]
	pub enable: bool,

	/// URI of the LDAP server.
	///
	/// example: "ldap://ldap.example.com:389"
	#[serde(deserialize_with = "crate::utils::deserialize_from_str")]
	pub uri: Url,

	/// Whether to use StartTLS to bind to the LDAP server.
	///
	/// example: true
	#[serde(default)]
	pub start_tls: bool,

	/// Root of the searches.
	///
	/// example: "ou=users,dc=example,dc=org"
	pub base_dn: String,

	/// Bind DN if anonymous search is not enabled.
	///
	/// example: "cn=ldap-reader,dc=example,dc=org"
	#[serde(default)]
	pub bind_dn: Option<String>,

	/// Path to a file on the system that contains the password for the
	/// `bind_dn`.
	///
	/// The server must be able to access the file, and it must not be empty.
	#[serde(default)]
	pub bind_password_file: Option<PathBuf>,

	/// Search filter to limit user searches.
	///
	/// example: "(&(objectClass=person)(memberOf=matrix))"
	#[serde(default)]
	pub filter: Option<String>,

	/// Attribute to use to uniquely identify the user.
	///
	/// example: "uid" or "cn"
	///
	/// default: "uid"
	pub uid_attribute: String,

	/// Attribute containing the mail of the user.
	///
	/// example: "mail"
	pub mail_attribute: String,

	/// Attribute containing the distinguished name of the user.
	///
	/// example: "givenName" or "sn"
	pub name_attribute: String,
}
