use crate::database::DB_POOL;
use crate::models::database::UserOmitId;
use crate::models::api::NewAccount;

use sha2::{Sha256, Digest};
use hex;

use diesel::prelude::*;

pub fn save(new_user: NewAccount) -> (i32, i32) {
    use crate::schema::users::dsl::*;

    let connection = &*DB_POOL.get().unwrap();

    let mut hasher = Sha256::new();
    hasher.update(new_user.password);

    let data = UserOmitId {
        email: new_user.email,
        password_hash: hex::encode(hasher.finalize()),
        region_id: 1,
        home_id: new_user.home_id.unwrap_or(1)
    };

    let result = diesel::insert_into(users)
        .values(&data)
        .execute(connection)
        .unwrap();
    
    if result == 0 {
        return (-1, -1)
    }

    let inserted_user = users
        .select((id, home_id))
        .load::<(i32, i32)>(connection)
        .unwrap();

    *inserted_user.last().unwrap_or(&(1, 1))
}
