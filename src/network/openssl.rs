use openssl::ssl::Error;
use openssl::ssl::SslConnector;
use openssl::ssl::SslContext;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;
use openssl::ssl::SslVerifyMode;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio_openssl::SslStream;

pub type SslRead = ReadHalf<SslStream<TcpStream>>;
pub type SslWrite = WriteHalf<SslStream<TcpStream>>;

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