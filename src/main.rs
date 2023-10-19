
use std::error::Error;

use actix_web::{get, App, HttpServer,HttpResponse,http::header,HttpRequest};
use reqwest::{self, StatusCode};
use serde_json::Value;

static CLIENT_ID:&str= "f5ae4d4c94fe1681e191";
static CLIENT_PASSWD:&str="6ebf7ef23ab15986c06b41dfbfb35558afb33479";


#[tokio::main]
async fn main() {
    println!("==rs-github-oauth==");
    let _ =server().await;
}

async fn server()->std::result::Result<(),Box<dyn std::error::Error>> {
    HttpServer::new(|| {
        App::new()
        .service(mainpage)
        .service(redirect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}

#[get("/")]
async fn mainpage(_: HttpRequest) -> actix_web::Result<HttpResponse> {
    println!("Enter Main Page");
    let page =format!("
    <!doctype html>
    <html>
    <head>
        <title>rs-github-oauth</title>
    </head>
    <body>
    </body>
        <a href=\"https://github.com/login/oauth/authorize?client_id={CLIENT_ID}&redirect_uri=http://localhost:8080/oauth/redirect\">go github</a>
    </html>");

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(header::ContentType::html())
        .body(page))
}


#[get("/oauth/redirect")]
async fn redirect(req: HttpRequest) ->  actix_web::Result<HttpResponse> {
    println!("Enter redirect Page");
    let que:String=req.query_string().to_string();
    let code:Vec<String>=que.split('=').map(|x|x.to_string()).collect();

    let json=get_user(code[1].clone()).await?;
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type(header::ContentType::json())
            .body(json)
    )
}



async fn get_user(code:String)->std::result::Result<String,Box<dyn Error>>{
    println!("Enter get user");
    let  client = reqwest::Client::new();

    //get token
    let ret=client.post(
        format!("https://github.com/login/oauth/access_token?client_id={CLIENT_ID}&client_secret={CLIENT_PASSWD}&code={code}"))
    .header("accept", "application/json")
    .send()
    .await?;

    let cont=ret.text().await?;
    let js:Value =serde_json::from_str(cont.as_str())?;


    // print

    
    let token=js["access_token"].as_str().unwrap_or(format!("Cant Get Token Please Check CLIENT_ID:{CLIENT_ID} client_secret:{CLIENT_PASSWD} and code={code}").as_str()).to_string();
    let _scope = js["scope"].as_str().unwrap_or("").to_string();
    let _token_type = js["token_type"].as_str().unwrap_or("").to_string();
    
    println!("{}",js);
    if token==format!("Cant Get Token Please Check CLIENT_ID:{CLIENT_ID} client_secret:{CLIENT_PASSWD} and code={code}"){
        return Ok(token);
    }
    // get user
    let user=client.get("https://api.github.com/user")
        .header("Accept","application/vnd.github+json")
        .header("Authorization",format!("Bearer {token}"))
        // must to decl user-agent
        .header("USER-AGENT", "rs-demo")
        .send()
        .await?;
        
    let cont=user.text().await?;
    let js:Value =serde_json::from_str(cont.as_str())?;
    
    //print
    println!("{}",js);

    // return
    Ok(format!("{}",js))
}