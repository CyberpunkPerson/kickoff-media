use crate::server::Server;
use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use http::Method;
use openapi::{apis::image::*, models::*};

#[async_trait]
impl openapi::apis::image::Image for Server {
    type Claims = ();

    fn delete_avatar<'life0, 'life1, 'life2, 'life3, 'life4, 'life5, 'async_trait>(
        &'life0 self,
        method: &'life1 Method,
        host: &'life2 Host,
        cookies: &'life3 CookieJar,
        claims: &'life4 Self::Claims,
        header_params: &'life5 DeleteAvatarHeaderParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn Future<Output = Result<DeleteAvatarResponse, ()>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        'life4: 'async_trait,
        'life5: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    fn upload_avatar<'life0, 'life1, 'life2, 'life3, 'life4, 'life5, 'async_trait>(
        &'life0 self,
        method: &'life1 Method,
        host: &'life2 Host,
        cookies: &'life3 CookieJar,
        claims: &'life4 Self::Claims,
        header_params: &'life5 UploadAvatarHeaderParams,
        body: Multipart,
    ) -> ::core::pin::Pin<
        Box<dyn Future<Output = Result<UploadAvatarResponse, ()>> + Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        'life4: 'async_trait,
        'life5: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    fn upload_image_draft<'life0, 'life1, 'life2, 'life3, 'life4, 'life5, 'async_trait>(
        &'life0 self,
        method: &'life1 Method,
        host: &'life2 Host,
        cookies: &'life3 CookieJar,
        claims: &'life4 Self::Claims,
        header_params: &'life5 UploadImageDraftHeaderParams,
        body: Multipart,
    ) -> ::core::pin::Pin<
        Box<
            dyn Future<Output = Result<UploadImageDraftResponse, ()>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        'life4: 'async_trait,
        'life5: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
