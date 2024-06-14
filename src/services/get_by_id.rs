use actix_web::{ get, web, HttpResponse, HttpRequest, HttpResponseBuilder, http::StatusCode };
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{ Database, Collection, bson::Document };


#[get("/getEventById/{id}")]
pub async fn get_event_by_id(client: web::Data<mongodb::Client>, req: HttpRequest) -> HttpResponse {

    let db: Database = client.database("db_final");
    let events: Collection<Document> = db.collection("eventos");

    let id_event: ObjectId = req.match_info().get("id").unwrap().parse().unwrap();

    match events.find_one(doc! { "_id" : id_event }, None).await {
        Ok(Some(event)) => {
            HttpResponseBuilder::new(StatusCode::OK).json(event)
        }
        Ok(None) => {
            return HttpResponseBuilder::new(StatusCode::NOT_FOUND).json("No se encontro este evento")
        }
        Err(e) => {
            HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(e.to_string())
        }
    }

}