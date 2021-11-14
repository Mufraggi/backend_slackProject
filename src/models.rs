#[derive(Queryable)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub gps_point: String,
    pub mail: String,
    pub phone_number: String,
    pub validate: bool,
}

#[derive(Insertable)]
#[table_name="city"]
pub struct NewCity<'a> {
    pub name: &'a str,
    pub gps_point: &'a str,
    pub mail: &'a str,
    pub phone_number: &'a str,
    pub validate: &'a bool,
}