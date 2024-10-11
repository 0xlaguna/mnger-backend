use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::routes::account::login::{self};
use crate::routes::users::create_user::{self};
use crate::routes::users::edit_user::{self};
use crate::routes::users::fetch_profile::{self};
use crate::routes::users::me::{self};
use crate::routes::workorder::create_workorder::{self};
use crate::routes::workorder::list_workorders::{self};

use mnger_preon::dto::users::DataEditUser;
use mnger_preon::dto::users::{
    DataCreateAccount, DataLoginAccount, LoginResponse, User, UserGetMeData,
};
use mnger_preon::dto::work_order::{DataCreateWorkOrder, WorkOrderItem, WorkOrderListData};

#[derive(OpenApi)]
#[openapi(
    paths(
        login::req,
        create_user::req,
        fetch_profile::req,
        edit_user::req,
        create_workorder::req,
        list_workorders::req,
        me::req,
    ),
    components(
        schemas(DataLoginAccount, LoginResponse),
        schemas(DataCreateAccount, User, DataEditUser, UserGetMeData),
        schemas(DataCreateWorkOrder),
        schemas(WorkOrderItem, WorkOrderListData),
    ),
    tags(
        (name = "Mnger", description = "Mnger endpoints.")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "x-session-token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-session-token"))),
        )
    }
}
