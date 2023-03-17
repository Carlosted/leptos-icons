#[cfg(feature = "IoTabletPortraitSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTabletPortraitSharp")]
/// *This icon requires the feature* `IoTabletPortraitSharp` *to be enabled*.
#[component]
pub fn TabletPortraitSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M430,0H82A18,18,0,0,0,64,18V494a18,18,0,0,0,18,18H430a18,18,0,0,0,18-18V18A18,18,0,0,0,430,0ZM100,448V64H412V448Z" /></svg>
   }
}