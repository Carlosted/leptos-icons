#[cfg(feature = "SiSteamdeck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSteamdeck")]
/// *This icon requires the feature* `SiSteamdeck` *to be enabled*.
#[component]
pub fn Steamdeck(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M8.494 0v4.309c4.242 0 7.694 3.45 7.694 7.691s-3.452 7.691-7.694 7.691V24c6.617 0 12-5.383 12-12s-5.383-12-12-12zm10.819 3.62v.194h.328v.893h.228v-.893h.33V3.62zm1.037 0v1.087h.207v-.684l.298.653h.14l.29-.66v.691h.219V3.619h-.23l-.338.772-.368-.772zM8.494 6.011a5.998 5.998 0 0 0-5.998 6 5.998 5.998 0 1 0 11.998 0 6 6 0 0 0-6-6z" /></svg>
   }
}