#[cfg(feature = "SiSteinberg")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSteinberg")]
/// *This icon requires the feature* `SiSteinberg` *to be enabled*.
#[component]
pub fn Steinberg(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M19.4807 12.6291c.6422-.371.6422-1.0092 0-1.3792L14.711 8.4954c-.6422-.371-1.1952-.052-1.1952.6901v5.508c0 .741.553 1.0601 1.1952.69zm-7.4812-9.9036c5.1219 0 9.2745 4.1526 9.2745 9.2745s-4.1526 9.2745-9.2745 9.2745S2.726 17.122 2.726 12s4.1516-9.2745 9.2735-9.2745m0-2.7255C5.3834 0 .0005 5.3829.0005 12s5.3829 12 12 12 11.999-5.3829 11.999-12-5.3829-12-12-12z" /></svg>
   }
}