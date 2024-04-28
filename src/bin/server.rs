extern crate image_post_practice;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            image_post_practice::routes::routes::upload_image
        ])
        .launch().await;
}