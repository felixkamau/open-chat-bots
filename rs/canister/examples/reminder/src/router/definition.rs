use super::commands;
use oc_bots_sdk::{
    api::definition::{AutonomousConfig, BotDefinition},
    types::BotPermissions,
};
use oc_bots_sdk_canister::{HttpRequest, HttpResponse};

pub async fn get(_request: HttpRequest) -> HttpResponse {
    HttpResponse::json(
        200,
        &BotDefinition {
            description: "Use this bot to send reminder messages.\n\nYou could use this bot in a direct chat for personal reminders or in a group/channel to remind all members - perhaps about a daily meeting.\n\nexample: \n\n/remind \"Team standup starts now\" \"every weekday at 10 am UTC\"".to_string(),
            commands: commands::definitions(),
            autonomous_config: Some(AutonomousConfig {
                permissions: BotPermissions::text_only(),
                sync_api_key: true,
            }),
        },
    )
}
