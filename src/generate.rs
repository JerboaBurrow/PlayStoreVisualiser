use std::{fmt, vec};

use scraper::{selectable::Selectable, Html, Selector};
use lol_html::{element, html_content::ContentType, text, HtmlRewriter, Settings};
use rand::prelude::*;

use crate::model::{AppEntry, UserAppEntry};

#[derive(Debug, Clone)]
pub struct MockupError
{
    pub why: String
}

impl fmt::Display for MockupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} while mocking up content", self.why)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageType {List, Hero}

pub fn page_type(html: &str) -> PageType
{
    let selector = Selector::parse(r#"div[role="listitem"]"#).unwrap();
    let img_icon = Selector::parse(r#"img[alt="Thumbnail image"]"#).unwrap();
    let img_feature = Selector::parse(r#"img[alt="Screenshot image"]"#).unwrap();
    let span = Selector::parse(r#"span"#).unwrap();
    let href = Selector::parse(r#"a"#).unwrap();

    let document = Html::parse_document(&html);
    let mut apps: Vec<AppEntry> = vec![];

    let mut is_hero_video: bool = false;
    let mut is_hero_screenshots: bool = false;

    for element in document.select(&selector)
    {
        let features: Vec<_> = element.select(&img_feature).collect();
        let icons: Vec<_> = element.select(&img_icon).collect();
        let spans: Vec<_> = element.select(&span).collect();
        let hrefs: Vec<_> = element.select(&href).collect();

        if features.len() == 1 && icons.len() == 1 && spans.len() >= 2 && hrefs.len() == 1
        {
            continue;
        }
        else 
        {
            let images: Vec<_> = element.select(&Selector::parse("img").unwrap()).collect();
            let screenshots: Vec<_> = element.select(&Selector::parse("img[alt=\"Screenshot image\"]").unwrap()).collect();
            let buttons: Vec<_> = element.select(&Selector::parse("button").unwrap()).collect();

            if !is_hero_video && screenshots.len() > 0
            {
                is_hero_screenshots = true;
            }
            else if buttons.len() == 1 && images.len() == 1
            {
                is_hero_video = true;
            }
        }
    }

    if is_hero_screenshots || is_hero_video { PageType::Hero } else { PageType::List }
}

/// Replaces the listitem element at the given ```position``` by the 
///  given ```app``` see [AppEntry] and [UserAppEntry]
pub fn generate_mockup
(
    html: String, 
    app: UserAppEntry,
    position: Option<usize>
) -> Result<String, MockupError>
{
    let page = page_type(&html);
    // assume these structures exist in the html
    let selector = Selector::parse(r#"div[role="listitem"]"#).unwrap();
    let img_icon = Selector::parse(r#"img[alt="Thumbnail image"]"#).unwrap();
    let img_feature = Selector::parse(r#"img[alt="Screenshot image"]"#).unwrap();
    let span = Selector::parse(r#"span"#).unwrap();
    let href = Selector::parse(r#"a"#).unwrap();

    let document = Html::parse_document(&html);
    let mut apps: Vec<AppEntry> = vec![];

    let mut skipping: bool = true;
    let mut skip: usize = 0;

    for element in document.select(&selector)
    {
        let features: Vec<_> = element.select(&img_feature).collect();
        let icons: Vec<_> = element.select(&img_icon).collect();
        let spans: Vec<_> = element.select(&span).collect();
        let hrefs: Vec<_> = element.select(&href).collect();

        if features.len() == 1 && icons.len() == 1 && spans.len() >= 2 && hrefs.len() == 1
        {
            skipping = false;
            // this is a valid play store item
            let feature = features.first().unwrap();
            let icon = icons.first().unwrap();
            let s = spans;
            let h = hrefs.first().unwrap();

            let mut entry = AppEntry::new();

            entry.feature = feature.attr("src").unwrap().to_string();
            entry.icon = icon.attr("src").unwrap().to_string();
            entry.title = s[0].inner_html();
            entry.developer = s[1].inner_html();
            entry.rating = if s.len() >= 2 { s[2].inner_html() } else { " ".to_string() };
            entry.link = h.attr("href").unwrap().to_string();
            entry.html = scraper::ElementRef::wrap(element.first_child().unwrap()).unwrap().html();
            apps.push(entry);
        }
        else if skipping
        {
            skip += 1;
        }
    }

    let selected_app: &AppEntry = apps.first().unwrap();

    let feature_pattern = "img[src=\"".to_string()+selected_app.feature.as_str()+"\"]";
    let icon_pattern = "img[src=\"".to_string()+selected_app.icon.as_str()+"\"]";
    let link_pattern = "a[href=\"".to_string()+selected_app.link.as_str()+"\"]";

    let mut output = vec![];

    // First rewrite the html taken from an app, assuming feature_, icon_, and link_pattern's
    //   exist. Could use a set template but it's possible some of the other html/css may
    //   change later. 
    let mut app_rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!(feature_pattern, |el| {
                    el.set_attribute("src", &app.feature_link)?;
                    Ok(())
                }),
                element!(icon_pattern, |el| {
                    el.set_attribute("src", &app.icon_link)?;
                    Ok(())
                }),
                element!(link_pattern, |el| {
                    el.set_attribute("href", &app.app_link)?;
                    Ok(())
                }),
                text!(r#"span"#, |t| {
                    if t.as_str() == selected_app.title
                    {
                        t.replace(&app.title, ContentType::Text);
                    }
                    else if t.as_str() == selected_app.developer
                    {
                        t.replace(&app.developer, ContentType::Text);
                    }
                    else if t.as_str() == selected_app.rating
                    {
                        t.replace(&app.rating, ContentType::Text)
                    }
                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );
    
    match app_rewriter.write(selected_app.html.as_bytes())
    {
        Ok(_) => {},
        Err(e) => {return Err(MockupError{ why: format!("{}",e)})}
    }
    
    match app_rewriter.end() {
        Ok(_) => {},
        Err(e) => {return Err(MockupError{ why: format!("{}",e)})}
    }

    let replacement_html = String::from_utf8(output).unwrap();

    let mut generated_page = vec![];

    let replacement_index = if position.is_some_and(|p| (3..apps.len()).contains(&p))
    {
        position.unwrap() + skip
    }
    else
    {
        3 + (random::<usize>() % (apps.len()-3)) + skip
    };

    let mut index: usize = 0;

    // replace the replacement_index'th app html with the re-written version
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!(r#"div[role="listitem"]"#, |el| {
                    if index == replacement_index
                    {
                        el.set_inner_content(&replacement_html, ContentType::Html);
                    }
                    index += 1;
                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| generated_page.extend_from_slice(c)
    );

    match rewriter.write(html.as_bytes()) {
        Ok(_) => {},
        Err(e) => {return Err(MockupError{ why: format!("{}",e)})}
    }

    match rewriter.end() {
        Ok(_) => {},
        Err(e) => {return Err(MockupError{ why: format!("{}",e)})}
    }

    let page = match String::from_utf8(generated_page) {
        Ok(gen) => {gen},
        Err(e) => {return Err(MockupError{ why: format!("{}",e)})}
    };

    insert_ribbon(page, "Generated by PSV!")
}

const ribbon_style: &str = r#"
<style>
.ribbon {
    z-index: 1;
    font-size: 28px;
    font-weight: bold;
    color: #1c1c1c;
    position: absolute;
    top: 0;
    right: 0;
    line-height: 1.8;
    padding-inline: 1lh;
    clip-path: polygon(
      100% 100%,0 100%,999px calc(100% - 999px),calc(100% - 999px) calc(100% - 999px));
    transform: translate(calc((1 - cos(45deg))*100%), -100%) rotate(45deg);
    transform-origin: 0% 100%;
    background-color: #C6C6F8; /* the main color  */
  }
</style>"#;

const img_ribbon: &str = r#"<svg viewBox="0 0 100 100" width="28" height="27" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z" fill="\#24292f"/></svg>"#;

pub fn insert_ribbon(html: String, text: &str) -> Result<String, MockupError>
{
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("head", |el| {
                    el.prepend(ribbon_style, ContentType::Html);
                    Ok(())
                }),
                element!("header[role=\"banner\"]", |el| {
                    el.prepend(&format!("<a href=\"https://github.com/JerboaBurrow/PlayStoreVisualiser\"><div class=\"ribbon\">{}{}</div></a>",text, img_ribbon)  , ContentType::Html);
                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );

    match rewriter.write(html.as_bytes()) {
        Ok(_) => {},
        Err(e) => {return Err(MockupError{ why: format!("{}",e)})}
    }

    match rewriter.end() {
        Ok(_) => {},
        Err(e) => {return Err(MockupError{ why: format!("{}",e)})}
    }

    match String::from_utf8(output) {
        Ok(gen) => {Ok(gen)},
        Err(e) => {return Err(MockupError{ why: format!("{}",e)})}
    }
}