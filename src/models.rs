use std::error::Error;

use chrono::NaiveDateTime;
use rocket::form::FromForm;
use sqlx::PgPool;

#[derive(Debug)]
pub struct PostDescription {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub title: String,
    pub published: bool,
}

#[derive(Debug)]
pub struct BlogPost {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub views: i32,
}

#[derive(Debug, FromForm)]
pub struct BlogForm {
    pub title: String,
    pub body: String,
}

pub async fn get_info(entry: &str, pool: &PgPool) -> Result<String, Box<dyn Error>> {
    sqlx::query!("SELECT entry FROM info WHERE name = $1", entry)
        .fetch_one(pool)
        .await
        .map(|r| r.entry)
        .map_err(|e| e.into())
}

pub async fn list_info_kinds(pool: &PgPool) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(sqlx::query!("SELECT name FROM info")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.name)
        .collect())
}

pub async fn set_info(entry: &str, name: &str, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        "INSERT INTO info (entry, name) VALUES ($1, $2)
        ON CONFLICT (name) DO
        UPDATE SET entry = EXCLUDED.entry",
        ammonia::clean(&entry),
        name
    )
    .execute(pool)
    .await?;
    Ok(())
}

impl BlogPost {
    pub fn from_form(form: BlogForm, publish: bool) -> BlogPost {
        BlogPost::new(form.title, form.body, publish)
    }

    pub fn new(title: String, body: String, publish: bool) -> BlogPost {
        BlogPost {
            id: -1,
            created_at: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            title,
            body: ammonia::clean(&body),
            published: publish,
            views: 0,
        }
    }

    pub async fn get(
        id: i32,
        published: bool,
        pool: &PgPool,
    ) -> Result<BlogPost, Box<dyn std::error::Error>> {
        Ok(if published {
            sqlx::query_as!(
                BlogPost,
                r#"SELECT * FROM posts WHERE id = $1 AND published = true"#,
                id
            )
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_as!(BlogPost, r#"SELECT * FROM posts WHERE id = $1"#, id)
                .fetch_one(pool)
                .await?
        })
    }

    pub async fn all(pool: &PgPool) -> Result<Vec<PostDescription>, Box<dyn std::error::Error>> {
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
        pool: &PgPool,
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

    pub fn _update(_update: BlogPost, _pool: &PgPool) -> Result<(), &'static str> {
        todo!()
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
DELETE FROM posts
WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn publish(id: i32, pool: &PgPool) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
UPDATE posts
SET published = NOT published
WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn commit(&self, pool: &PgPool) -> Result<i32, Box<dyn Error>> {
        Ok(sqlx::query!(
            r#"
INSERT INTO posts(title, body, published)
VALUES ( $1, $2, $3)
RETURNING id
            "#,
            self.title,
            self.body,
            self.published
        )
        .fetch_one(pool)
        .await?
        .id)
    }
}
