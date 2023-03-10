#[cfg(feature = "ImUndo2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImUndo2")]
/// *This icon requires the feature* `ImUndo2` *to be enabled*.
#[component]
pub fn Undo2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M11.904 16c1.777-3.219 2.076-8.13-4.904-7.966v3.966l-6-6 6-6v3.881c8.359-0.218 9.29 7.378 4.904 12.119z" /></svg>
   }
}