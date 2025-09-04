use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result, middleware::Logger};
use actix_cors::Cors;
use mongodb::{Client, options::ClientOptions, Collection};
use mongodb::bson::{doc, Document};
use serde::{Serialize, Deserialize};
use std::env;
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserData {
    #[serde(skip_serializing_if = "Option::is_none")]
    photo: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hero: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    about_me: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    education: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skills: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projets: Option<serde_json::Value>,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            photo: None,
            hero: None,
            username: None,
            about_me: None,
            contact: None,
            services: None,
            education: None,
            skills: None,
            projets: None,
        }
    }
}

#[derive(Clone)]
struct AppState {
    db_client: Client,
    db_name: String,
}

#[get("/api/userdata")]
async fn get_userdata(data: web::Data<AppState>) -> Result<impl Responder> {
    let db = data.db_client.database(&data.db_name);
    
    let collections = vec![
        "photo", "hero", "username", "about", "contact",
        "services", "education", "skills", "projets",
    ];

    let mut user_data = UserData::default();

    for col_name in collections {
        let collection = db.collection::<Document>(col_name);
        if let Ok(Some(doc)) = collection.find_one(None, None).await {
            let value = serde_json::to_value(&doc).ok();
            match col_name {
                "photo" => user_data.photo = value,
                "hero" => user_data.hero = value,
                "username" => user_data.username = value,
                "about" => user_data.about_me = value,
                "contact" => user_data.contact = value,
                "services" => user_data.services = value,
                "education" => user_data.education = value,
                "skills" => user_data.skills = value,
                "projets" => user_data.projets = value,
                _ => {}
            }
        }
    }

    Ok(HttpResponse::Ok().json(user_data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env variables
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load env variables
    let mongodb_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb+srv://smailyazidi:S20S09S2003S@cluster.jhvf7mf.mongodb.net/smail_yazidi?retryWrites=true&w=majority&appName=Cluster".to_string());
    let db_name = env::var("MONGODB_DB")
        .unwrap_or_else(|_| "smail_yazidi".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // MongoDB client
    let client_options = ClientOptions::parse(&mongodb_uri).await
        .expect("Failed to parse MongoDB URI");
    let client = Client::with_options(client_options)
        .expect("Failed to create MongoDB client");

    // Test connection
    client.database(&db_name).run_command(doc! {"ping": 1}, None).await
        .expect("Failed to connect to MongoDB");

    println!("Connected to MongoDB, starting server on 0.0.0.0:{}", port);

    let app_state = AppState { db_client: client, db_name };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(get_userdata)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
