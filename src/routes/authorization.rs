use crate::server::Server;
use async_trait::async_trait;
#[async_trait]
impl openapi::apis::ApiKeyAuthHeader for Server {
    type Claims = ();

    fn extract_claims_from_header<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        _headers: &'life1 axum::http::header::HeaderMap,
        _key: &'life2 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Option<Self::Claims>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
