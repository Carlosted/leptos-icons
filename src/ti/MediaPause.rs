#[cfg(feature = "TiMediaPause")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMediaPause")]
/// *This icon requires the feature* `TiMediaPause` *to be enabled*.
#[component]
pub fn MediaPause(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M8 6c-1.104 0-2 .896-2 2v8c0 1.104.896 2 2 2s2-.896 2-2v-8c0-1.104-.896-2-2-2zM15 6c-1.104 0-2 .896-2 2v8c0 1.104.896 2 2 2s2-.896 2-2v-8c0-1.104-.896-2-2-2z" /></svg>
   }
}