use crate::operations::UploadAvatarOperation;
use axum::extract::Multipart;
use openapi::apis::image::UploadAvatarResponse;
use openapi::models::UploadAvatarHeaderParams;

impl UploadAvatarOperation {
    pub fn activate<'async_trait, 'life5>(
        header_params: &'life5 UploadAvatarHeaderParams,
        body: Multipart,
    ) -> Result<UploadAvatarResponse, ()>
    where
        'life5: 'async_trait,
        Self: 'async_trait,
    {
        // return Ok(UploadAvatarResponse::Status200_OK {
        //     body: (

        //     ),
        //     trace_id: (),
        //     server_time: (),
        // });
        todo!()
    }
}
