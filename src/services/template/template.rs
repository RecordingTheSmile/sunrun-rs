use actix_web::HttpResponse;
use lazy_static::lazy_static;
use serde_json::Value;
use tera::{Context, Test};

lazy_static! {
    pub static ref TEMPLATE:tera::Tera = {
        let mut t = tera::Tera::new("templates/**/*").unwrap();
        t.autoescape_on(vec![".sql"]);
        t.register_tester("none",TemplateIsNoneTest);
        t
    };
}

pub trait ActixHttpResponseExt {
    fn html(&mut self, template_name: &str, context: &tera::Context) -> HttpResponse;
}

impl ActixHttpResponseExt for actix_web::HttpResponseBuilder {
    fn html(&mut self, template_name: &str, context: &Context) -> HttpResponse {
        let result = TEMPLATE.render(template_name, context).unwrap_or_else(|e| {
            log::error!("TemplateRenderErr: {:?}",e);
            r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>抱歉，出错了</title>
        </head>
        <body>
            <h1 style="text-align:center">抱歉，出错了</h1>
            <h5 style="text-align:center">原因：模板渲染失败</h5>
            <hr>
            <p style="text-align:center">Sunrun</p>
        </body>
        </html>
        "#.into()
        });

        self.content_type(actix_web::http::header::ContentType::html()).body(result)
    }
}

pub struct TemplateIsNoneTest;

impl Test for TemplateIsNoneTest {
    fn test(&self, value: Option<&Value>, _: &[Value]) -> tera::Result<bool> {
        Ok(match value {
            Some(v) => v.is_null(),
            None => true
        })
    }
}