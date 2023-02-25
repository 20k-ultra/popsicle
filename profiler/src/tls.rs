use std::sync::Arc;

use rustls::{ClientConnection, OwnedTrustAnchor, RootCertStore, ServerName};

pub fn init_connection(domain: ServerName) -> Result<ClientConnection, rustls::Error> {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    rustls::ClientConnection::new(Arc::new(config), domain)
}
