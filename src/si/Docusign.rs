#[cfg(feature = "SiDocusign")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiDocusign")]
/// *This icon requires the feature* `SiDocusign` *to be enabled*.
#[component]
pub fn Docusign(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M9.517 3.31h4.966v6.621h3.31L12 16.552 6.207 9.931h3.31V3.31zM0 19.034h24v1.655H0v-1.655z" /></svg>
   }
}