use actix_web::{ get, web, HttpResponse, HttpResponseBuilder, http::StatusCode };
use futures::TryStreamExt;
use mongodb::{ Database, Collection, bson::Document };


#[get("/getAllEvents")]
pub async fn get_all_events(client: web::Data<mongodb::Client>) -> HttpResponse {

    let db: Database = client.database("db_final");
    let events: Collection<Document> = db.collection("eventos");

    let mut list_events: Vec<Document> = [].to_vec();

    match events.find(None, None).await {
        Ok(mut events) => {

            while let Some(result) = events.try_next().await.expect("error") {
                list_events.push(result)
            }

            if list_events.is_empty() {
                return HttpResponseBuilder::new(StatusCode::NOT_FOUND).json("No se encontraron eventos")
            }

            HttpResponseBuilder::new(StatusCode::OK).json(list_events)
        }
        Err(e) => {
            HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(e.to_string())
        }
    }

}