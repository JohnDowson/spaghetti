use std::error::Error;

use chrono::NaiveDateTime;
use rocket::form::FromForm;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct PostDescription {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub title: String,
    pub published: bool,
}

#[derive(Debug)]
pub struct BlogPost {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub views: i64,
}

#[derive(Debug, FromForm)]
pub struct BlogForm {
    pub title: String,
    pub body: String,
}

impl BlogPost {
    pub fn from_form(form: BlogForm, publish: bool) -> BlogPost {
        BlogPost::new(form.title, form.body, publish)
    }

    pub fn new(title: String, body: String, publish: bool) -> BlogPost {
        BlogPost {
            id: -1,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            title,
            body: ammonia::clean(&body),
            published: publish,
            views: 0,
        }
    }

    pub async fn get(
        id: i64,
        published: bool,
        pool: &SqlitePool,
    ) -> Result<BlogPost, Box<dyn std::error::Error>> {
        Ok(if published {
            sqlx::query_as!(
                BlogPost,
                r#"SELECT * FROM posts WHERE id = ?1 AND published = true"#,
                id
            )
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_as!(BlogPost, r#"SELECT * FROM posts WHERE id = ?1"#, id)
                .fetch_one(pool)
                .await?
        })
    }

    pub async fn all(
        pool: &SqlitePool,
    ) -> Result<Vec<PostDescription>, Box<dyn std::error::Error>> {
        Ok(sqlx::query_as!(
            PostDescription,
            r#"
    SELECT id, created_at, title, published FROM posts
    ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn all_published(
        pool: &SqlitePool,
    ) -> Result<Vec<PostDescription>, Box<dyn std::error::Error>> {
        Ok(sqlx::query_as!(
            PostDescription,
            r#"
    SELECT id, created_at, title, published FROM posts
    WHERE published = true
    ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?)
    }

    pub fn _update(_update: BlogPost, _pool: &SqlitePool) -> Result<(), &'static str> {
        todo!()
    }

    pub async fn delete(id: i64, pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
DELETE FROM posts
WHERE id = ?1
            "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn publish(id: i64, pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
UPDATE posts
SET published = NOT published
WHERE id = ?1
            "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn commit(&self, pool: &SqlitePool) -> Result<i64, Box<dyn Error>> {
        Ok(sqlx::query!(
            r#"
INSERT INTO posts(title, body, published)
VALUES ( ?1, ?2, ?3)
            "#,
            self.title,
            self.body,
            self.published
        )
        .execute(pool)
        .await?
        .last_insert_rowid())
    }
}
