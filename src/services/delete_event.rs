use actix_web::{ delete, web, HttpResponse, HttpRequest, HttpResponseBuilder, http::StatusCode };
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{ Database, Collection, bson::Document };


#[delete("/deleteEvent/{id}")]
pub async fn delete_event(client: web::Data<mongodb::Client>, req: HttpRequest) -> HttpResponse {

    let db: Database = client.database("db_final");
    let events: Collection<Document> = db.collection("eventos");

    let id_event: ObjectId = req.match_info().get("id").unwrap().parse().unwrap();

    match events.delete_one(doc! { "_id" : id_event }, None).await {
        Ok(result) => {

            if result.deleted_count == 0 {
                return HttpResponseBuilder::new(StatusCode::NOT_FOUND).json("No se elimino este evento")
            } else {
                HttpResponseBuilder::new(StatusCode::OK).json("evento eliminado")
            }

        }
        Err(e) => {
            HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(e.to_string())
        }
    }

}