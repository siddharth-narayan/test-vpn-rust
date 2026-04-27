use std::io::Read;
use std::io::Write;

use openssl::ssl::Error;
use openssl::ssl::SslConnector;
use openssl::ssl::SslContext;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;
use openssl::ssl::SslStream;
use openssl::ssl::SslVerifyMode;

// pub type SslRead = ReadHalf<SslStream<TcpStream>>;
// pub type SslWrite = WriteHalf<SslStream<TcpStream>>;

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

pub struct BufferedSsl<T> {
    inner: SslStream<T>
}

impl<T: Read + Write> BufferedSsl<T> {
    pub fn new(stream: SslStream<T>) -> BufferedSsl<T> {
        Self {
            inner: stream
        }
    }
}

impl<T: Read + Write> Read for BufferedSsl<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.inner.ssl_read(buf) {
            Ok(s) => Ok(s),
            Err(_e) => Err(std::io::Error::last_os_error())
        }
    }
}

impl<T: Read + Write> Write for BufferedSsl<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.inner.ssl_write(buf) {
            Ok(s) => Ok(s),
            Err(_e) => Err(std::io::Error::last_os_error())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}