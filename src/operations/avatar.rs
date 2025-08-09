use crate::operations::UploadAvatarOperation;
use crate::service::image::ImageType;
use crate::state::ApplicationState;
use axum::extract::{Multipart, State};
use openapi::apis::image::UploadAvatarResponse;
use openapi::models::UploadAvatarHeaderParams;


impl UploadAvatarOperation {
    pub fn activate<'async_trait, 'life5>(
        State(state): State<ApplicationState>,
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
        let image_service = state.image_service.as_ref();
        image_service.upload_image(body, ImageType::Avatar);
        todo!()
    }
}
