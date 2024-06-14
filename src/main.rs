use actix_web::{ web, App, http::header, HttpServer };
use mongodb::{ Client, options::{ClientOptions, ResolverConfig} };
use dotenvy::dotenv;
use std::env;

mod models;
mod services;
use crate::services::create_event::create_event;
use crate::services::delete_event::delete_event;
use crate::services::update_event::update_event;
use crate::services::get_all_events::get_all_events;
use crate::services::get_by_id::get_event_by_id;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    dotenv().ok();
    let url = env::var("DB_URL").expect("error");
    let port: u16 = env::var("PORT").expect("error").to_owned().parse().unwrap();

    let options = ClientOptions::parse_with_resolver_config(&url, ResolverConfig::cloudflare())
        .await.expect("Error in options");

    let client = Client::with_options(options).expect("Error create client");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(create_event)
            .service(delete_event)
            .service(update_event)
            .service(get_all_events)
            .service(get_event_by_id)

    })
        .bind(("0.0.0.0", port))?
        .run()
        .await

}
