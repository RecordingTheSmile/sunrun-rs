use crate::errors::{BusinessError, BusinessResult};
use crate::services::database::database::Database;
use crate::services::sunrun::sunrun::Sunrun;
use crate::services::sunrun::wx_login::{WxLogin, WxQrcodeInfo, WxQrcodeStatus};
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter,
    QuerySelect,
};

pub struct QrManager;

impl QrManager {
    pub async fn create_qr(id: &str) -> BusinessResult<WxQrcodeInfo> {
        let login_info = WxLogin::get_qr_login_info().await?;

        let time_now = chrono::Local::now().timestamp();

        if entity::qr_scan::Entity::find()
            .filter(entity::qr_scan::Column::QrId.eq(id))
            .count(Database::get_conn())
            .await?
            == 1
        {
            entity::qr_scan::Entity::update_many()
                .filter(entity::qr_scan::Column::QrId.eq(id))
                .col_expr(
                    entity::qr_scan::Column::QrUuid,
                    Expr::value(login_info.uuid.to_owned()),
                )
                .col_expr(
                    entity::qr_scan::Column::Imeicode,
                    Expr::value(Option::<String>::None),
                )
                .col_expr(entity::qr_scan::Column::CreateAt, Expr::value(time_now))
                .exec(Database::get_conn())
                .await?;
        } else {
            entity::qr_scan::ActiveModel {
                qr_uuid: Set(login_info.uuid.to_owned()),
                qr_id: Set(id.to_owned()),
                imeicode: Set(None),
                create_at: Set(time_now),
                ..Default::default()
            }
            .insert(Database::get_conn())
            .await?;
        }

        Ok(login_info)
    }

    pub async fn get_qr_status(uuid: &str, id: &str) -> BusinessResult<WxQrcodeStatus> {
        if entity::qr_scan::Entity::find()
            .filter(entity::qr_scan::Column::QrUuid.eq(uuid))
            .filter(entity::qr_scan::Column::QrId.eq(id))
            .count(Database::get_conn())
            .await?
            != 1
        {
            return Err(BusinessError::new_code("二维码信息不存在，请重新获取", 404));
        }

        let qr_code_result = WxLogin::get_qr_code_status(uuid).await?;

        match &qr_code_result {
            WxQrcodeStatus::Success(token) => {
                let sunrun = Sunrun::new_by_wx_token(&token).await?;
                let imei = sunrun.get_imeicode();

                entity::qr_scan::Entity::update_many()
                    .filter(entity::qr_scan::Column::QrUuid.eq(uuid))
                    .col_expr(entity::qr_scan::Column::Imeicode, Expr::value(imei))
                    .exec(Database::get_conn())
                    .await?;
            }
            WxQrcodeStatus::Error | WxQrcodeStatus::Cancel | WxQrcodeStatus::Expire => {
                entity::qr_scan::Entity::delete_many()
                    .filter(entity::qr_scan::Column::QrUuid.eq(uuid))
                    .exec(Database::get_conn())
                    .await?;
            }
            _ => (),
        };
        Ok(qr_code_result)
    }

    pub async fn get_qr_result(id: &str) -> BusinessResult<String> {
        #[derive(FromQueryResult)]
        struct QueryResult {
            imeicode: Option<String>,
        }

        let result = entity::qr_scan::Entity::find()
            .filter(entity::qr_scan::Column::QrId.eq(id))
            .select_only()
            .column(entity::qr_scan::Column::Imeicode)
            .into_model::<QueryResult>()
            .one(Database::get_conn())
            .await?;

        if let Some(result) = result {
            if let Some(result) = result.imeicode {
                entity::qr_scan::Entity::delete_many()
                    .filter(entity::qr_scan::Column::QrId.eq(id))
                    .exec(Database::get_conn())
                    .await?;
                Ok(result)
            } else {
                Err(BusinessError::new_code("二维码尚未被扫描", 201))
            }
        } else {
            Err(BusinessError::new_code("二维码已过期或未被创建", 402))
        }
    }
}
