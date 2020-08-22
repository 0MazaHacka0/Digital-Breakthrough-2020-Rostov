use crate::database::DB_POOL;
use crate::models::database::CompanyOmitId;
use crate::models::api::NewAccount;

use sha2::{Sha256, Digest};
use hex;

use diesel::prelude::*;

pub fn save(new_company: NewAccount) {
    use crate::schema::companys::dsl::*;

    let mut hasher = Sha256::new();
    hasher.update(new_company.password);

    let connection = &*DB_POOL.get().unwrap();

    let data = CompanyOmitId {
        email: new_company.email,
        password_hash: hex::encode(hasher.finalize()),
        region_id: 1
    };

    diesel::insert_into(companys)
        .values(&data)
        .execute(connection)
        .unwrap();
}
