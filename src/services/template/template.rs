use lazy_static::lazy_static;
use serde_json::Value;
use tera::Test;

lazy_static! {
    pub static ref TEMPLATE: tera::Tera = {
        let mut t = tera::Tera::new("templates/**/*").unwrap();
        t.autoescape_on(vec![".sql"]);
        t.register_tester("none", TemplateIsNoneTest);
        t
    };
}
pub struct TemplateIsNoneTest;

impl Test for TemplateIsNoneTest {
    fn test(&self, value: Option<&Value>, _: &[Value]) -> tera::Result<bool> {
        Ok(match value {
            Some(v) => v.is_null(),
            None => true,
        })
    }
}
