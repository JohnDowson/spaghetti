#![allow(dead_code)]
use crate::DbConn;
mod schema;
use diesel::{self, prelude::*, result::QueryResult};
use schema::posts;
use schema::posts::dsl::{id as post_id, posts as all_posts, published};
///placeholder
struct Time;
enum Update {
    Content(String),
    Title(String),
}
#[table_name = "posts"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct BlogPost {
    pub id: i32,
    pub created: String,
    pub title: String,
    pub body: String,
    pub published: i32,
    pub views: i32,
}
impl BlogPost {
    pub fn get(id: i32, conn: &SqliteConnection) -> QueryResult<BlogPost> {
        all_posts
            .filter(published.eq(1))
            .find(id)
            .first::<BlogPost>(conn)
    }
    fn update(update: Update) -> Result<(), &'static str> {
        match update {
            Update::Content(_c) => {
                /* update post body */
                Ok(())
            }
            Update::Title(_t) => {
                /* update post title */
                Ok(())
            }
        }
    }
}
