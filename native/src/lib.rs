use rustler::{Env, NifUnitEnum, Term, nif};

mod client;
mod req;
mod runtime;
mod utils;

mod atoms {
    rustler::atoms! {
        cancelled,
        additional_root_certs,
        basic_auth,
        body,
        connect_timeout,
        cookie_store,
        client_builder_error,
        danger_accept_invalid_certs,
        danger_accept_invalid_hostnames,
        erqwest_response,
        erqwest_runtime_stopped,
        error,
        follow_redirects,
        headers,
        https_only,
        identity,
        method,
        ok,
        pool_idle_timeout,
        pool_max_idle_per_host,
        proxy,
        reason,
        status,
        timeout,
        url,
        use_built_in_root_certs,
    }
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(client::ClientResource, env);
    rustler::resource!(req::AbortResource, env);
    rustler::resource!(runtime::RuntimeResource, env);
    true
}

#[derive(NifUnitEnum)]
enum Feature {
    Cookies,
}

#[nif]
fn feature(f: Feature) -> bool {
    use Feature::*;
    match f {
        Cookies => cfg!(feature = "cookies"),
    }
}

rustler::init!(
    "erqwest",
    [
        runtime::start_runtime,
        runtime::stop_runtime,
        client::make_client,
        client::close_client,
        req::req_async_internal,
        req::cancel,
        feature
    ],
    load = load
);
