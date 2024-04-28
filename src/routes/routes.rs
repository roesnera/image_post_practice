use std::{fs, path::PathBuf};

use rocket::{data::Data, http::{ContentType, Status}, response::status::Custom, serde::json::{json, Value}};
use rocket_multipart_form_data::{mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};

use super::server_error;

#[rocket::post("/images/new", data = "<data>")]
pub async fn upload_image(content_type: &ContentType, data: Data<'_>) -> Result<Custom<Value>, Custom<Value>> {
  log::info!("Uploading image");
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("media").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap(),
    ]);

    log::info!("Parsing form data");
    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options).await {
      Ok(r) => r,
      Err(e) => {
          return Err(server_error(e.into()))
      }
  };

  match multipart_form_data.raw.remove("media") {
      Some(mut media) => {
          let file = media.remove(0);

          let filename = match file.file_name {
              Some(n) => n,
              None => match file.content_type {
                  Some(t) => format!(
                      "{}.{}",
                      generate_random_filename(),
                      t.subtype()
                  ),
                  None => todo!(),
              },
          };

          let filename_clone = filename.clone();

          let filepath = get_media_filepath(filename.into());
          log::info!("Saving file to {:?}", filepath);

          if let Some(p) = filepath.parent() {
              if let Err(e) = fs::create_dir_all(p) {
                  return Err(server_error(e.into()));
              }
          }
          log::info!("Reading and writing file");

          match fs::write(filepath, file.raw) {
                  Ok(_) => Ok(Custom(Status::Created, json!({ "filename": filename_clone }))),
                  Err(_) => Err(Custom(Status::BadRequest, json!({ "error": "media field is required" }))),
          }
      }
      None => Err(Custom(Status::BadRequest, json!({ "error": "media field is required" }))),
  }

}

fn generate_random_filename() -> String {
  use rand::distributions::Alphanumeric;
  use rand::{thread_rng, Rng};

  thread_rng()
      .sample_iter(&Alphanumeric)
      .map(char::from)
      .take(16)
      .collect()
}

fn get_media_filepath(filename: String) -> PathBuf {
  let mut path = std::env::current_dir().unwrap();
  path.push("images");
  path.push(filename);
  path
}