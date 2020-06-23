extern crate curl;
extern crate image;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};

use actix_multipart::Multipart;

use curl::easy::Easy;
use qstring::QString;

mod image_pro;

//接受form提交的图片，压缩之后返回
async fn form_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut buffer = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        println!("content_type:{}", content_type);
        let form_name = content_type.get_name().unwrap();
        //TODO：名称判断和类型判断
        println!("name:{:?}", form_name);
        if form_name == "file" {
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                buffer.extend_from_slice(&data);
            }
        }
    }
    let buf2 = image_pro::image_fn(&buffer);
    Ok(HttpResponse::Ok().body(buf2))
}

//https://www.rectcircle.cn/posts/rust-actix/#3-urlencoded-body
//下载文件
fn curl_image() -> HttpResponse {
    let url="https://img5.daling.com/zin/public/specialTopic/2020/05/25/13/59/58/5254006B762DN6QUS000007436152.jpg";

    let mut buffer = Vec::new();
    {
        let mut easy = Easy::new();
        easy.url(url).unwrap();
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                let mut b = Vec::new();
                b.extend_from_slice(data);
                // buffer.extend_from_slice(data);
                buffer.append(&mut b);
                println!("长度:{:?}", buffer.len());
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    println!("文件下载完成:{}", buffer.len());
    let buf2 = image_pro::image_fn(&buffer);
    HttpResponse::Ok().body(buf2)
}

//直接上传文件
async fn bin_image(mut body: web::Payload, req: HttpRequest) -> Result<HttpResponse, Error> {
    let qs = QString::from(req.query_string());
    print!("请求参数:{:?}", qs);
    print!("请求a:{:?}", qs.get("a"));

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }
    println!("Chunk: {:?}", bytes.len());
    let buf2 = image_pro::image_fn(&bytes);
    Ok(HttpResponse::Ok().body(buf2))
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="text" name="test"  />
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    let ip = "0.0.0.0:3000";

    println!("启动服务:{}", ip);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .route("/form", web::post().to(form_image))
            .route("/curl", web::get().to(curl_image))
            .route("/bin", web::get().to(bin_image))
    })
    .bind(ip)?
    .run()
    .await
}
