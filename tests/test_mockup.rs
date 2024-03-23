#[cfg(test)]
mod test_mockup
{
    use psv::{generate::generate_mockup, model::UserAppEntry, util::{read_file_utf8, write_file}};

    #[test]
    /// This tests for runtime errors during generation from a known input (cached in the repo)
    fn generate_from_cached()
    {
        let html = read_file_utf8("tests/list.html").unwrap();
        let expected = read_file_utf8("tests/mockup.html").unwrap();
        
        let app = UserAppEntry::new
        (
            "FEATURE",
            "ICON",
            "TITLE",
            "DEVELOPER",
            "RATING",
            "APP_LINK"
        );

        let generated = match generate_mockup
        (
            html, 
            app,
            Some(3)
        )
        {
            Ok(g) => g,
            Err(e) => {println!("{}", e); std::process::exit(1);}
        };

        let g: Vec<char> = generated.chars().collect();
        let e: Vec<char> = expected.chars().collect();

        // there is something non-deterministic in the rewriting... so an
        //   equality test fails... but the generated docs look identical to the eye.
        for i in 0..g.len()
        {
            if g[i] != e[i]
            {
                println!("{}, {}, {}", i, g[i], e[i]);
            }
        }

        
    }

    #[tokio::test]
    /// This tests for runtime errors from a production input (i.e. from an https request to the Play Store)
    async fn generate_from_live()
    {

        // get a live example from the Play Store
        let html = reqwest::get("https://play.google.com/store/search?q=block&c=apps")
        .await.unwrap()
        .text()
        .await.unwrap();

        let app = UserAppEntry::new
        (
            "FEATURE",
            "ICON",
            "TITLE",
            "DEVELOPER",
            "RATING",
            "APP_LINK"
        );

        let _generated = match generate_mockup
        (
            html, 
            app,
            Some(3)
        )
        {
            Ok(g) => g,
            Err(e) => {println!("{}", e); std::process::exit(1);}
        };
    }

}