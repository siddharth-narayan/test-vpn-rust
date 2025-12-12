use std::net::SocketAddr;

use openssl::dh::Dh;
use openssl::ssl::Error;
use openssl::ssl::SslAcceptor;
use openssl::ssl::SslConnector;
use openssl::ssl::SslContext;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;
use openssl::ssl::SslOptions;
use openssl::ssl::SslVerifyMode;
use openssl::ssl::SslVersion;


pub fn create_server_ctx() -> Result<SslContext, Error> {
    let mut builder = SslConnector::builder(SslMethod::tls_server())?;
    builder.set_verify(SslVerifyMode::NONE);
    builder.set_certificate_file("cert.pem", SslFiletype::PEM)?;
    builder.set_private_key_file("key.pem", SslFiletype::PEM)?;

    Ok(builder.build().into_context())
}

pub fn create_client_ctx() -> Result<SslContext, Error> {
    let mut builder = SslConnector::builder(SslMethod::tls_client())?;
    builder.set_verify(SslVerifyMode::NONE);

    Ok(builder.build().into_context())
}