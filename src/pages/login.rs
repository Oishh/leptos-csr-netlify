use dotenvy_macro::dotenv;
use leptos::{html::Input, *};

use crate::{models::login::{DirectusLoginRequest, DirectusLoginResponse}, utilities::{cookies::{set_jabra_cookie, JabraCookie}, errors::JabraError, http_wrapper::{call_and_parse, HttpMethod}}, HasError, Refetcher};


#[allow(non_snake_case)]
#[component]
pub fn Login() -> impl IntoView {
    let url_env = dotenv!("DIRECTUSURL");
    view! {
        <div class="h-full lg:grid lg:grid-cols-3">
            <div class="h-full flex items-center justify-center px-4">
                <div class="card flex-shrink-0 w-full max-w-sm shadow-lg bg-base-100">
                    <div class="card-body">
                        <LoginIsland/>
                    </div>
                </div>
            </div>
            <div class="h-full lg:flex items-center hidden lg:col-span-2">
                <div class="flex flex-col">
                    <div class="flex items-center">
                        <h1 class="text-6xl font-base">
                            "Bespoke Structured Products" <br/> "for your"
                            <span class="font-bold">" digital assets"</span>
                        </h1>
                    </div>
                    <p class="mt-4">{url_env}</p>
                    <p class="mt-4">A tailored solution for your investment thesis.</p>
                </div>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn LoginIsland() -> impl IntoView {
    let login_has_error = create_rw_signal(false);
    let login_action = create_action(move |(userid, password): &(String, String)| {
        let userid_clone = userid.clone();
        let password_clone = password.clone();
        async move {
            let result = directus_login(userid_clone, password_clone).await;
            let navigate = leptos_router::use_navigate();

            match result {
                Ok(res) => {
                    if res {
                        use_context::<Refetcher>().unwrap().0.update(|s| *s = !*s);
                        navigate("/quotes/builder", Default::default());
                        true
                    } else {
                        login_has_error.set(true);
                        false
                    }
                }
                Err(_e) => {login_has_error.set(true);false},
            }
        }
    });
    // let login_action: Action<DirectusLogin, Result<bool, ServerFnError>> =
    //     create_server_action::<DirectusLogin>();
    let is_pending = login_action.pending();

    create_effect(move |_| {
        log::info!("Is_pending: {:?}", is_pending());

        let value = login_action.value();

        if let Some(data) = value.get() {
            match data {
                true => {
                    use_context::<Refetcher>().unwrap().0.set(true);
                    use_context::<HasError>().unwrap().0.set(false);
                }
                false => {
                    use_context::<Refetcher>().unwrap().0.set(false);
                    use_context::<HasError>().unwrap().0.set(true);
                }
            }
        }
    });

    let email_ref = create_node_ref::<Input>();
    let pass_ref = create_node_ref::<Input>();

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let userid = email_ref.get().expect("input to exist");
            let password = pass_ref.get().expect("input to exist");
            login_action.dispatch((userid.value(), password.value()));
        }>

            <label for="userid" class="label">
                <span class="label-text">Email</span>
            </label>
            <input
                type="text"
                name="userid"
                class="input input-sm w-full bg-white rounded hover:shadow-md text-black border-gray-800 shadow-md"
                autocomplete
                required
                node_ref=email_ref
            />

            <label for="password" class="label">
                <span class="label-text">Password</span>
            </label>
            <input
                type="password"
                name="password"
                class="input input-sm w-full bg-white rounded hover:shadow-md text-black border-gray-800 shadow-md"
                autocomplete
                required
                node_ref=pass_ref
            />
            <label class="label">
                <a href="#" class="label-text-alt link link-hover">
                    Forgot password?
                </a>
            </label>

            {move || match is_pending() {
                true => {
                    view! {
                        <div class="form-control mt-6">
                            <button type="submit" class="btn btn-block btn-success">
                                <span class="loading loading-spinner loading-sm"></span>
                            </button>
                        </div>
                    }
                        .into_any()
                }
                false => {
                    view! {
                        <div class="form-control mt-6">
                            <button type="submit" class="btn rounded btn-block btn-success">
                                LOGIN
                            </button>
                        </div>
                    }
                        .into_any()
                }
            }}

        </form>

        // <Show when=move || login_has_error.get()>
        //     <StatusModal
        //         signal=login_has_error
        //         title="ERROR!".to_string()
        //         description="Your email or password is not valid.".to_string()
        //         status=ComponentStatus::Error
        //         position=Position::TopMiddle
        //     />
        // </Show>
    }
}


pub async fn directus_login(userid: String, password: String) -> Result<bool, ServerFnError> {
    let url = dotenv!("DIRECTUSURL");
    let path = format!("{}/auth/login", url);
    let email = userid.clone();
    let login_request = DirectusLoginRequest::new(userid.into(), password.into());
    let response = call_and_parse::<DirectusLoginRequest, DirectusLoginResponse>(
        Some(login_request),
        path,
        reqwest::header::HeaderMap::new(),
        HttpMethod::POST,
    )
    .await;

    match response {
        Ok(res) => {
            // Calculate expiration time in millis, subract 2 minute to be safe
            // Why 10 minutes? There are other api resource that are automatically when users navigate to a certain page
            // Only those API calls in action will have the refresh token
            // Which means during the manual submit, the refresh token is used
            // 10 minutes will act as a buffer for those action

            let expiration_time =
                chrono::Utc::now().timestamp_millis() + res.data.expires - 600_000;

            let jabra_cookie = JabraCookie::new(
                email,
                res.data.access_token.clone(),
                res.data.refresh_token,
                expiration_time,
            );
            set_jabra_cookie(jabra_cookie, "admin_portal_csr".to_string());

            Ok(true)
        }
        Err(e) => {
            log::info!("Login Error: {}", e.to_string());
            Err(ServerFnError::ServerError(
                JabraError::LoginError.to_string(),
            ))
        }
    }
}