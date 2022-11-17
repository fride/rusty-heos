use std::collections::HashMap;
use axum::{extract::Form, response::Html, routing::get, Router, Extension};
use axum::extract::Query;
use axum::http::{header, HeaderMap};
use axum::response::Redirect;
use heos_api::{HeosDriver, HeosResult};
use heos_api::error::HeosError;
use maud::{html, Markup};
use serde::Deserialize;
use tracing::info;
use crate::http::AppError;

pub fn router(driver: HeosDriver) -> Router {
    Router::new()
        .route("/login"
               , get(show_login).post(accept_login))
        .layer(Extension(driver))
}

async fn show_login(Query(params): Query<HashMap<String, String>>) -> Markup {
    html!({
        @if let Some(error) = params.get("error") {
            div {
                ( error )
            }
        }
        form action="/login" method="post" {
            label for="un" { ("Name")}
            input type="text" name="un" id="un"{}
            label for="un" { ("Password")}
            input type="password" name="pw" id="pw"{}
            input type="submit" {}
        }
    })
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LoginForm {
    pub un: String,
    pub pw: String,
    pub error: Option<String>
}

impl LoginForm {
    pub fn render_html(&self) -> Markup{
        html!({
        @if let Some(error) = &self.error {
            div {
                ( error )
            }
        }
        form action="/login" method="post" {
            label for="un" { ("Name")}
            input type="text" name="un" id="un" value=(self.un){}
            label for="un" { ("Password")}
            input type="password" name="pw" id="pw"{}
            input type="submit" {}
        }
    })
    }
}

async fn accept_login(
    Extension(driver): Extension<HeosDriver>,
    Form(mut input): Form<LoginForm>) -> Result<Redirect,Markup> {
    match driver.login(input.un.clone(), input.pw.clone()).await {
        Ok(account_state) => {
            Ok(Redirect::temporary("/sources"))
        }
        Err(HeosError::InvalidCommand { command, eid, text }) => {
            input.error = Some(text);
            Err(input.render_html())
        }
        Err(_) => {
            input.error = Some("Something went wrong!".to_string());
            Err(input.render_html())
        }
    }
}

