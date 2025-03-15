use anyhow::anyhow;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Process {
    pub id: String,
    pub owners: Vec<String>,
}

impl Process {
    #[must_use]
    pub fn new(id: String, owners: Vec<String>) -> Self {
        Self { id, owners }
    }

    #[must_use]
    pub fn is_authorized(&self, owner: &str) -> bool {
        self.owners.contains(&owner.to_string())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionCounter {
    pub tc: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OwnerList {
    pub owners: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    pub data: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataTransaction {
    pub timestamp: i64,
    pub process_id: String,
    pub document_id: String,
    pub payload: String,
    pub client_id: String,
    pub clearing_house_version: String,
}

impl DataTransaction {
    /// Signs a `DataTransaction` with a given key on the `key_path` and returns a `Receipt`.
    ///
    /// # Errors
    /// Only if issues with reading the key or signing the `DataTransaction` occur.
    pub fn sign_jsonwebtoken(
        &self,
        cert_util: &ids_daps_cert::CertUtil,
    ) -> anyhow::Result<Receipt> {
        let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::PS512);
        header.typ = None;
        header.kid = cert_util.fingerprint().ok();

        let private_key_der = cert_util
            .private_key_der()
            .map_err(|e| anyhow!("Error getting private_key_der: {e}"))?;
        let private_key = jsonwebtoken::EncodingKey::from_rsa_der(&private_key_der);

        let data = jsonwebtoken::encode(&header, self, &private_key)?;

        Ok(Receipt { data })
    }
}
