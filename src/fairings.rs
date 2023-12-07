use std::{future::Future, pin::Pin};

use hmac::{Hmac, Mac};
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    request::FromRequest,
    Build, Request, Response, Rocket, State,
};
use sha2::Sha256;
use sqlx::PgPool;

pub(crate) struct DbManager;

impl Fairing for DbManager {
    fn info(&self) -> Info {
        Info {
            name: "DbManager",
            kind: Kind::Singleton | Kind::Ignite,
        }
    }

    fn on_ignite<'life0, 'async_trait>(
        &'life0 self,
        rocket: Rocket<Build>,
    ) -> Pin<Box<dyn Future<Output = fairing::Result> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let db_uri: String = rocket
                .figment()
                .extract_inner("db")
                .expect("Please configure ROCKET_DB");
            log::info!("Using db: {}", db_uri);
            let pool = sqlx::PgPool::connect(&db_uri)
                .await
                .expect("Couldn't create DB pool");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("Couldn't run migrations");
            Ok(rocket.manage(pool))
        })
    }
}

pub(crate) struct SecretManager;

impl Fairing for SecretManager {
    fn info(&self) -> Info {
        Info {
            name: "SecretManager",
            kind: Kind::Singleton | Kind::Ignite,
        }
    }

    fn on_ignite<'life0, 'async_trait>(
        &'life0 self,
        rocket: Rocket<Build>,
    ) -> Pin<Box<dyn Future<Output = fairing::Result> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let figment = rocket.figment();
            let secret_key: String = figment
                .extract_inner("secret_key")
                .expect("Couldn't extract secret_key");
            let secret_key: Hmac<Sha256> =
                Hmac::new_from_slice(secret_key.as_bytes()).expect("Couldn't generate HMAC");
            let admin_password: String = figment
                .extract_inner("admin_password")
                .expect("Couldn't extract admin password");
            let admin_password =
                bcrypt::hash(admin_password, 12).expect("Could not hash admin password");
            let secrets = Secrets {
                secret_key,
                admin_password,
            };
            Ok(rocket.manage(secrets))
        })
    }
}

pub struct Secrets {
    secret_key: Hmac<Sha256>,
    admin_password: String,
}
impl Secrets {
    pub fn secret_key(&self) -> &Hmac<Sha256> {
        &self.secret_key
    }
    pub fn admin_password(&self) -> &str {
        &self.admin_password
    }
}

pub struct HitCount;

impl Fairing for HitCount {
    fn info(&self) -> Info {
        Info {
            name: "HitCount",
            kind: Kind::Response,
        }
    }

    #[allow(
        clippy::let_unit_value,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn on_response<'r, 'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        req: &'r Request<'life1>,
        res: &'life2 mut Response<'r>,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'r: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let ip = req.client_ip().map(sqlx::types::ipnetwork::IpNetwork::from);
            let page = req.uri().path();
            if let Some("static") = page.segments().next() {
                return;
            }
            let status = res.status().code as i32;
            let db = <&State<PgPool>>::from_request(req)
                .await
                .succeeded()
                .unwrap();
            _ = sqlx::query!(
                "INSERT INTO page_hits VALUES ($1, $2, $3)",
                ip,
                page.to_string(),
                status
            )
            .execute(&**db)
            .await;
        })
    }
}
