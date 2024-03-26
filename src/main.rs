use psv::{generate::generate_mockup, model::UserAppEntry, program_version, util::{read_file_utf8, write_file}};

fn get_argument(arg: &str, args: &Vec<String>, default: &str) -> String
{
    if args.iter().any(|x| x == &arg)
    {
        let index = args.iter().position(|a| a == arg).unwrap();

        if index < args.len()
        {
            args.get(index+1).unwrap().clone()
        }
        else
        {
            default.to_string()
        }
    }
    else
    {
        default.to_string()
    }
}

#[tokio::main]
async fn main()
{

    let args: Vec<String> = std::env::args().collect();
 
    if args.iter().any(|x| x == "-v")
    {
        println!("Version: {}", program_version());
        std::process::exit(0);
    }

    let feature = get_argument("-feature", &args, "FEATURE");
    let icon = get_argument("-icon", &args, "ICON");
    let title = get_argument("-title", &args, "TITLE");
    let dev = get_argument("-developer", &args, "DEVELOPER");
    let stars = get_argument("-stars", &args, "RATING");
    let link = get_argument("-link", &args, "APP_LINK");
    let position: usize = match get_argument("-position", &args, "0").parse::<usize>()
    {
        Ok(p) => {p},
        Err(e) => {println!("-position must be a positive integer\n{}", e); std::process::exit(1);}
    };

    let query = get_argument("-query", &args, "particles");

    // get a live example from the Play Store
    let html = reqwest::get(format!("https://play.google.com/store/search?q={}&c=apps",query))
    .await.unwrap()
    .text()
    .await.unwrap();
    
    let app = UserAppEntry::new
    (
        &feature, &icon, &title, &dev, &stars, &link
    );

    let generated = match generate_mockup
    (
        html, 
        app,
        Some(position)
    )
    {
        Ok(g) => g,
        Err(e) => {println!("{}", e); std::process::exit(1);}
    };

    write_file("mockup.html", generated.as_bytes());
}
