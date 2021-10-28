use std::{future::Future, pin::Pin};

use hmac::{Hmac, NewMac};
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    Build, Rocket,
};
use sha2::Sha256;

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
