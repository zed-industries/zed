// Copyright (c) 2022 FL33TW00D (https://github.com/FL33TW00D)
// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE

#[cfg(features = "deflate")]
mod inner {
    use async_zip::write::ZipFileWriter;
    use async_zip::{Compression, ZipEntryBuilder};

    use std::path::Path;

    use actix_multipart::Multipart;
    use actix_web::{web, App, HttpServer, Responder, ResponseError, Result};
    use derive_more::{Display, Error};
    use futures::StreamExt;
    use futures_lite::io::AsyncWriteExt;
    use tokio::fs::File;
    use uuid::Uuid;

    const TMP_DIR: &str = "./tmp/";

    #[derive(Debug, Display, Error)]
    #[display("An error occurred during ZIP creation which was logged to stderr.")]
    struct CreationError;

    impl ResponseError for CreationError {}

    async fn do_main() -> std::io::Result<()> {
        let tmp_path = Path::new(TMP_DIR);

        if !tmp_path.exists() {
            tokio::fs::create_dir(tmp_path).await?;
        }

        let factory = || App::new().route("/", web::post().to(handler));
        HttpServer::new(factory).bind(("127.0.0.1", 8080))?.run().await
    }

    async fn handler(multipart: Multipart) -> Result<impl Responder, CreationError> {
        match create_archive(multipart).await {
            Ok(name) => Ok(format!("Successfully created archive: {}", name)),
            Err(err) => {
                eprintln!("[ERROR] {:?}", err);
                Err(CreationError)
            }
        }
    }

    async fn create_archive(mut body: Multipart) -> Result<String, anyhow::Error> {
        let archive_name = format!("tmp/{}", Uuid::new_v4());
        let mut archive = File::create(archive_name.clone()).await?;
        let mut writer = ZipFileWriter::new(&mut archive);

        while let Some(item) = body.next().await {
            let mut field = item?;

            let filename = match field.content_disposition().get_filename() {
                Some(filename) => sanitize_filename::sanitize(filename),
                None => Uuid::new_v4().to_string(),
            };

            let builder = ZipEntryBuilder::new(filename, Compression::Deflate);
            let mut entry_writer = writer.write_entry_stream(builder).await.unwrap();

            while let Some(chunk) = field.next().await {
                entry_writer.write_all_buf(&mut chunk?).await?;
            }

            entry_writer.close().await.unwrap();
        }

        writer.close().await.unwrap();
        archive.shutdown().await.unwrap();

        Ok(archive_name)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(features = "deflate")]
    {
        inner::do_main().await?;
    }

    Ok(())
}
