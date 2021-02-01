extern crate flight_contract;
extern crate flight_dao;

use flight_contract::Flights;

pub async fn get_flight_by_id(id:&String) -> Option<Flights> {
  if id.is_empty() {
    return None
  }
  return flight_dao::get_flight_by_id(id).await;
}

pub async fn delete_flight_by_id(id:&String) -> Option<bool> {
  if id.is_empty() {
    return None
  }
  return flight_dao::delete_flight_by_id(id).await;
}

pub async fn list_flight() -> Option<Vec<Flights>> {
  return flight_dao::list_flight().await;
}

pub async fn insert_flight(corpno:&String,flightno:&String) -> Option<Flights> {
  if corpno.is_empty() || flightno.is_empty() {
    return None
  }
  return flight_dao::insert_flight(&corpno,&flightno).await;
}