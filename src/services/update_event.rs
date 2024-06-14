use actix_web::{ put, web, HttpResponse, HttpRequest, HttpResponseBuilder, http::StatusCode };
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{ Database, Collection, bson::Document };

use crate::models::models::EventoUpdate;


#[put("/updateEvent/{id}")]
pub async fn update_event(client: web::Data<mongodb::Client>, req: HttpRequest, data: web::Json<EventoUpdate>) -> HttpResponse {

    let db: Database = client.database("db_final");
    let events: Collection<Document> = db.collection("eventos");

    let id_event: ObjectId = req.match_info().get("id").unwrap().parse().unwrap();

    let event_doc = bson::to_document(&data).expect("error");

    match events.update_one(
        doc! { "_id" : id_event },
        { doc! {"$set" : event_doc.to_owned()} },
        None
    ).await {
        Ok(result) => {

            if result.matched_count == 0 {
                HttpResponseBuilder::new(StatusCode::NOT_FOUND).json("No encontro este evento")
            } else if result.modified_count == 0 {
                HttpResponseBuilder::new(StatusCode::OK).json("No actualizo este evento")
            } else {
                HttpResponseBuilder::new(StatusCode::OK).json("evento actualizado")
            }

        }
        Err(e) => {
            HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(e.to_string())
        }
    }

}