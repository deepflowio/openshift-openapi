// Generated from definition com.github.openshift.api.template.v1.BrokerTemplateInstance

/// BrokerTemplateInstance holds the service broker-related state associated with a TemplateInstance.  BrokerTemplateInstance is part of an experimental API.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BrokerTemplateInstance {
    /// Standard object metadata.
    pub metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// spec describes the state of this BrokerTemplateInstance.
    pub spec: crate::api::template::v1::BrokerTemplateInstanceSpec,
}

// Begin template.openshift.io/v1/BrokerTemplateInstance

// Generated from operation createTemplateOpenshiftIoV1BrokerTemplateInstance

impl BrokerTemplateInstance {
    /// create a BrokerTemplateInstance
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::CreateResponse`]`<Self>>` constructor, or [`k8s_openapi::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_broker_template_instance(
        body: &crate::api::template::v1::BrokerTemplateInstance,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/template.openshift.io/v1/brokertemplateinstances?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteTemplateOpenshiftIoV1BrokerTemplateInstance

impl BrokerTemplateInstance {
    /// delete a BrokerTemplateInstance
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the BrokerTemplateInstance
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_broker_template_instance(
        name: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/template.openshift.io/v1/brokertemplateinstances/{name}",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&optional).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteTemplateOpenshiftIoV1CollectionBrokerTemplateInstance

impl BrokerTemplateInstance {
    /// delete collection of BrokerTemplateInstance
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<`[`k8s_openapi::List`]`<Self>>>` constructor, or [`k8s_openapi::DeleteResponse`]`<`[`k8s_openapi::List`]`<Self>>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection_broker_template_instance(
        delete_optional: k8s_openapi::DeleteOptional<'_>,
        list_optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<k8s_openapi::List<Self>>>), k8s_openapi::RequestError> {
        let __url = "/apis/template.openshift.io/v1/brokertemplateinstances?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&delete_optional).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation listTemplateOpenshiftIoV1BrokerTemplateInstance

impl BrokerTemplateInstance {
    /// list or watch objects of kind BrokerTemplateInstance
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ListResponse`]`<Self>>` constructor, or [`k8s_openapi::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_broker_template_instance(
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/template.openshift.io/v1/brokertemplateinstances?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchTemplateOpenshiftIoV1BrokerTemplateInstance

impl BrokerTemplateInstance {
    /// partially update the specified BrokerTemplateInstance
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the BrokerTemplateInstance
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_broker_template_instance(
        name: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: k8s_openapi::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/template.openshift.io/v1/brokertemplateinstances/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match body {
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation readTemplateOpenshiftIoV1BrokerTemplateInstance

impl BrokerTemplateInstance {
    /// read the specified BrokerTemplateInstance
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadBrokerTemplateInstanceResponse`]`>` constructor, or [`ReadBrokerTemplateInstanceResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the BrokerTemplateInstance
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_broker_template_instance(
        name: &str,
        optional: ReadBrokerTemplateInstanceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadBrokerTemplateInstanceResponse>), k8s_openapi::RequestError> {
        let ReadBrokerTemplateInstanceOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/template.openshift.io/v1/brokertemplateinstances/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`BrokerTemplateInstance::read_broker_template_instance`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadBrokerTemplateInstanceOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadBrokerTemplateInstanceResponse as Response>::try_from_parts` to parse the HTTP response body of [`BrokerTemplateInstance::read_broker_template_instance`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadBrokerTemplateInstanceResponse {
    Ok(crate::api::template::v1::BrokerTemplateInstance),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadBrokerTemplateInstanceResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadBrokerTemplateInstanceResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadBrokerTemplateInstanceResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceTemplateOpenshiftIoV1BrokerTemplateInstance

impl BrokerTemplateInstance {
    /// replace the specified BrokerTemplateInstance
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the BrokerTemplateInstance
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_broker_template_instance(
        name: &str,
        body: &crate::api::template::v1::BrokerTemplateInstance,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>), k8s_openapi::RequestError> {
        let __url = format!("/apis/template.openshift.io/v1/brokertemplateinstances/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(name.as_bytes(), k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::put(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchTemplateOpenshiftIoV1BrokerTemplateInstance

impl BrokerTemplateInstance {
    /// list or watch objects of kind BrokerTemplateInstance
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::WatchResponse`]`<Self>>` constructor, or [`k8s_openapi::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_broker_template_instance(
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>), k8s_openapi::RequestError> {
        let __url = "/apis/template.openshift.io/v1/brokertemplateinstances?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// End template.openshift.io/v1/BrokerTemplateInstance

impl k8s_openapi::Resource for BrokerTemplateInstance {
    const API_VERSION: &'static str = "template.openshift.io/v1";
    const GROUP: &'static str = "template.openshift.io";
    const KIND: &'static str = "BrokerTemplateInstance";
    const VERSION: &'static str = "v1";
}

impl k8s_openapi::ListableResource for BrokerTemplateInstance {
    const LIST_KIND: &'static str = concat!("BrokerTemplateInstance", "List");
}

impl k8s_openapi::Metadata for BrokerTemplateInstance {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as k8s_openapi::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for BrokerTemplateInstance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BrokerTemplateInstance;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::api::template::v1::BrokerTemplateInstanceSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as k8s_openapi::Resource>::API_VERSION {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as k8s_openapi::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as k8s_openapi::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as k8s_openapi::Resource>::KIND));
                            }
                        },
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BrokerTemplateInstance {
                    metadata: value_metadata,
                    spec: value_spec.ok_or_else(|| serde::de::Error::missing_field("spec"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BrokerTemplateInstance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            3 +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        serde::ser::SerializeStruct::end(state)
    }
}