use actix_web::{ post, web, HttpResponse, HttpResponseBuilder };
use actix_web::http::StatusCode;
use mongodb::{ Database, Collection, bson::Document };

use crate::models::models::*;

#[post("/createEvent")]
pub async fn create_event(client: web::Data<mongodb::Client>, data: web::Json<Evento>) -> HttpResponse {

    let db: Database = client.database("db_final");
    let events: Collection<Document> = db.collection("eventos");

    let event: Evento = Evento {
        titulo: data.titulo.clone(),
        descripcion: data.descripcion.clone(),
        categoria: data.categoria.clone(),
        fecha: data.fecha.clone(),
        lugar: data.lugar.clone(),
        asistentes: data.asistentes.clone(),
        conferencistas: data.conferencistas.clone(),
        facultades: data.facultades.clone(),
        programa: data.programa.clone(),
        comentarios: data.comentarios.clone(),
    };

    let doc_event: Document = bson::to_document(&event).unwrap();

    match events.insert_one(doc_event, None).await {
        Ok(result) => {
            HttpResponseBuilder::new(StatusCode::OK).json("Insertado")
        }
        Err(e) => {
            HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(e.to_string())
        }
    }

}