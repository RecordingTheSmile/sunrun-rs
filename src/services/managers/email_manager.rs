use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;

use crate::errors::BusinessResult;
use crate::services::template::template::TEMPLATE;

pub struct EmailManager;

impl EmailManager {
    async fn send_email(address: String, subject: String, content: String, nickname: String) -> BusinessResult<()> {
        let smtp_host = std::env::var("SMTP_HOST")?;
        let smtp_port: u16 = std::env::var("SMTP_PORT")?.parse().unwrap_or(465);
        let smtp_username = std::env::var("SMTP_USERNAME")?;
        let smtp_password = std::env::var("SMTP_PASSWORD")?;
        let smtp_address = std::env::var("SMTP_ADDRESS")?;
        let smtp_nickname = std::env::var("SMTP_NICKNAME")?;
        let smtp_starttls = std::env::var("SMTP_STARTTLS")?.parse().unwrap_or(false);

        let message = Message::builder()
            .from(format!("{smtp_nickname} <{smtp_address}>").parse()?)
            .to(format!("{nickname} <{address}>").parse()?)
            .subject(subject)
            .multipart(MultiPart::alternative()
                .singlepart(SinglePart::plain(content.to_owned()))
                .singlepart(SinglePart::html(content)))?;

        let mailer: AsyncSmtpTransport<Tokio1Executor> = if smtp_starttls {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host)?.port(smtp_port).credentials(Credentials::new(smtp_username, smtp_password)).build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?.port(smtp_port).credentials(Credentials::new(smtp_username, smtp_password)).build()
        };

        mailer.send(message).await?;

        Ok(())
    }

    pub fn send_verify_email(to: String, code: String) {
        let mut ctx = tera::Context::new();
        ctx.insert("code", &code);
        let template = TEMPLATE.render("email/email_verify.html", &ctx)
            .unwrap_or(format!("您的邮箱验证码为：{code}，请尽快验证！"));

        tokio::spawn(async move {
            match Self::send_email(to, "您的邮箱验证码 - 阳光小助手".into(), template, "待验证用户".into()).await {
                Ok(_) => (),
                Err(e) => {
                    log::error!("发送邮件错误：{:#?}",e);
                }
            };
        });
    }

    pub fn send_result_email(to: String, is_success: bool, reason: Option<String>) {
        let mut ctx = tera::Context::new();
        ctx.insert("is_success", &is_success);
        ctx.insert("reason", &reason);

        let template = TEMPLATE.render("email/run_result.html", &ctx)
            .unwrap_or(format!("您的跑步结果为：{}，失败原因：{}", is_success, reason.unwrap_or("跑步成功".into())));

        tokio::spawn(async move {
            match Self::send_email(to, "您的跑步结果 - 阳光小助手".into(), template, "用户".into()).await {
                Ok(_) => (),
                Err(e) => {
                    log::error!("发送邮件错误：{:#?}",e);
                }
            };
        });
    }
}