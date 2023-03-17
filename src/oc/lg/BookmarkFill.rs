#[cfg(feature = "OcLgBookmarkFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgBookmarkFill")]
/// *This icon requires the feature* `OcLgBookmarkFill` *to be enabled*.
#[component]
pub fn BookmarkFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6.69 2h10.56c.966 0 1.75.784 1.75 1.75v17.5a.75.75 0 0 1-1.218.585L12 17.21l-5.781 4.626A.75.75 0 0 1 5 21.253L4.94 3.756A1.748 1.748 0 0 1 6.69 2Z" /></svg>
   }
}