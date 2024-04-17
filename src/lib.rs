use url::Url;
use worker::*;

use console_error_panic_hook::set_once as set_panic_hook;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    console_log!("{} - [{}]", Date::now().to_string(), req.path());
    let image_path = req.path()[1..].to_string();

    set_panic_hook();
    
    match handle_render(image_path).await {
        Err(err) => {
            println!("error: {:?}", err);
            Response::error(format!("an unexpected error occurred: {}", err), 500)
        }
        Ok(res) => Ok(res),
    }
}

async fn handle_render(svg_url: String) -> Result<Response> {
    let opt = usvg::Options::default();
    console_log!("svgUrl: {}", svg_url);
    let url = Url::parse(&svg_url)
        .map_err(|err| format!("failed to parse URL: {}", err))?;

    let mut res = Fetch::Url(url)
        .send()
        .await
        .map_err(|err| format!("failed to request remote image: {}", err))?;
    if res.status_code() != 200 {
        let body = res.text().await?;
        return Response::error(
            format!("upstream image returned: {}: {}", res.status_code(), body),
            500,
        );
    }
    let svg_data = res.bytes().await?;

    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref())
        .map_err(|err| format!("failed to decode SVG: {}", err))?;

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or("failed to create new pixmap")?;
    resvg::render(
        &rtree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .ok_or("failed to render PNG")?;

    let out = pixmap
        .encode_png()
        .map_err(|err| format!("failed to encode PNG: {}", err))?;

    let mut headers = Headers::new();
    headers.set("content-type", "image/png").unwrap();
    Ok(Response::from_bytes(out).unwrap().with_headers(headers))
}
