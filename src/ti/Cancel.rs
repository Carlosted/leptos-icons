#[cfg(feature = "TiCancel")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiCancel")]
/// *This icon requires the feature* `TiCancel` *to be enabled*.
#[component]
pub fn Cancel(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M12 4c-4.411 0-8 3.589-8 8s3.589 8 8 8 8-3.589 8-8-3.589-8-8-8zm-5 8c0-.832.224-1.604.584-2.295l6.711 6.711c-.691.36-1.463.584-2.295.584-2.757 0-5-2.243-5-5zm9.416 2.295l-6.711-6.711c.691-.36 1.463-.584 2.295-.584 2.757 0 5 2.243 5 5 0 .832-.224 1.604-.584 2.295z" /></svg>
   }
}