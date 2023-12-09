use actix_web::*;

pub async fn products() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        // return a JSON object with api info and a list of products
        .body(
            r#"{
                "api": "rust-actix",
                "qty": 10,
                "version": "1.0.0",
                "products": [
                    {
                        "id": 1,
                        "name": "Product 1"
                    },
                    {
                        "id": 2,
                        "name": "Product 2"
                    },
                    {
                        "id": 3,
                        "name": "Product 3"
                    },
                    {
                        "id": 4,
                        "name": "Product 4"
                    },
                    {
                        "id": 5,
                        "name": "Product 5"
                    },
                    {
                        "id": 6,
                        "name": "Product 6"
                    },
                    {
                        "id": 7,
                        "name": "Product 7"
                    },
                    {
                        "id": 8,
                        "name": "Product 8"
                    },
                    {
                        "id": 9,
                        "name": "Product 9"
                    },
                    {
                        "id": 10,
                        "name": "Product 10"
                    }
                ]
            }"#,
        )
}
