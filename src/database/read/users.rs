use crate::database::DB_POOL;
use crate::models::database::User;

use diesel::prelude::*;

pub fn check_account(checking_email: String) -> bool {
    use crate::schema::users::dsl::*;

    let connection = &*DB_POOL.get().unwrap();

    let result = users
        .filter(email.eq(checking_email))
        .load::<User>(connection)
        .expect("Error loading companys");

    return result.len() == 0;
}
