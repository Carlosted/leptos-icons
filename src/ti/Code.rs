#[cfg(feature = "TiCode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiCode")]
/// *This icon requires the feature* `TiCode` *to be enabled*.
#[component]
pub fn Code(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M8.171 18c-.512 0-1.023-.195-1.414-.586l-4.414-4.414 4.414-4.414c.781-.781 2.049-.781 2.828 0 .781.781.781 2.047 0 2.828l-1.585 1.586 1.585 1.586c.781.781.781 2.047 0 2.828-.39.391-.902.586-1.414.586zM15.829 18c-.512 0-1.024-.195-1.414-.586-.781-.781-.781-2.047 0-2.828l1.585-1.586-1.585-1.586c-.781-.781-.781-2.047 0-2.828.779-.781 2.047-.781 2.828 0l4.414 4.414-4.414 4.414c-.39.391-.902.586-1.414.586z" /></svg>
   }
}