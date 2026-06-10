use anyhow::Result;

use crate::context::Context;
use crate::output;

pub async fn run(ctx: &Context) -> Result<()> {
    let client = ctx.client()?;
    let resp = client.whoami().await?;
    // Humans see the user object directly; --json/--jq get the full payload.
    if ctx.global.json || ctx.global.jq.is_some() {
        output::print_single(&ctx.global, resp.data)
    } else {
        let user = resp.data.get("user").cloned().unwrap_or(resp.data);
        output::print_single(&ctx.global, user)
    }
}
