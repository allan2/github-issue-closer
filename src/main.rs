use std::{env, error, fmt, fs};

use clap::{Parser, Subcommand};
use octocrab::{
    models::{issues::IssueStateReason, InstallationToken, IssueState},
    params::{apps::CreateInstallationAccessToken, State},
    OctocrabBuilder,
};

#[derive(Parser)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    CloseAll {
        #[arg(long)]
        owner: String,
        #[arg(long)]
        repo: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app_id = env::var("GITHUB_APP_ID")
        .map_err(|_| Error::GitHubAppId)?
        .parse::<u64>()
        .map_err(|_| Error::GitHubAppId)?
        .into();

    let key_path =
        env::var("GITHUB_APP_PRIVATE_KEY_PATH").map_err(|_| Error::GitHubAppPrivateKey)?;
    let key = fs::read_to_string(key_path).map_err(|_| Error::GitHubAppPrivateKey)?;
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(key.as_bytes())?;
    let token = octocrab::auth::create_jwt(app_id, &key)?;

    let octocrab = OctocrabBuilder::new().personal_token(token).build()?;

    let cli = Cli::parse();
    let (owner, repo) = match cli.command {
        Command::CloseAll { owner, repo } => (owner, repo),
    };

    // Create an installation access token.
    // https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-an-installation-access-token-for-an-app
    let installations = octocrab.apps().installations().send().await?.take_items();

    let mut token = CreateInstallationAccessToken::default();
    token.repositories = vec![repo.to_owned()];

    let access: InstallationToken = octocrab
        .post(
            installations[0].access_tokens_url.as_ref().unwrap(),
            Some(&token),
        )
        .await?;

    let octocrab = OctocrabBuilder::new()
        .personal_token(access.token)
        .build()?;

    // Get open issues and open PRs.
    let mut page = octocrab
        .issues(&owner, &repo)
        .list()
        .state(State::Open)
        .send()
        .await?;

    let mut count = 0;
    // Pages are not rate-limited. Loop through all pages.
    loop {
        for issue in &page {
            count += 1;

            let label = if issue.pull_request.is_some() {
                "PR"
            } else {
                "Issue"
            };
            println!(
                "{:3} - {:5} {:3} - {}",
                count, label, issue.number, issue.title
            );

            // Close the current issue or PR.
            octocrab
                .issues(&owner, &repo)
                .update(issue.number)
                .state(IssueState::Closed)
                .state_reason(IssueStateReason::NotPlanned)
                .send()
                .await?;
        }

        page = match octocrab.get_page(&page.next).await? {
            Some(next_page) => next_page,
            None => break,
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Error {
    GitHubAppId,
    GitHubAppPrivateKey,
    GitHubAppPrivateKeyDecode(jsonwebtoken::errors::Error),
    Octocrab(octocrab::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GitHubAppId => write!(f, "GITHUB_APP_ID not set or invalid"),
            Error::GitHubAppPrivateKey => write!(f, "GITHUB_APP_PRIVATE_KEY not set or invalid"),
            Error::GitHubAppPrivateKeyDecode(e) => {
                write!(f, "GITHUB_APP_PRIVATE_KEY could not be decoded: {}", e)
            }
            Error::Octocrab(e) => write!(f, "Octocrab error: {}", e),
        }
    }
}

impl From<octocrab::Error> for Error {
    fn from(e: octocrab::Error) -> Self {
        Error::Octocrab(e)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Error::GitHubAppPrivateKeyDecode(e)
    }
}
