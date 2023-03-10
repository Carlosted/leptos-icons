#[cfg(feature = "TiAdjustContrast")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiAdjustContrast")]
/// *This icon requires the feature* `TiAdjustContrast` *to be enabled*.
#[component]
pub fn AdjustContrast(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M12 4c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zm0 14c-3.314 0-6-2.686-6-6s2.686-6 6-6 6 2.686 6 6-2.686 6-6 6zM12 7v10c2.757 0 5-2.243 5-5s-2.243-5-5-5z" /></g></svg>
   }
}