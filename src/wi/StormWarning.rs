#[cfg(feature = "WiStormWarning")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "WiStormWarning")]
/// *This icon requires the feature* `WiStormWarning` *to be enabled*.
#[component]
pub fn StormWarning(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" id="Layer_1" x="0px" y="0px" viewBox="0 0 30 30" style="enable-background:new 0 0 30 30;" xml:space="preserve"><path d="M9.76,24.6V7.45h1.13V24.6H9.76z M11.7,14.05v-6.6h8.55v6.6H11.7z M14.06,12.05h3.81v-2.5h-3.81V12.05z" /></svg>
   }
}