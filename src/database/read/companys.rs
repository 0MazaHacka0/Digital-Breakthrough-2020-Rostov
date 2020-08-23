use crate::database::DB_POOL;
use crate::models::database::Company;

use diesel::prelude::*;

pub fn check_account(checking_phone: String) -> bool {
    use crate::schema::companys::dsl::*;

    let connection = &*DB_POOL.get().unwrap();

    let result = companys
        .filter(phone.eq(checking_phone))
        .load::<Company>(connection)
        .expect("Error loading companys");

    return result.len() == 0;
}
