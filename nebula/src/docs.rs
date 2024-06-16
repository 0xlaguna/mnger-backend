use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::routes::account::login::{DataLoginAccount, LoginResponse};
use crate::routes::account::login;

#[derive(OpenApi)]
#[openapi(
    paths(
        login::req,
    ),
    components(
        schemas(DataLoginAccount, LoginResponse)
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
