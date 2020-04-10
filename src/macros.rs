/// Automatically generate From impls for types given using a small DSL like
/// macrot
macro_rules! from {
    ($(@$f: ident
     $( ?> $i1: ident = $e1: tt )*
     $( => $t: ident )*
     $( -> $i2: ident = $e2: tt )* )*) => (
        $($(
        impl <'g> From<$f<'g>> for $t<'g> {
            fn from(f: $f<'g>) -> Self {
                Self {
                    request: f.request,
                    core: f.core,
                    client: f.client,
                    parameter: None,
                }
            }
        }
        )*$(
        impl <'g> From<$f<'g>> for $i1<'g> {
            fn from(mut f: $f<'g>) -> Self {
                use std::str::FromStr;

                // This is borrow checking abuse and about the only
                // time I'd do is_ok(). Essentially this allows us
                // to either pass the error message along or update
                // the url
                if f.request.is_ok() {
                    // We've checked that this works
                    let mut req = f.request.unwrap();
                    let url = f.parameter
                        .ok_or("Expecting parameter".into())
                        .and_then(|param| {
                            let sep =
                                if req
                                    .get_mut()
                                    .uri()
                                    .query()
                                    .is_some() { "&" } else { "?" };
                            hyper::Uri::from_str(
                                &format!("{}{}{}={}",
                                    req.get_mut().uri(),
                                    sep,
                                    $e1,
                                    param
                                )
                            ).chain_err(|| "Failed to parse Url")
                        });
                    match url {
                        Ok(u) => {
                            *req.get_mut().uri_mut() = u;
                            f.request = Ok(req);
                        },
                        Err(e) => {
                            f.request = Err(e);
                        }
                    }

                    Self {
                        request: f.request,
                        core: f.core,
                        client: f.client,
                        parameter: None,
                    }

                } else {

                    Self {
                        request: f.request,
                        core: f.core,
                        client: f.client,
                        parameter: None,
                    }

                }
            }
        }
        )*$(
        impl <'g> From<$f<'g>> for $i2<'g> {
            fn from(mut f: $f<'g>) -> Self {
                // This is borrow checking abuse and about the only
                // time I'd do is_ok(). Essentially this allows us
                // to either pass the error message along or update
                // the url
                if f.request.is_ok() {
                    // We've checked that this works
                    let mut req = f.request.unwrap();
                    let url = url_join(req.borrow().uri(), $e2);
                    match url {
                        Ok(u) => {
                            *req.get_mut().uri_mut() = u;
                            f.request = Ok(req);
                        },
                        Err(e) => {
                            f.request = Err(e.into());
                        }
                    }

                    Self {
                        request: f.request,
                        core: f.core,
                        client: f.client,
                        parameter: None,
                    }

                } else {

                    Self {
                        request: f.request,
                        core: f.core,
                        client: f.client,
                        parameter: None,
                    }

                }
            }
        }
        )*)*
    );
    ($(@$t: ident => $p: expr)*) => (
        $(
        impl <'g> From<&'g Github> for $t<'g> {
            fn from(gh: &'g Github) -> Self {
                use hyper::header::{ ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT };
                let res = Request::builder().method($p)
                    .uri("https://api.github.com")
                    .body(hyper::Body::empty())
                    .map_err(From::from)
                    .and_then(|req| {
                        let token = String::from("token ") + &gh.token;
                        HeaderValue::from_str(&token).map(|token| (req, token))
                            .map_err(From::from)
                    });
                match res {
                    Ok((mut req, token)) => {
                        {
                            let headers = req.headers_mut();
                            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                            headers.insert(USER_AGENT, HeaderValue::from_static("github-rs"));
                            headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
                            headers.insert(AUTHORIZATION, token);
                        }
                        Self {
                            request: Ok(RefCell::new(req)),
                            core: &gh.core,
                            client: &gh.client,
                            parameter: None,
                        }
                    }
                    Err(err) => {
                        Self {
                            request: Err(err),
                            core: &gh.core,
                            client: &gh.client,
                            parameter: None,
                        }
                    }
                }
            }
        }
    )*
    );
}

/// Used to identify a new type used in a query pipeline. The types are
/// consistent between each one in terms of transforming one to another.
/// This helps reduce boiler plate code and makes it easy to expand and
/// maintain code in the future by simply adding a new field here if needed
macro_rules! new_type {
    ($($i: ident)*) => (
        $(
        pub struct $i<'g> {
            pub(crate) request: Result<RefCell<Request<Body>>>,
            pub(crate) core: &'g Rc<RefCell<Core>>,
            pub(crate) client: &'g Rc<Client<HttpsConnector>>,
            pub(crate) parameter: Option<String>,
        }
        )*
    );
}

/// Used to generate an execute function for a terminal type in a query
/// pipeline. If passed a type it creates the impl as well as it needs
/// no extra functions.
macro_rules! exec {
    ($t: ident) => {
        impl<'a> Executor for $t<'a> {
            /// Execute the query by sending the built up request to GitHub.
            /// The value returned is either an error or the Status Code and
            /// Json after it has been deserialized. Please take a look at
            /// the GitHub documentation to see what value you should receive
            /// back for good or bad requests.
            fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
            where
                T: DeserializeOwned,
            {
                let mut core_ref = self.core.try_borrow_mut()?;
                let client = self.client;
                let work = client.request(self.request?.into_inner()).and_then(|res| {
                    let header = res.headers().clone();
                    let status = res.status();
                    res.into_body()
                        .fold(Vec::new(), |mut v, chunk| {
                            v.extend(&chunk[..]);
                            ok::<_, hyper::Error>(v)
                        })
                        .map(move |chunks| {
                            if chunks.is_empty() {
                                Ok((header, status, None))
                            } else {
                                Ok((header, status, Some(serde_json::from_slice(&chunks)?)))
                            }
                        })
                });
                core_ref.run(work)?
            }
        }
    };
}

/// Using a small DSL like macro generate an impl for a given type
/// that creates all the functions to transition from one node type to another
macro_rules! impl_macro {
    ($(@$i: ident $(|=> $id1: ident -> $t1: ident)*|
     $(|=> $id2: ident -> $t2: ident = $e2: ident)*
     $(|?> $id3: ident -> $t3: ident = $e3: ident)*
     $(|...> $id4: ident -> $t4: ident = $e4: ident)*)+
    )=> (
        $(
            impl<'g> $i <'g>{
            $(
                pub fn $id1(self) -> $t1<'g> {
                    self.into()
                }
            )*$(
                pub fn $id2(mut self, $e2: &str) -> $t2<'g> {
                    // This is borrow checking abuse and about the only
                    // time I'd do is_ok(). Essentially this allows us
                    // to either pass the error message along or update
                    // the url
                    if self.request.is_ok() {
                        // We've checked that this works
                        let mut req = self.request.unwrap();
                        let url = url_join(req.borrow().uri(), $e2);
                        match url {
                            Ok(u) => {
                                *req.get_mut().uri_mut() = u;
                                self.request = Ok(req);
                            },
                            Err(e) => {
                                self.request = Err(e.into());
                            }
                        }
                    }
                    self.into()
                }
            )*$(
                pub fn $id3(mut self, $e3: &str) -> $t3<'g> {
                    self.parameter = Some($e3.to_string());
                    self.into()
                }
            )*$(
                pub fn $id4(mut self, $e4: &str) -> $t4<'g> {
                    // This is borrow checking abuse and about the only
                    // time I'd do is_ok(). Essentially this allows us
                    // to either pass the error message along or update
                    // the url
                    if self.request.is_ok() {
                        use std::str::FromStr;
                        // We've checked that this works
                        let mut req = self.request.unwrap();
                        let url = url_join_ellipsis(req.get_mut().uri(), $e4);
                        match url {
                            Ok(u) => {
                                *req.get_mut().uri_mut() = u;
                                self.request = Ok(req);
                            },
                            Err(e) => {
                                self.request = Err(e.into());
                            }
                        }
                    }
                    self.into()
                }
            )*
            }
        )+
    );
}

/// A variation of `impl_macro` for the client module that allows partitioning of
/// types. Create a function with a given name and return type. Used for
/// creating functions for simple conversions from one type to another, where
/// the actual conversion code is in the From implementation.
macro_rules! func_client{
    ($i: ident, $t: ty) => (
        pub fn $i(self) -> $t {
            self.into()
        }
    );
    ($i: ident, $t: ident, $e: ident) => (
        pub fn $i(mut self, $e: &str) -> $t<'g> {
            // This is borrow checking abuse and about the only
            // time I'd do is_ok(). Essentially this allows us
            // to either pass the error message along or update
            // the url
            if self.request.is_ok() {
                // We've checked that this works
                let mut req = self.request.unwrap();
                let url = url_join(req.borrow().uri(), $e);
                match url {
                    Ok(u) => {
                        *req.get_mut().uri_mut() = u;
                        self.request = Ok(req);
                    },
                    Err(e) => {
                        self.request = Err(e.into());
                    }
                }
            }
            self.into()
        }
    );
}

/// Common imports for every file
macro_rules! imports {
    () => {
        use tokio_core::reactor::Core;
        #[cfg(feature = "rustls")]
        type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
        #[cfg(feature = "rust-native-tls")]
        use hyper_tls;
        #[cfg(feature = "rust-native-tls")]
        type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;
        use crate::errors::*;
        use crate::util::url_join;
        use crate::util::url_join_ellipsis;
        use futures::future::ok;
        use futures::{Future, Stream};
        use hyper::client::Client;
        use hyper::Request;
        use hyper::StatusCode;
        use hyper::{self, Body, HeaderMap};
        use serde::de::DeserializeOwned;
        use serde_json;
        use std::cell::RefCell;
        use std::rc::Rc;

        use $crate::client::Executor;
    };
}
