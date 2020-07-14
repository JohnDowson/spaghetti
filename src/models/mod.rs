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
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct PostDescription {
    pub id: i32,
    pub created: String,
    pub title: String,
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
    pub fn new(title: &str, body: &str, publish: bool) -> BlogPost {
        BlogPost {
            id: -1,
            created: String::default(),
            title: title.to_owned(),
            body: ammonia::clean(body),
            published: match publish {
                true => 1,
                false => 0,
            },
            views: 0,
        }
    }
    pub fn get(id: i32, conn: &SqliteConnection) -> QueryResult<BlogPost> {
        use schema::posts::dsl::{posts, published};
        posts
            .filter(published.eq(1))
            .find(id)
            .first::<BlogPost>(conn)
    }
    pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<PostDescription>> {
        use schema::posts::dsl::{created, id, posts, published, title};
        posts
            .filter(published.eq(1))
            .order(id.desc())
            .select((id, created, title))
            .load::<PostDescription>(conn)
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
    pub fn commit(&self, conn: &SqliteConnection) -> Result<i32, String> {
        use schema::posts::dsl::*;
        if self.id != -1 {
            return Err(format!(
                "You are trying to insert an existing post {}",
                self.id
            ));
        }
        match diesel::insert_into(posts)
            .values((
                body.eq(self.body.to_owned()),
                title.eq(self.title.to_owned()),
                published.eq(self.published),
            ))
            .execute(conn)
        {
            Ok(_) => match posts.select(id).order(id.desc()).first(conn) {
                Ok(post_id) => Ok(post_id),
                Err(e) => Err(format!("{:?}", e)),
            },
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
