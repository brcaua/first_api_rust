use actix_web::*;

pub async fn info() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "
            <html>
                <head>
                    <title>Actix Web API</title>
                </head>
                <body>
                    <h1>Actix Web API</h1>
                    <p>Actix Web API is a simple API built with Actix Web.</p>
                    
                    <h2>Endpoints</h2>
                    <ul>
                        <li><a href='/ping'>/ping</a></li>
                        <li><a href='/info'>/info</a></li>
                    </ul>

                    <footer>
                        <p>Actix Web API by <a href='https://github.com/brcaua'>Breno Pereira</a></p>
                    </footer>
                </body>
            </html>
            "
        ))
}
