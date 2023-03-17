#[cfg(feature = "BiRegularSlideshow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSlideshow")]
/// *This icon requires the feature* `BiRegularSlideshow` *to be enabled*.
#[component]
pub fn Slideshow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h7v3H8v2h8v-2h-3v-3h7c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zM4 15V5h16l.001 10H4z" /><path d="m10 13 5-3-5-3z" /></svg>
   }
}