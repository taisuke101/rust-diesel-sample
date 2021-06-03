use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

pub fn find_user_by_uuid(
    uuid: Uuid,
    connection: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users.filter(id.eq(uuid.to_string()))
    .first::<models::User>(connection)
    .optional()?;

    Ok(user)
}

pub fn insert_new_user(
    username: &str,
    connection: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: username.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(connection)?;

    Ok(new_user)
}

// pub fn update_user(
//     uuid: Uuid,
//     username: &str,
//     connection: &PgConnection,
// ) -> Result<models::User, diesel::result::Error> {
//     use crate::schema::users::dsl::*;

//     let update_user = diesel::update(users.filter(id.eq(uuid.to_string())))
//     .set(name.eq(username)).get_result(connection);

//     Ok(update_user)
// }