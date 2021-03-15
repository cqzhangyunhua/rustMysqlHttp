use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Payment {
   pub customer_id: i32,
   pub  amount: i32,
   pub  account_name: Option<String>,
}

pub  fn connmysql(url:String)->mysql::Pool{
    let url = url;
    let pool = Pool::new(url).unwrap();
    return pool;
    
}

pub  fn testquery(url:String)->String{

    let url = url;

let pool = Pool::new(url).unwrap();

let mut conn = pool.get_conn().unwrap();

// // Let's create a table for payments.
// conn.query_drop(
//     r"CREATE  TABLE payment (
//         customer_id int not null,
//         amount int not null,
//         account_name text
//     )").unwrap();

// let payments = vec![
//     Payment { customer_id: 1, amount: 2, account_name: None },
//     Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
//     Payment { customer_id: 5, amount: 6, account_name: None },
//     Payment { customer_id: 7, amount: 8, account_name: None },
//     Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
// ];

// // Now let's insert payments to the database
// conn.exec_batch(
//     r"INSERT INTO payment (customer_id, amount, account_name)
//       VALUES (:customer_id, :amount, :account_name)",
//     payments.iter().map(|p| params! {
//         "customer_id" => p.customer_id,
//         "amount" => p.amount,
//         "account_name" => &p.account_name,
//     })
// ).unwrap();

// Let's select payments from database. Type inference should do the trick here.
let selected_payments = conn
    .query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| {
            Payment { customer_id, amount, account_name }
        },
    ).unwrap();

// Let's make sure, that `payments` equals to `selected_payments`.
// Mysql gives no guaranties on order of returned rows
// without `ORDER BY`, so assume we are lucky.
//assert_eq!(payments, selected_payments);

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
return a;
//Ok(a.tosting());
//println!("{}",a);

}