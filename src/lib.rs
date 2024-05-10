use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_cookie;
use leptos_use::utils::FromToStringCodec;
use utilities::cookies::JabraCookie;
use crate::dashboard::page::PageManager;

// Modules
mod components;
mod pages;
mod dashboard;
mod utilities;
mod models;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

#[derive(Copy, Clone)]
pub struct HasError(pub RwSignal<bool>);

#[derive(Copy, Clone)]
pub struct Refetcher(pub RwSignal<bool>);

#[derive(Copy, Clone)]
pub struct CheckCookie(pub Resource<bool, Result<bool, ServerFnError>>);

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let refetcher = create_rw_signal(false);
    let has_error = create_rw_signal(false);

    let auth_resource: Resource<bool, Result<bool, ServerFnError>> =
        create_local_resource(refetcher, move |_| async move {
            check_server_cookie("admin_portal_csr".to_string()).await
        });

    provide_context(Refetcher(refetcher));
    provide_context(HasError(has_error));
    provide_context(CheckCookie(auth_resource));

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="darkpurple"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <main class="font-poppins">
                <Routes>
                    <Route path="/" view=PageManager/>
                    <Route path="/login" view=PageManager/>
                    <Route path="/quotes/builder" view=PageManager/>
                    <Route path="/*" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

pub async fn check_server_cookie(cookie_name: String) -> Result<bool, ServerFnError> {
    let (cookie, _set_cookie) = use_cookie::<String, FromToStringCodec>(cookie_name.as_str());
    match cookie.get_untracked() {
        Some(val) => {
            if val.len() > 0 {
                match JabraCookie::decrypt(val) {
                    Ok(e) => Ok(!e.is_expired()),
                    Err(_) => Ok(false),
                }
                // Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}