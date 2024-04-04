use my_http_server::*;

pub struct MyMiddleware;

#[async_trait::async_trait]
impl HttpServerMiddleware for MyMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        _get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let mut content = String::new();

        content.push_str(format!("[{:?}]\n", ctx.request.method.as_str()).as_str());
        content.push_str("----------------\n");

        for (name, value) in ctx.request.get_headers().to_hash_map() {
            content.push_str(&format!("{}: {}\n", name, value));
        }

        content.push_str("----------------\n");
        content.push_str(
            format!(
                "Body size: {}\n",
                ctx.request.get_body().await?.as_slice().len()
            )
            .as_str(),
        );

        HttpOutput::as_text(content).into_ok_result(false)
    }
}
