// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Bigtable Admin* crate version *3.0.0+20220222*, where *20220222* is the exact revision of the *bigtableadmin:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v3.0.0*.
//! 
//! Everything else about the *Bigtable Admin* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/bigtable/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/bigtableadmin2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](BigtableAdmin) ... 
//! 
//! * [operations](api::Operation)
//!  * [*cancel*](api::OperationCancelCall), [*delete*](api::OperationDeleteCall), [*get*](api::OperationGetCall) and [*projects operations list*](api::OperationProjectOperationListCall)
//! * projects
//!  * [*instances app profiles create*](api::ProjectInstanceAppProfileCreateCall), [*instances app profiles delete*](api::ProjectInstanceAppProfileDeleteCall), [*instances app profiles get*](api::ProjectInstanceAppProfileGetCall), [*instances app profiles list*](api::ProjectInstanceAppProfileListCall), [*instances app profiles patch*](api::ProjectInstanceAppProfilePatchCall), [*instances clusters backups create*](api::ProjectInstanceClusterBackupCreateCall), [*instances clusters backups delete*](api::ProjectInstanceClusterBackupDeleteCall), [*instances clusters backups get*](api::ProjectInstanceClusterBackupGetCall), [*instances clusters backups get iam policy*](api::ProjectInstanceClusterBackupGetIamPolicyCall), [*instances clusters backups list*](api::ProjectInstanceClusterBackupListCall), [*instances clusters backups patch*](api::ProjectInstanceClusterBackupPatchCall), [*instances clusters backups set iam policy*](api::ProjectInstanceClusterBackupSetIamPolicyCall), [*instances clusters backups test iam permissions*](api::ProjectInstanceClusterBackupTestIamPermissionCall), [*instances clusters create*](api::ProjectInstanceClusterCreateCall), [*instances clusters delete*](api::ProjectInstanceClusterDeleteCall), [*instances clusters get*](api::ProjectInstanceClusterGetCall), [*instances clusters list*](api::ProjectInstanceClusterListCall), [*instances clusters partial update cluster*](api::ProjectInstanceClusterPartialUpdateClusterCall), [*instances clusters update*](api::ProjectInstanceClusterUpdateCall), [*instances create*](api::ProjectInstanceCreateCall), [*instances delete*](api::ProjectInstanceDeleteCall), [*instances get*](api::ProjectInstanceGetCall), [*instances get iam policy*](api::ProjectInstanceGetIamPolicyCall), [*instances list*](api::ProjectInstanceListCall), [*instances partial update instance*](api::ProjectInstancePartialUpdateInstanceCall), [*instances set iam policy*](api::ProjectInstanceSetIamPolicyCall), [*instances tables check consistency*](api::ProjectInstanceTableCheckConsistencyCall), [*instances tables create*](api::ProjectInstanceTableCreateCall), [*instances tables delete*](api::ProjectInstanceTableDeleteCall), [*instances tables drop row range*](api::ProjectInstanceTableDropRowRangeCall), [*instances tables generate consistency token*](api::ProjectInstanceTableGenerateConsistencyTokenCall), [*instances tables get*](api::ProjectInstanceTableGetCall), [*instances tables get iam policy*](api::ProjectInstanceTableGetIamPolicyCall), [*instances tables list*](api::ProjectInstanceTableListCall), [*instances tables modify column families*](api::ProjectInstanceTableModifyColumnFamilyCall), [*instances tables restore*](api::ProjectInstanceTableRestoreCall), [*instances tables set iam policy*](api::ProjectInstanceTableSetIamPolicyCall), [*instances tables test iam permissions*](api::ProjectInstanceTableTestIamPermissionCall), [*instances test iam permissions*](api::ProjectInstanceTestIamPermissionCall), [*instances update*](api::ProjectInstanceUpdateCall), [*locations get*](api::ProjectLocationGetCall) and [*locations list*](api::ProjectLocationListCall)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](BigtableAdmin)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.operations().projects_operations_list(...).doit().await
//! let r = hub.operations().cancel(...).doit().await
//! let r = hub.operations().delete(...).doit().await
//! let r = hub.operations().get(...).doit().await
//! let r = hub.projects().instances_app_profiles_patch(...).doit().await
//! let r = hub.projects().instances_clusters_backups_create(...).doit().await
//! let r = hub.projects().instances_clusters_create(...).doit().await
//! let r = hub.projects().instances_clusters_partial_update_cluster(...).doit().await
//! let r = hub.projects().instances_clusters_update(...).doit().await
//! let r = hub.projects().instances_tables_restore(...).doit().await
//! let r = hub.projects().instances_create(...).doit().await
//! let r = hub.projects().instances_partial_update_instance(...).doit().await
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-bigtableadmin2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_bigtableadmin2 as bigtableadmin2;
//! use bigtableadmin2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use bigtableadmin2::{BigtableAdmin, oauth2, hyper, hyper_rustls};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = BigtableAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.operations().projects_operations_list("name")
//!              .page_token("At")
//!              .page_size(-8)
//!              .filter("sed")
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub extern crate hyper;
pub extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
pub extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

pub mod api;
pub mod client;

// Re-export the hub type and some basic client structs
pub use api::BigtableAdmin;
pub use client::{Result, Error, Delegate};
