#[cfg(feature = "ImPageBreak")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPageBreak")]
/// *This icon requires the feature* `ImPageBreak` *to be enabled*.
#[component]
pub fn PageBreak(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 8h2v1h-2zM3 8h3v1h-3zM7 8h2v1h-2zM10 8h3v1h-3zM14 8h2v1h-2zM13.75 0l0.25 7h-12l0.25-7h0.5l0.25 6h10l0.25-6zM2.25 16l-0.25-6h12l-0.25 6h-0.5l-0.25-5h-10l-0.25 5z" /></svg>
   }
}