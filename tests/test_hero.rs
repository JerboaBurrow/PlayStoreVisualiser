#[cfg(test)]
mod test_hero
{
    use psv::{generate::{page_type, PageType}, util::read_file_utf8};

    #[test]
    /// This tests for runtime errors during generation from a known input (cached in the repo)
    fn generate_from_cached()
    {
        let html_list = read_file_utf8("tests/list.html").unwrap();
        let html_hero_vid = read_file_utf8("tests/hero-video.html").unwrap();
        let html_hero_screen = read_file_utf8("tests/hero-screenshots.html").unwrap();
        let html_hero_video_screen = read_file_utf8("tests/hero.html").unwrap();
        
        assert_eq!(page_type(&html_list), PageType::List);
        assert_eq!(page_type(&html_hero_vid), PageType::Hero);
        assert_eq!(page_type(&html_hero_screen), PageType::Hero);
        assert_eq!(page_type(&html_hero_video_screen), PageType::Hero);
        
    }
}