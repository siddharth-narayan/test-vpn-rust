use std::net::SocketAddr;

use openssl::ssl::Error;
use openssl::ssl::SslContext;
use openssl::ssl::SslMethod;
use openssl::ssl::SslVerifyMode;

pub fn create_ctx() -> Result<SslContext, Error>{
    let mut ctx_builder = SslContext::builder(SslMethod::tls_client())?;
    ctx_builder.set_verify(SslVerifyMode::NONE);
    // let ctx = ctx_builder.set_groups_list(); ??

    Ok(ctx_builder.build())
}