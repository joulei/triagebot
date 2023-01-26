// Function to match the scheduled job function with its corresponding handler.
// In case you want to add a new one, just add a new clause to the match with
// the job name and the corresponding function.

// Further info could be find in src/jobs.rs
use super::Context;
use crate::github::*;
use crate::handlers::decision::{DecisionProcessActionMetadata, DECISION_PROCESS_JOB_NAME};
use parser::command::decision::Resolution::{Hold, Merge};
use reqwest::Client;
use tracing as log;

pub async fn handle_job(
    ctx: &Context,
    name: &String,
    metadata: &serde_json::Value,
) -> anyhow::Result<()> {
    match name.as_str() {
        "docs_update" => super::docs_update::handle_job().await,
        "rustc_commits" => {
            super::rustc_commits::synchronize_commits_inner(ctx, None).await;
            Ok(())
        }
        matched_name if *matched_name == DECISION_PROCESS_JOB_NAME.to_string() => {
            decision_process_handler(&metadata).await
        }
        _ => default(&name, &metadata),
    }
}

fn default(name: &String, metadata: &serde_json::Value) -> anyhow::Result<()> {
    tracing::trace!(
        "handle_job fell into default case: (name={:?}, metadata={:?})",
        name,
        metadata
    );

    Ok(())
}

async fn decision_process_handler(metadata: &serde_json::Value) -> anyhow::Result<()> {
    tracing::trace!(
        "handle_job fell into decision process case: (metadata={:?})",
        metadata
    );

    let metadata: DecisionProcessActionMetadata = serde_json::from_value(metadata.clone())?;
    let gh_client = GithubClient::new_with_default_token(Client::new().clone());
    let request = gh_client.get(&metadata.get_issue_url);

    match gh_client.json::<Issue>(request).await {
        Ok(issue) => match metadata.status {
            Merge => issue.merge(&gh_client).await?,
            Hold => issue.close(&gh_client).await?,
        },
        Err(e) => log::error!(
            "Failed to get issue {}, error: {}",
            metadata.get_issue_url,
            e
        ),
    }

    Ok(())
}
