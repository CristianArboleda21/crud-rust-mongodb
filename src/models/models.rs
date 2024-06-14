use serde::{Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Cuidad {
    pub nombre: String,
    pub departamento: String,
    pub pais : String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LugarEvento {
    pub nombre: String,
    pub direccion: String,
    pub cuidad : Cuidad
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AsisConfe {
    pub identificador: String,
    pub nombre_usuario: String,
    pub nombre_completo : String,
    pub relacion_institucion : String,
    pub email : String,
    pub cuidad : String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comentarios {
    pub nombre: String,
    pub comentario: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Evento {
    pub titulo: String,
    pub descripcion: String,
    pub categoria : Vec<String>,
    pub fecha: String,
    pub lugar: LugarEvento,
    pub asistentes: Vec<AsisConfe>,
    pub conferencistas: Vec<AsisConfe>,
    pub facultades: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programa: Option<String>,
    pub comentarios: Vec<Comentarios>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EventoUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titulo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descripcion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categoria : Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lugar: Option<LugarEvento>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asistentes: Option<Vec<AsisConfe>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conferencistas: Option<Vec<AsisConfe>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facultades: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comentarios: Option<Vec<Comentarios>>,
}