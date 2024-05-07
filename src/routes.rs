
async fn handle_rejection(){}

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
