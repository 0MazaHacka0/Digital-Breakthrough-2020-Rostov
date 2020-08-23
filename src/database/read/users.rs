use crate::database::DB_POOL;
use crate::models::database::User;

use diesel::prelude::*;

pub fn check_account(checking_phone: String) -> bool {
    use crate::schema::users::dsl::*;

    let connection = &*DB_POOL.get().unwrap();

    let result = users
        .filter(phone.eq(checking_phone))
        .load::<User>(connection)
        .expect("Error loading companys");

    return result.len() == 0;
}

pub fn get_user_id(phone_user: String) -> i32 {
    use crate::schema::users::dsl::*;

    let connection = &*DB_POOL.get().unwrap();

    users
        .select(id)
        .filter(
            phone.eq(phone_user)
        )
        .first::<i32>(connection)
        .unwrap_or(-1)
}