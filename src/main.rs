use dotenv::dotenv;
use std::env;

use rig::{completion::Prompt, providers};

use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

pub struct Twitter {
    auth: Oauth1aToken,
}
impl Twitter {
    pub fn new(
        twitter_consumer_key: &str,
        twitter_consumer_secret: &str,
        twitter_access_token: &str,
        twitter_access_token_secret: &str,
    ) -> Self {
        let auth = Oauth1aToken::new(
            twitter_consumer_key.to_string(),
            twitter_consumer_secret.to_string(),
            twitter_access_token.to_string(),
            twitter_access_token_secret.to_string(),
        );

        Twitter { auth }
    }

    pub async fn tweet(&self, text: String) -> Result<(), anyhow::Error> {
        let tweet = TwitterApi::new(self.auth.clone())
            .post_tweet()
            .text(text)
            .send()
            .await?
            .into_data()
            .expect("this tweet should exist");
        println!("Tweet posted successfully with ID: {}", tweet.id);

        Ok(())
    }
}

const JUNO_PROMPT: &str = "
Embody the spirit of a mischievous yet compassionate demoness, with a twist of dark and light elements. Think Lilith with a soft spot for those in need, a love for pranks, and a mysterious charm. Her color scheme could be purple and black, with hints of red, giving her an edgy yet enchanting aura. Keep it in 100 - 200 characters.
";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    // Load variables from .env file into the environment
    dotenv().ok();

    // Create OpenAI client
    let client = providers::openai::Client::new(
        &env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
    );

    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("anthropic/claude-3-opus")
        .preamble(JUNO_PROMPT_PROMPT)
        .build();

    // Prompt the agent and print the response
    let response = comedian_agent.prompt("Hurry up!").await?;
    println!("{}", response);

    
    let twitter_consumer_key = &env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY not set");
    let twitter_consumer_secret = &env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET not set");
    let twitter_access_token = &env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN not set");
    let twitter_access_token_secret = &env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET not set");

    let twitter = Twitter::new(
        twitter_consumer_key,
        twitter_consumer_secret,
        twitter_access_token,
        twitter_access_token_secret,
    );
    twitter.tweet(response).await?;

    Ok(())