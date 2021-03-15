
mod modmysql;
mod conf;
mod http;
fn main() {

    let mysqlline=match conf::conf::myqlconfig() {
    Ok(v) => v,
    Err(_e) =>_e.to_string()
};
 //println!("mysqlLink:{}",mysqlline);
 //  let b=modmysql::modmysql::testquery(mysqlline);
  // println!("{}",b);
    http::http::testmain();

}
