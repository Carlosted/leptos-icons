#[cfg(feature = "ImSpinner11")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImSpinner11")]
/// *This icon requires the feature* `ImSpinner11` *to be enabled*.
#[component]
pub fn Spinner11(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M16 6h-6l2.243-2.243c-1.133-1.133-2.64-1.757-4.243-1.757s-3.109 0.624-4.243 1.757c-1.133 1.133-1.757 2.64-1.757 4.243s0.624 3.109 1.757 4.243c1.133 1.133 2.64 1.757 4.243 1.757s3.109-0.624 4.243-1.757c0.095-0.095 0.185-0.192 0.273-0.292l1.505 1.317c-1.466 1.674-3.62 2.732-6.020 2.732-4.418 0-8-3.582-8-8s3.582-8 8-8c2.209 0 4.209 0.896 5.656 2.344l2.343-2.344v6z" /></svg>
   }
}