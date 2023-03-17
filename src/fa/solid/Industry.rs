#[cfg(feature = "FaSolidIndustry")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidIndustry")]
/// *This icon requires the feature* `FaSolidIndustry` *to be enabled*.
#[component]
pub fn Industry(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M32 32C14.3 32 0 46.3 0 64V304v48 80c0 26.5 21.5 48 48 48H464c26.5 0 48-21.5 48-48V304 152.2c0-18.2-19.4-29.7-35.4-21.1L320 215.4V152.2c0-18.2-19.4-29.7-35.4-21.1L128 215.4V64c0-17.7-14.3-32-32-32H32z" /></svg>
   }
}