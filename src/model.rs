#[derive(Debug)]
/// A model of the Google Play App store's listitem children
///  ```feature```: feature image url
/// ```icon```: icon image url
/// ```title```: app name/title
/// ```developer```: developer name
/// ```rating```: star rating (e.g. 4.7)
/// ```link```: link to Play Store page of the app
/// ```html```: scrapped html
/// 
/// The following is (currently) the format used on the Play Store
/// ```<div class="VfPpkd-WsjYwc VfPpkd-WsjYwc-OWXEXe-INsAgc KC1dQ Usd1Ac AaN0Dd  Y8RQXd">
///     <div class="VfPpkd-aGsRMb">
///     <div jsaction="click:cOuCgd; mousedown:UX7yZ; mouseup:lbsD7e; mouseenter:tfO1Yc; mouseleave:JywGue; touchstart:p6p2H; touchmove:FwuNnf; touchend:yfqBxc; touchcancel:JMtRjd; focus:AHmuwe; blur:O22p3e; contextmenu:mg9Pef" class="VfPpkd-EScbFb-JIbuQc TAQqTe" jscontroller="tKHFxf"><a class="Si6A0c Gy4nib" href="/store/apps/details?id=com.sunglab.tripleafree" jslog="38003; 1:577|qgJNGksIAEoTCL7Vx/jMh4UDFc+BCgkd98oK6/oBDwoNCAASCQoFZW4tVVMQALICHwodChdjb20uc3VuZ2xhYi50cmlwbGVhZnJlZRABGAM=; track:click,impression">
///         <div class="Vc0mnc"><img src="https://i.ytimg.com/vi/-8CPVh5i_R8/hqdefault.jpg" class="T75of nIMMJc" aria-hidden="true" alt="Screenshot image" loading="lazy">
///           <div class="aCy7Gf"><button aria-label="Play Triple A" class="FN1l2 XdjT2b" jscontroller="M2Qezd" jsaction="click:e7xSJf" jsmodel="hQqEkb" jsname="OvWdXe" data-video-url="https://play.google.com/video/lava/web/player/yt:movie:-8CPVh5i_R8?autoplay=1&amp;authuser=0&amp;embed=play" data-web-presentation="1" data-youtube-id="-8CPVh5i_R8" data-youtube-asset-id="yt:movie:-8CPVh5i_R8" data-stop-propagating-events="true" jslog="37885; 1:218|qgJNGksIAEoTCL7Vx/jMh4UDFc+BCgkd98oK6/oBDwoNCAASCQoFZW4tVVMQALICHwodChdjb20uc3VuZ2xhYi50cmlwbGVhZnJlZRABGAM=; track:click,impression"><span class="Qv3d6b" aria-hidden="true"><svg viewBox="0 0 56 56" fill="none" xmlns="http://www.w3.org/2000/svg" class="bKsVV">
///                   <path fill-rule="evenodd" clip-rule="evenodd" d="M28 56C43.464 56 56 43.464 56 28C56 12.536 43.464 0 28 0C12.536 0 0 12.536 0 28C0 43.464 12.536 56 28 56Z" fill="black" fill-opacity="0.54"></path>
///                    <path fill-rule="evenodd" clip-rule="evenodd" d="M39.6667 28L21 17.5V38.5L39.6667 28Z" fill="white"></path>
///                  </svg></span></button></div>
///          </div>
///          <div class="j2FCNc"><img src="https://play-lh.googleusercontent.com/YyUAftjICcXy7Z5rFtwNW9nAMsc02SmLwgO8UqbZeySPqbyxIVWcpGtnbI3lScx4i58=s64" srcset="https://play-lh.googleusercontent.com/YyUAftjICcXy7Z5rFtwNW9nAMsc02SmLwgO8UqbZeySPqbyxIVWcpGtnbI3lScx4i58=s128 2x" class="T75of stzEZd" aria-hidden="true" alt="Thumbnail image" loading="lazy">
///          <div class="cXFu1">
///            <div class="ubGTjb"><span class="DdYX5">Triple A</span></div>
///            <div class="ubGTjb"><span class="wMUdtb">SungLab Inc</span></div>
///            <div class="ubGTjb">
///              <div style="display: inline-flex; align-items: center;" aria-label="Rated 4.7 stars out of five stars"><span class="w2kbF">4.7</span><span class="Q4fJQd"><i class="google-material-icons Yvy3Fd" aria-hidden="true">star</i></span></div>
///            </div>
///          </div>
///        </div>
///      </a>
///      <div class="VfPpkd-FJ5hab"></div>
///     </div>
///    </div><span aria-hidden="true" class="VfPpkd-BFbNVe-bF1uUb NZp2ef"></span>
///    </div>```
pub struct AppEntry
{
    pub feature: String,
    pub icon: String,
    pub title: String,
    pub developer: String,
    pub rating: String,
    pub link: String,
    pub html: String
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
            rating: String::new(),
            link: String::new(),
            html: String::new()
        }
    }
}

#[derive(Debug)]
/// A model of a mockup Google Play store's listitem child
///  ```feature_link```: feature image url
/// ```icon_link```: icon image url
/// ```title```: app name/title
/// ```developer```: developer name
/// ```rating```: star rating (e.g. 4.7)
/// ```app_link```: link to Play Store page of the app
pub struct UserAppEntry
{
    pub feature_link: String,
    pub icon_link: String,
    pub title: String,
    pub developer: String,
    pub rating: String,
    pub app_link: String
}

impl UserAppEntry
{
    pub fn new
    (
        feature_link: &str,
        icon_link:  &str,
        title:  &str,
        developer:  &str,
        rating:  &str,
        app_link:  &str
    ) -> UserAppEntry
    {
        UserAppEntry
        {
            feature_link: feature_link.to_string(),
            icon_link: icon_link.to_string(),
            title: title.to_string(),
            developer: developer.to_string(),
            rating: rating.to_string(),
            app_link: app_link.to_string()
        }
    }
}