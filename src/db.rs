use rusqlite::Connection;

pub fn get_db_conn()->Connection{
  let conn = Connection:open("restaurant.db").expect("failed to open sqlite connection");
}


pub fn initialize_db(){
  println!("Initializing Database...");let conn = Connection::open("restaurant.db").expect("Failed to open sqlite connection");
  conn.execute("PRAGMA foreign_keys = ON;", []).expect("Failed to enable foreign key support");


  
  println!("Create Table table");
  create_table_table_if_not_exists(&conn).expect("Failed to create table Table Orders")

  println!("Create Menu table");
  create_menu_table_if_not_exists(&conn).expect("Failed to create table Table Orders")

  println!("Create Order table");
  create_order_table_if_not_exists(&conn).expect("Failed to create table Table Orders")

  println!("Create OrderItem table");
  create_orderitem_table_if_not_exists(&conn).expect("Failed to create table Table Orders")
}


fn create_table_table_if_not_exists(conn : Connection)->rusqlite::Result <()>{
  conn.execute("CREATE TABLE IF NOT EXISTS tables (id INTEGER PRIMARY KEY, code TEXT NOT NULL UNIQUE )", [])?;
  Ok(())

}


fn create_menu_table_if_not_exists(conn : Connection)->rusqlite::Result <()>{
  conn.execute("CREATE TABLE IF NOT EXISTS menus (id INTEGER PRIMARY KEY, name TEXT NOT NULL )", [])?;
  Ok(())

}

fn create_order_table_if_not_exists(conn : Connection)->rusqlite::Result <()>{

  conn.execute("CREATE TABLE IF NOT EXISTS orders (id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL, FOREIGN KEY (table_id) REFERENCES tables(id),UNIQUE(table_id) ", [])?;
  Ok(())
}

fn create_orderitem_table_if_not_exists(conn : Connection)->rusqlite::Result <()>{

  conn.execute("CREATE TABLE IF NOT EXISTS order_items (id INTEGER PRIMARY KEY, order_id INTEGER NOT NULL,menu_id INTEGER NOT NULL,cooking_time NOT NULL,quantity INTEGER NOT NULL default 1, FOREIGN KEY (order_id) REFERENCES orders(id),  FOREIGN KEY (menu_id) REFERENCES menus(id) ", [])?;
  Ok(())
}
