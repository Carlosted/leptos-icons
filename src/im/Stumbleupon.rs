#[cfg(feature = "ImStumbleupon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImStumbleupon")]
/// *This icon requires the feature* `ImStumbleupon` *to be enabled*.
#[component]
pub fn Stumbleupon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 5c-0.55 0-1 0.45-1 1v4c0 1.653-1.347 3-3 3s-3-1.347-3-3v-2h2v2c0 0.55 0.45 1 1 1s1-0.45 1-1v-4c0-1.653 1.347-3 3-3s3 1.347 3 2.781v0.969l-1.281 0.375-0.719-0.375v-0.969c0-0.331-0.45-0.781-1-0.781z" /><path fill="#000000" d="M15 10c0 1.653-1.347 3-3 3s-3-1.347-3-3.219v-1.938l0.719 0.375 1.281-0.375v1.938c0 0.769 0.45 1.219 1 1.219s1-0.45 1-1v-2h2v2z" /></svg>
   }
}