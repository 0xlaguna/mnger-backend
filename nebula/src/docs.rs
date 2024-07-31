use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::routes::account::login::{DataLoginAccount, LoginResponse, self};
use crate::routes::users::create_user::{DataCreateAccount, self};
use crate::routes::users::fetch_profile::{FetchProfileResponse, self};
use crate::routes::users::me::{UserGetMeData, self};
use crate::routes::workorder::create_workorder::{DataCreateWorkOrder, self};
use crate::routes::workorder::list_workorders::{WorkOrderItem, WorkOrderListData, self};

#[derive(OpenApi)]
#[openapi(
    paths(
        login::req,
        create_user::req,
        fetch_profile::req,
        create_workorder::req,
        list_workorders::req,
        me::req,
    ),
    components(
        schemas(DataLoginAccount, LoginResponse),
        schemas(DataCreateAccount, FetchProfileResponse, UserGetMeData),
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
