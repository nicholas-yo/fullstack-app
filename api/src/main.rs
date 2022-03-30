#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dotenv_codegen;

use std::path::{Path, PathBuf};

use dotenv::dotenv;
use migration::MigratorTrait;
use rocket::fairing::{self, AdHoc, Fairing, Info, Kind};
use rocket::form::{Form, FromForm};
use rocket::fs::NamedFile;
use rocket::http::{Header};
use rocket::serde::{Deserialize, Serialize};
use rocket::{build, catchers, fs::FileServer, launch, routes, Build, Request, Response, Rocket};
use sea_orm::{entity::*, query::*};
use sea_orm_rocket::{Connection, Database};
use serde_json::json;

pub use entity::user;
pub use entity::user::Entity as UserEntity;

mod pool;
use pool::Db;

#[derive(Serialize, Deserialize, Debug, FromForm)]
struct User<'__f> {
    name: &'__f str,
    email: &'__f str,
    password: &'__f str,
    confirm_password: &'__f str,
}

#[derive(Serialize, Debug)]
struct RegisterResponse<'a> {
    msg: &'a str,
}

#[derive(Serialize, Debug)]
struct NotFound {
    status_code: i32,
    msg: String,
}

pub struct CSP;

#[rocket::async_trait]
impl Fairing for CSP {
    fn info(&self) -> Info {
        Info {
            name: "Add CSP header to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Content-Security-Policy", "default-src 'self'"));
    }
}

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    json!(NotFound {
        status_code: 404,
        msg: format!("Sorry, '{}' is not a valid path.", req.uri()),
    })
}

#[get("/<path..>")]
async fn scripts(path: PathBuf) -> Option<NamedFile> {
    dotenv().ok();

    let mut path = Path::new(dotenv!("SCRIPTS_PATH")).join(path);

    if path.is_dir() {
        path.push("index.js");
    }

    NamedFile::open(path).await.ok()
}

#[post("/register", data = "<user_form>")]
async fn register(conn: Connection<'_, Db>, user_form: Form<User<'_>>) -> JsonValue {
    let form = user_form.into_inner();

    if Set(form.password.to_owned()) != Set(form.confirm_password.to_owned()) {
        json!(RegisterResponse {
            msg: "Passwords do not match",
        })
    } else {
        let db = conn.into_inner();

        user::ActiveModel {
            email: Set(form.email.to_owned()),
            name: Set(form.name.to_owned()),
            password: Set(form.password.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
        .expect("couldn't insert user");

        json!(RegisterResponse {
            msg: "User successfully created",
        })
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;

    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .attach(CSP)
        .register("/", catchers![not_found])
        .mount("/", FileServer::from(dotenv!("STATICS_PATH")))
        .mount("/dist", routes![scripts])
        .mount("/api", routes![register])
}
