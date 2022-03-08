// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *drive* crate version *3.0.0+20220225*, where *20220225* is the exact revision of the *drive:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v3.0.0*.
//! 
//! Everything else about the *drive* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/drive/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/drive2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](DriveHub) ... 
//! 
//! * [about](api::About)
//!  * [*get*](api::AboutGetCall)
//! * [apps](api::App)
//!  * [*get*](api::AppGetCall) and [*list*](api::AppListCall)
//! * [changes](api::Change)
//!  * [*get*](api::ChangeGetCall), [*get start page token*](api::ChangeGetStartPageTokenCall), [*list*](api::ChangeListCall) and [*watch*](api::ChangeWatchCall)
//! * [channels](api::Channel)
//!  * [*stop*](api::ChannelStopCall)
//! * children
//!  * [*delete*](api::ChildrenDeleteCall), [*get*](api::ChildrenGetCall), [*insert*](api::ChildrenInsertCall) and [*list*](api::ChildrenListCall)
//! * [comments](api::Comment)
//!  * [*delete*](api::CommentDeleteCall), [*get*](api::CommentGetCall), [*insert*](api::CommentInsertCall), [*list*](api::CommentListCall), [*patch*](api::CommentPatchCall) and [*update*](api::CommentUpdateCall)
//! * [drives](api::Drive)
//!  * [*delete*](api::DriveDeleteCall), [*get*](api::DriveGetCall), [*hide*](api::DriveHideCall), [*insert*](api::DriveInsertCall), [*list*](api::DriveListCall), [*unhide*](api::DriveUnhideCall) and [*update*](api::DriveUpdateCall)
//! * [files](api::File)
//!  * [*copy*](api::FileCopyCall), [*delete*](api::FileDeleteCall), [*empty trash*](api::FileEmptyTrashCall), [*export*](api::FileExportCall), [*generate ids*](api::FileGenerateIdCall), [*get*](api::FileGetCall), [*insert*](api::FileInsertCall), [*list*](api::FileListCall), [*patch*](api::FilePatchCall), [*touch*](api::FileTouchCall), [*trash*](api::FileTrashCall), [*untrash*](api::FileUntrashCall), [*update*](api::FileUpdateCall) and [*watch*](api::FileWatchCall)
//! * parents
//!  * [*delete*](api::ParentDeleteCall), [*get*](api::ParentGetCall), [*insert*](api::ParentInsertCall) and [*list*](api::ParentListCall)
//! * [permissions](api::Permission)
//!  * [*delete*](api::PermissionDeleteCall), [*get*](api::PermissionGetCall), [*get id for email*](api::PermissionGetIdForEmailCall), [*insert*](api::PermissionInsertCall), [*list*](api::PermissionListCall), [*patch*](api::PermissionPatchCall) and [*update*](api::PermissionUpdateCall)
//! * [properties](api::Property)
//!  * [*delete*](api::PropertyDeleteCall), [*get*](api::PropertyGetCall), [*insert*](api::PropertyInsertCall), [*list*](api::PropertyListCall), [*patch*](api::PropertyPatchCall) and [*update*](api::PropertyUpdateCall)
//! * replies
//!  * [*delete*](api::ReplyDeleteCall), [*get*](api::ReplyGetCall), [*insert*](api::ReplyInsertCall), [*list*](api::ReplyListCall), [*patch*](api::ReplyPatchCall) and [*update*](api::ReplyUpdateCall)
//! * [revisions](api::Revision)
//!  * [*delete*](api::RevisionDeleteCall), [*get*](api::RevisionGetCall), [*list*](api::RevisionListCall), [*patch*](api::RevisionPatchCall) and [*update*](api::RevisionUpdateCall)
//! * teamdrives
//!  * [*delete*](api::TeamdriveDeleteCall), [*get*](api::TeamdriveGetCall), [*insert*](api::TeamdriveInsertCall), [*list*](api::TeamdriveListCall) and [*update*](api::TeamdriveUpdateCall)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*insert files*](api::FileInsertCall)
//! * [*update files*](api::FileUpdateCall)
//! 
//! Download supported by ...
//! 
//! * [*export files*](api::FileExportCall)
//! * [*get files*](api::FileGetCall)
//! * [*watch files*](api::FileWatchCall)
//! 
//! Subscription supported by ...
//! 
//! * [*list changes*](api::ChangeListCall)
//! * [*watch changes*](api::ChangeWatchCall)
//! * [*get files*](api::FileGetCall)
//! * [*insert files*](api::FileInsertCall)
//! * [*watch files*](api::FileWatchCall)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](DriveHub)**
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
//! let r = hub.files().copy(...).doit().await
//! let r = hub.files().delete(...).doit().await
//! let r = hub.files().empty_trash(...).doit().await
//! let r = hub.files().export(...).doit().await
//! let r = hub.files().generate_ids(...).doit().await
//! let r = hub.files().get(...).doit().await
//! let r = hub.files().insert(...).doit().await
//! let r = hub.files().list(...).doit().await
//! let r = hub.files().patch(...).doit().await
//! let r = hub.files().touch(...).doit().await
//! let r = hub.files().trash(...).doit().await
//! let r = hub.files().untrash(...).doit().await
//! let r = hub.files().update(...).doit().await
//! let r = hub.files().watch(...).doit().await
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
//! google-drive2 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_drive2 as drive2;
//! use drive2::api::File;
//! use drive2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use drive2::{DriveHub, oauth2, hyper, hyper_rustls};
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
//! let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = File::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.files().patch(req, "fileId")
//!              .use_content_as_indexable_text(true)
//!              .update_viewed_date(true)
//!              .timed_text_track_name("ipsum")
//!              .timed_text_language("est")
//!              .supports_team_drives(true)
//!              .supports_all_drives(false)
//!              .set_modified_date(true)
//!              .remove_parents("eos")
//!              .pinned(false)
//!              .ocr_language("sed")
//!              .ocr(false)
//!              .new_revision(false)
//!              .modified_date_behavior("no")
//!              .include_permissions_for_view("Stet")
//!              .enforce_single_parent(true)
//!              .convert(true)
//!              .add_parents("vero")
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
pub use api::DriveHub;
pub use client::{Result, Error, Delegate};
