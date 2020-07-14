#![allow(dead_code)]
mod schema;
use diesel::{self, prelude::*, result::QueryResult};
use schema::posts;
///placeholder
struct Time;
pub enum BlogPostUpdate {
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
        use schema::posts::dsl::{posts as all_posts, published};
        all_posts
            .filter(published.eq(1))
            .find(id)
            .first::<BlogPost>(conn)
    }
    pub fn update(update: BlogPostUpdate) -> Result<(), &'static str> {
        match update {
            BlogPostUpdate::Content(_c) => {
                /* update post body */
                Ok(())
            }
            BlogPostUpdate::Title(_t) => {
                /* update post title */
                Ok(())
            }
        }
    }
    pub fn commit(&self, conn: &SqliteConnection) -> Result<usize, String> {
        use schema::posts::dsl::posts;
        match diesel::insert_into(posts).values(self).execute(conn) {
            Ok(id) => Ok(id),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
