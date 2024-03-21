use std::vec;

use play_store_visualiser::util::read_file_utf8;
use scraper::{selectable::Selectable, Html, Selector};
use lol_html::{element, HtmlRewriter, Settings};
use rand::prelude::*;

#[derive(Debug)]
struct AppEntry
{
    pub feature: String,
    pub icon: String,
    pub title: String,
    pub developer: String,
    pub rating: String
}

impl AppEntry
{
    pub fn new() -> AppEntry
    {
        AppEntry
        {
            feature: String::new(),
            icon: String::new(),
            title: String::new(),
            developer: String::new(),
            rating: String::new()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{

    let mut replaced = 0;

    let html = read_file_utf8("tests/example.html").unwrap();

    let selector = Selector::parse(r#"div[role="listitem"]"#).unwrap();
    let img_icon = Selector::parse(r#"img[alt="Thumbnail image"]"#).unwrap();
    let img_feature = Selector::parse(r#"img[alt="Screenshot image"]"#).unwrap();
    let span = Selector::parse(r#"span"#).unwrap();

    let document = Html::parse_document(&html);
    let mut apps: Vec<AppEntry> = vec![];

    for element in document.select(&selector)
    {
        let features: Vec<_> = element.select(&img_feature).collect();
        let icons: Vec<_> = element.select(&img_icon).collect();
        let spans: Vec<_> = element.select(&span).collect();

        println!("{}, {}, {}", features.len(), icons.len(), spans.len());

        if features.len() == 1 && icons.len() == 1 && spans.len() >= 2
        {
            // this is a valid play store item
            let feature = features.first().unwrap();
            let icon = icons.first().unwrap();
            let s = spans;

            let mut entry = AppEntry::new();

            entry.feature = feature.attr("src").unwrap().to_string();
            entry.icon = icon.attr("src").unwrap().to_string();
            entry.title = s[0].inner_html();
            entry.developer = s[1].inner_html();
            entry.rating = if s.len() >= 2 { s[2].inner_html() } else { " ".to_string() };
            println!("Found: {:?}", entry);

            apps.push(entry);

        }
    }

    let mut output = vec![];
    

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!(r#"img[alt="Thumbnail image"]"#, |el| {
                    replaced += 1;
                    println!("{}", replaced);
                    el.set_attribute("src", "SRC")?;
                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );
    rewriter.write(html.as_bytes())?;
    rewriter.end()?;
    //println!("{}",String::from_utf8(output).unwrap());

    Ok(())
}
