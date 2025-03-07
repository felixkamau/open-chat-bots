use oc_bots_sdk::oc_api::actions::{delete_channel, ActionArgsBuilder};
use oc_bots_sdk::types::{AuthToken, ChannelId};
use oc_bots_sdk_canister::{HttpRequest, HttpResponse, OPENCHAT_CLIENT_FACTORY};

#[derive(serde::Deserialize)]
struct Args {
    channel_id: ChannelId,
    auth_token: AuthToken,
}

pub async fn execute(request: HttpRequest) -> HttpResponse {
    let args: Args = match super::extract_args(&request) {
        Ok(args) => args,
        Err(response) => return response,
    };

    let context = match super::extract_context(args.auth_token) {
        Ok(cxt) => cxt,
        Err(response) => return response,
    };

    let response = OPENCHAT_CLIENT_FACTORY
        .build(context)
        .delete_channel(args.channel_id)
        .execute_async()
        .await;

    match response {
        Ok(delete_channel::Response::Success) => HttpResponse::status(200),
        Err((code, message)) => HttpResponse::text(500, format!("{}: {}", code, message)),
        other => HttpResponse::text(500, format!("{:?}", other)),
    }
}
