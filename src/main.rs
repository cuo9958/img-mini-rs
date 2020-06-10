extern crate curl;
extern crate image;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};

use actix_multipart::Multipart;

use std::fs::File;
use std::io::{Read, Write};

use image::ImageFormat;

use curl::easy::Easy;

/**
 * 图片处理方法
 */
fn imageFN(fromBuf: &[u8]) -> Vec<u8> {
    let img = image::load_from_memory_with_format(fromBuf, ImageFormat::Jpeg).unwrap();
    let mut buffer = Vec::new();
    img.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();
    buffer
}

//接受form提交的图片，压缩之后返回
async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    let mut filepath = String::from("");
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        println!("content_type:{}", content_type);
        let formName = content_type.get_name().unwrap();
        //TODO：名称判断和类型判断
        println!("name:{:?}", formName);
        if formName == "file" {
            let filename = content_type.get_filename().unwrap_or("");
            println!("filename:{}", filename);
            filepath = format!("./tmp/{}", filename);
            let filepath2 = filepath.clone();
            println!("filepath:{}", filepath2);
            let mut f = web::block(|| File::create(filepath2)).await.unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }
        }
    }
    let mut file2 = File::open(&filepath).expect("打开失败");
    let mut buffer = Vec::new();
    file2.read_to_end(&mut buffer).expect("文件读取失败");
    let buf2 = imageFN(&buffer);
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

//https://www.rectcircle.cn/posts/rust-actix/#3-urlencoded-body
//下载文件
fn index2() -> HttpResponse {
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
    let buf2 = imageFN(&buffer);
    HttpResponse::Ok().body(buf2)
}
async fn index3(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    println!("Chunk: {:?}", bytes);
    Ok(HttpResponse::Ok().body(bytes))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    let ip = "0.0.0.0:3000";

    println!("启动服务:{}", ip);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
            .route("/test", web::get().to(index2))
            .route("/test2", web::get().to(index3))
    })
    .bind(ip)?
    .run()
    .await
}
