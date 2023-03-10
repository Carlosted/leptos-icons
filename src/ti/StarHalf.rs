#[cfg(feature = "TiStarHalf")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiStarHalf")]
/// *This icon requires the feature* `TiStarHalf` *to be enabled*.
#[component]
pub fn StarHalf(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M11.5 4.3c-.9 1.9-2.2 4.8-2.2 4.8s-3.1.4-5.2.6c-.2 0-.4.2-.4.3-.1.2 0 .4.1.5 1.6 1.4 3.9 3.6 3.9 3.6s-.6 3.1-1.1 5.2c0 .2 0 .4.2.5.2.2.4.2.6.1 1.8-1 4.6-2.6 4.6-2.6v-13.3c-.2 0-.4.2-.5.3z" /></svg>
   }
}