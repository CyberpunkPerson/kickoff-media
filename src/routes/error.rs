use openapi::apis::ErrorHandler;

use crate::server::Server;
impl ErrorHandler<()> for Server {
    fn handle_error<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        method: &'life1 ::http::Method,
        host: &'life2 axum_extra::extract::Host,
        cookies: &'life3 axum_extra::extract::CookieJar,
        error: (),
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<axum::response::Response, http::StatusCode>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: ::core::marker::Sync + 'async_trait,
    {
        todo!()
    }
}
