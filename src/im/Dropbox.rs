#[cfg(feature = "ImDropbox")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImDropbox")]
/// *This icon requires the feature* `ImDropbox` *to be enabled*.
#[component]
pub fn Dropbox(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M11.5 0.5l-3.5 3 4.5 3 3.5-3z" /><path fill="#000000" d="M8 3.5l-3.5-3-4.5 3 3.5 3z" /><path fill="#000000" d="M12.5 6.5l3.5 3-4.5 2.5-3.5-3z" /><path fill="#000000" d="M8 9l-4.5-2.5-3.5 3 4.5 2.5z" /><path fill="#000000" d="M11.377 13.212l-3.377-2.895-3.377 2.895-2.123-1.179v1.467l5.5 2.5 5.5-2.5v-1.467z" /></svg>
   }
}