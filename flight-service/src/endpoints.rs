use actix_web::{get,put,delete,Responder,HttpResponse,web};
#[path="service.rs"] mod service;

#[get("/")]
async fn index() -> impl Responder {
    format!("Endpoints: /flights ")
}

#[get("/flights")]
pub async fn list_flight() -> HttpResponse  {
    let news = service::list_flight().await;
    HttpResponse::Ok().json(news)
}

#[get("/flights/{id}")]
pub async fn get_flight_by_id(info:web::Path<String>) -> HttpResponse  {
    let id  = &info.as_str();
    let mut new_string = String::new();
    new_string.push_str(id);

    let news = service::get_flight_by_id(&new_string).await;
    HttpResponse::Ok().json(news)
}

#[delete("/flights/{id}")]
pub async fn delete_flight_by_id(info:web::Path<String>) -> HttpResponse {
    let id  = &info.as_str();
    let mut new_string = String::new();
    new_string.push_str(id);

    let flight = service::delete_flight_by_id(&new_string).await;
    HttpResponse::Ok().json(flight)
}

#[put("/flights/{corpno}/{flightno}")]
pub async fn insert_flight(info:web::Path<(String, String)>) -> impl Responder {
    let corpno  = &info.0;
    let flightno = &info.1;
    
    
    let flight = service::insert_flight(corpno,flightno).await;
    HttpResponse::Ok().json(flight)
}