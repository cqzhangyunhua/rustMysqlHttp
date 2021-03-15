//read  mysql conf
use configparser::ini::Ini;
use std::error::Error;
pub fn myqlconfig() -> Result<String, Box<dyn Error>>
{
   
    let mut config = Ini::new();

    // You can easily load a file to get a clone of the map:
    let _map = config.load("conf/mysql.ini")?;
   // println!("{:?}", map);
    // You can also safely not store the reference and access it later with get_map_ref() or get a clone with get_map()
  
    // If you want to access the value, then you can simply do:
    let val = config.get("DEFAULT", "linkmysql").unwrap();
   // println!("This is test fun!{}",val);
    Ok(val)
}