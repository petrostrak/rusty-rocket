use crate::models::NewRustacean;
use crate::{models::Rustacean, schema::rustaceans};
use diesel::prelude::*;
use diesel::{QueryDsl, SqliteConnection};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn get_by_id(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn get_all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load::<Rustacean>(c)
    }

    pub fn create(c: &mut SqliteConnection, data: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(data)
            .execute(c)?;

        let last_id = Self::last_inserted_id(c)?;
        Self::get_by_id(c, last_id)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(c)
    }

    pub fn update(c: &mut SqliteConnection, id: i32, data: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(data.name.to_owned()),
                rustaceans::email.eq(data.email.to_owned()),
            ))
            .execute(c)?;
        Self::get_by_id(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
}
