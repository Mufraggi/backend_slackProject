
#[macro_use]
extern crate diesel;
extern crate dotenv;


impl City {
    fn create_city(conn: &PgConnection, city: NewCity) -> City {
        use schema::city;
        diesel::insert_to(city::table)
            .values(&city)
            .get_result(conn)
            .expect("Error saving new city")
    }
}