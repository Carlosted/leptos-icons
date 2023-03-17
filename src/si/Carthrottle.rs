#[cfg(feature = "SiCarthrottle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiCarthrottle")]
/// *This icon requires the feature* `SiCarthrottle` *to be enabled*.
#[component]
pub fn Carthrottle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 19.99h5.31l1-5.76h2.673L7.97 19.99h5.272l1.037-5.76h2.824l-1 5.76h7.584L21.9 17.029 24 4.01h-5.16l-.987 5.647h-2.86l.936-5.647H8.483l1.724 2.749-.487 2.898H6.996l.9-5.647H.35l1.76 2.774Z" /></svg>
   }
}