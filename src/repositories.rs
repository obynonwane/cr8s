use crate::schema::rustaceans;
use diesel::QueryDsl;
use diesel_async::AsyncPgConnection;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &AsyncPgConnection, id: i32) {
        rustaceans::table.find(id);
    }
}
