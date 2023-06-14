use fantoccini::{ClientBuilder, Locator};
use tokio;

#[tokio::main]
    async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    c.goto("https://webant.online/").await?;

    let h2 = c.find(Locator::Css("h2")).await?.html(true).await?;
    println!("{}", h2);

    c.find(Locator::LinkText("Theory")).await?.click().await?;
    let quote = c.find(Locator::Css("p.page__description")).await?.html(true).await?;
    let author = c.find(Locator::Css("p.author")).await?.html(true).await?;
    println!("{} {}", quote, author);

    c.find(Locator::LinkText("Tools")).await?.click().await?;
    let quote = c.find(Locator::Css("p.page__description")).await?.html(true).await?;
    let author = c.find(Locator::Css("p.author")).await?.html(true).await?;
    println!("{} {}", quote, author);

    c.close().await
}