use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};

#[post("/files/{file_id}")]
pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./{}", sanitize_filename::sanitize(&filename));
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        info!("{:?}", f);
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}
