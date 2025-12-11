use std::net::SocketAddr;

use openssl::dh::Dh;
use openssl::ssl::Error;
use openssl::ssl::SslAcceptor;
use openssl::ssl::SslConnector;
use openssl::ssl::SslContext;
use openssl::ssl::SslMethod;
use openssl::ssl::SslOptions;
use openssl::ssl::SslVerifyMode;
use openssl::ssl::SslVersion;

// const FFDHE_2048: &str = "
// -----BEGIN DH PARAMETERS-----
// MIIBCAKCAQEA//////////+t+FRYortKmq/cViAnPTzx2LnFg84tNpWp4TZBFGQz
// +8yTnc4kmz75fS/jY2MMddj2gbICrsRhetPfHtXV/WVhJDP1H18GbtCFY2VVPe0a
// 87VXE15/V8k1mE8McODmi3fipona8+/och3xWKE2rec1MKzKT0g6eXq8CrGCsyT7
// YdEIqUuyyOP7uWrat2DX9GgdT0Kj3jlN9K5W7edjcrsZCwenyO4KbXCeAvzhzffi
// 7MA0BM0oNC9hkXL+nOmFg/+OTxIy7vKBg8P+OxtMb61zO7X8vC7CIAXFjvGDfRaD
// ssbzSibBsu/6iGtCOGEoXJf//////////wIBAg==
// -----END DH PARAMETERS-----
// ";

// pub fn create_server_ctx() -> Result<SslContext, Error> {
//     let mut ctx_builder = SslContext::builder(SslMethod::tls())?;

//     // let dh = Dh::params_from_pem(FFDHE_2048.as_bytes())?;
//     // ctx_builder.set_tmp_dh(&dh)?;
//     // // ctx_builder.
//     // ctx_builder.set_options(SslOptions::NO_TLSV1 | SslOptions::NO_TLSV1_1);
//     // ctx_builder.set_min_proto_version(Some(SslVersion::TLS1_3))?;

//     // ctx_builder.set_cipher_list(
//     //     "ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:\
//     //         ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305",
//     // )?;
//     // ctx_builder.set_groups_list("X25519")?;
//     // ctx_builder.set_ciphersuites(
//     //     "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256",
//     // )?;

//     // // ctx_builder.set_verify(SslVerifyMode::NONE);

//     // println!("Returnoinn g ok");

//     let ctx = ctx_builder.build();
//     // println!("{:#?}", ctx.set);

//     Ok(ctx)
// }

// pub fn create_client_ctx() -> Result<SslContext, Error> {
//     let mut ctx_builder = SslContext::builder(SslMethod::tls_client())?;

//     ctx_builder.set_options(SslOptions::NO_TLSV1 | SslOptions::NO_TLSV1_1);
//     ctx_builder.set_min_proto_version(Some(SslVersion::TLS1_2))?;
//     ctx_builder.set_cipher_list(
//         "ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:\
//             ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305",
//     )?;

//     ctx_builder.set_ciphersuites(
//         "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256",
//     )?;

//     // ctx_builder.set_verify(SslVerifyMode::NONE);

//     println!("Returnoinn g ok");
//     Ok(ctx_builder.build())
// }

pub fn create_server_ctx() -> Result<SslContext, Error>{
    return Ok(SslAcceptor::mozilla_modern(SslMethod::tls_server())?.build().into_context());
}

pub fn create_client_ctx() -> Result<SslContext, Error>{
    return Ok(SslConnector::builder(SslMethod::tls_client())?.build().into_context());
}

// pub fn mozilla_intermediate_v5(method: SslMethod) -> Result<SslAcceptorBuilder, ErrorStack> {
//         let mut ctx = ctx(method)?;
//         ctx.set_options(SslOptions::NO_TLSV1 | SslOptions::NO_TLSV1_1);
//         let dh = Dh::params_from_pem(FFDHE_2048.as_bytes())?;
//         ctx.set_tmp_dh(&dh)?;
//         setup_curves(&mut ctx)?;
//         ctx.set_cipher_list(
//             "ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:\
//              ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:\
//              DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384"
//         )?;
//         #[cfg(any(ossl111, libressl))]
//         ctx.set_ciphersuites(
//             "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256",
//         )?;
