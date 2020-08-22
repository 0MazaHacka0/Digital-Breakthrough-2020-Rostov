use crate::models::database::Static;
use crate::database::DB_POOL;
use super::pages::get_page;

use diesel::prelude::*;

pub fn get_statics_for_page(route: &str, page: &str) -> Vec<Static> {
    use crate::schema::statics::dsl::*;

    let connection = &*DB_POOL.get().unwrap();

    let page = get_page(route, page);

    statics.filter(
        page_id.eq(page.id)
        .and(status.eq(1))
    )
    .load::<Static>(connection)
    .expect("Error loading static files")
}
