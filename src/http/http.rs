use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::conf;
use crate::modmysql;
use mysql::*;
use mysql::prelude::*;
use crate::modmysql::modmysql::Payment as Payment;

#[get("/")]
async fn hello(pool:web::Data<mysql::Pool>) -> impl Responder {
    
    let mut conn = pool.get_conn().unwrap();
    let selected_payments = conn
    .query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| {
            Payment { customer_id, amount, account_name }
        },
    ).unwrap();
    let mut data_arr = json::JsonValue::new_array();

for i in &selected_payments {
    
  //  println!("{:}", i.customer_id);
    let mut data = json::JsonValue::new_object();
    data["customer_id"] = i.customer_id.into();
    let b=match &i.account_name {
        None =>"None",
        Some(v) =>v

    };
    data["account_name"] = b.into();
    data_arr.push(data);
}
let a=data_arr.dump();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(a)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
pub async fn testmain() -> std::io::Result<()>
{
    let mysqlline=match conf::conf::myqlconfig() {
        Ok(v) => v,
        Err(_e) =>_e.to_string()
    };
    // println!("mysqlLink:{}",mysqlline);
       let mysqlconn=modmysql::modmysql::connmysql(mysqlline);
       

 //   #[actix_web::main]
   // async fn main() -> std::io::Result<()> {
        HttpServer::new(move|| {           
            App::new()
                .data(mysqlconn.clone())
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
        })
        .workers(1)
        .bind("127.0.0.1:9090")?
        .run()
        .await
   // }
}
