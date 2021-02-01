extern crate flight_contract;
extern crate tokio_postgres;
extern crate uuid;

use flight_contract::Flights;
use tokio_postgres::{NoTls};
use tokio;

pub async fn connect() -> Option<tokio_postgres::Client> {
  let (client,conn) =
        tokio_postgres::connect("host=127.0.0.1 user=postgres password=postgres dbname=postgres port=5432", NoTls).
        await.unwrap();
  tokio::spawn(async move {
      if let Err(e) = conn.await {
          eprintln!("connection error: {}", e);
      }
  });
  return Some(client);
}

pub async fn get_flight_by_id(id:&String) -> Option<Flights> {
  let client = connect().await.unwrap();
  let rows = &client.query("SELECT id::text,corpno,'flightno' FROM flight where id::text=$1", &[&id]).await.unwrap();
  let row = rows.get(0).unwrap();
    let flight = Flights { 
        id:   row.get(0),
        flightno: row.get(2),
        corpno:  row.get(1),
    };
  return Some(flight);
}

pub async fn delete_flight_by_id(id:&String) -> Option<bool> {
  let client = connect().await.unwrap();
  let _rows = &client.query("DELETE FROM flight where id::text=$1", &[&id]).await.unwrap();
  return Some(true);
}

pub async fn insert_flight(corpno:&String,flightno:&String) -> Option<Flights> {
  let client = connect().await.unwrap();
  let _row = client.query("INSERT INTO flight VALUES(uuid_in(md5(random()::text || clock_timestamp()::text)::cstring),$1,$2)",&[&flightno,&corpno]).await.unwrap();
  let flight = Flights {
    id: String::from("0"),
    flightno: String::from(flightno),
    corpno: String::from(corpno),
  };
  return Some(flight);
}

pub async fn list_flight() -> Option<Vec<Flights>> {
  let client = connect().await.unwrap();
  let mut vec_flight = Vec::new();  
  let rows = &client.query("SELECT id::text,corpno,flightno FROM flight", &[]).await.unwrap();
  for row in rows {
    let flight = Flights { 
        id:   row.get(0),
        flightno: row.get(1),
        corpno:  row.get(2),
    };
    vec_flight.push(flight);
  }
  return Some(vec_flight);
}

pub async fn mocked_list_flight() -> Option<Vec<Flights>> {
  let mut vec_flight = Vec::new();  
  vec_flight.push(Flights {
    id: String::from("1234"),
    flightno: String::from("123456"),
    corpno: String::from("CA")
  });
  vec_flight.push(Flights {
    id: String::from("1234"),
    flightno: String::from("123456"),
    corpno: String::from("CA")
  });
  return Some(vec_flight);
}