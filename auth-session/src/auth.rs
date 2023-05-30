use actix_session::Session;
use actix_web::{cookie::Cookie, http::header::COOKIE, web, HttpRequest, HttpResponse, Responder};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
}

#[derive(Serialize)]
struct ValidateResponse {
    message: String,
}

// Handler for POST /login
pub async fn login(form: web::Json<LoginForm>, session: Session) -> impl Responder {
    if form.username == "user" && form.password == "password" {
        session.insert("user_id", form.username.clone()).unwrap();
        HttpResponse::Ok()
            .append_header(("session-id", ""))
            .json(LoginResponse {
                message: "login".to_string(),
            })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            message: "unauthorized".to_string(),
        })
    }
}

// Handler for GET /validate
pub async fn validate(session: Session, request: HttpRequest) -> impl Responder {
    let cookies = request.headers().get(COOKIE);
    let cookies = match cookies {
        Some(cookies) => {
            let raw_cookies = cookies.to_str().unwrap();
            let parsed_cookies: Vec<Cookie> = raw_cookies
                .split(';')
                .filter_map(|c| Cookie::parse_encoded(c).ok())
                .collect();
            format!("Parsed Cookies: {:?}", parsed_cookies)
        }
        None => String::from("No Cookies"),
    };
    debug!("{}", cookies);

    if let Some(_id) = session.get::<String>("user_id").unwrap() {
        debug!("hit");
        HttpResponse::Ok().json(ValidateResponse {
            message: "User authenticated".to_string(),
        })
    } else {
        debug!("not hit");
        HttpResponse::Unauthorized().json(ValidateResponse {
            message: "Not valid session".to_string(),
        })
    }
}
