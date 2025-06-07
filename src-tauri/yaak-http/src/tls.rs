use native_tls::TlsConnector;

pub fn get_config(validate_certificates: bool) -> TlsConnector {
    let mut builder = TlsConnector::builder();

    if !validate_certificates {
        builder.danger_accept_invalid_certs(true);
    }

    builder.build().expect("Failed to build TLS connector")
}
