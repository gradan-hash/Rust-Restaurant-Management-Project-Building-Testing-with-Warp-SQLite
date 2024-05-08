
use crate::handlers::{
  create_order_handler,
  list_tables_handler,
  create_table_handler,
  list_menu_handler,
  create_menu_handler,
  list_order_handler,
  delete_item_from_order_handler,
  list_order_items_for_table_handler,
  get_order_item_for_table_handler

};

use warp::{Filter,Rejection,Reply};
use rusqlite::Connection;
use crate::db::get_db_conn;
use std::convert::Infallible;

async fn handle_rejection(err :Rejection-> Result<impl Reply,Rejection>){

  if err.if_not_found(){
    Ok(warp::reply::with_status(
      warp::reply::json(&format!("Error: {:?}", err)),
      warp::http::StatusCode::NOT_FOUND,
      
    ))

  }else if let_some(_) =err.find::<warp::filters::body::BodyDeserializeError>(){
    Ok(warp::reply::with_status(
      warp::reply::json(&format! ("Error: Failed to deserialize Body")),
      warp::http::StatusCode::BAD_REQUEST,
    ))
  } else {
    Ok(warp:reply_with_status(
      warp_reply_json(&format!("Error: {:?}", err)),
      warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
  }
}

pub fn list_all_order_route(){}

pub fn create_order_route(){}

pub fn delete_item_from_order_route(){}

pub fn  list_tables_route(){}


pub fn create_order_route(){}

pub fn create_table_route(){}
pub fn list_order_items_for_table_route(){}


pub fn get_item_from_order_route(){}


pub fn list_menus_route(){}


pub create_menu_route(){}




pub fn restaurant_routes()->impl Filter<Extract = impl Reply,Error = Rejection> + Clone{
  let routes = create_order_route()
  .or(create_table_route())
  .or(create_menu_route())
  .or(list_tables_route())
  .or(list_menus_route())
  .or(list_all_order_route())
  .or (delete_item_from_order_route())
  .or(list_order_items_for_table_route())
  .or(get_item_from_order_route());

  routes.recover(handle_rejection);


}
