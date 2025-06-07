use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use tonic::body::BoxBody;

pub(crate) fn get_transport(
    validate_certificates: bool,
) -> Client<HttpsConnector<HttpConnector>, BoxBody> {
    let tls_connector = yaak_http::tls::get_config(validate_certificates);

    let mut http = HttpConnector::new();
    http.enforce_http(false);

    let https = HttpsConnector::from((http, tls_connector.into()));

    let client = Client::builder(TokioExecutor::new())
        .pool_max_idle_per_host(0)
        .http2_only(true)
        .build(https);

    client
}
