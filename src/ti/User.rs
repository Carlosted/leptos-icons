#[cfg(feature = "TiUser")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiUser")]
/// *This icon requires the feature* `TiUser` *to be enabled*.
#[component]
pub fn User(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M17 9c0-1.381-.56-2.631-1.464-3.535-.905-.905-2.155-1.465-3.536-1.465s-2.631.56-3.536 1.465c-.904.904-1.464 2.154-1.464 3.535s.56 2.631 1.464 3.535c.905.905 2.155 1.465 3.536 1.465s2.631-.56 3.536-1.465c.904-.904 1.464-2.154 1.464-3.535zM6 19c0 1 2.25 2 6 2 3.518 0 6-1 6-2 0-2-2.354-4-6-4-3.75 0-6 2-6 4z" /></svg>
   }
}