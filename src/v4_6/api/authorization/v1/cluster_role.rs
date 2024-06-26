// Generated from definition com.github.openshift.api.authorization.v1.ClusterRole

/// ClusterRole is a logical grouping of PolicyRules that can be referenced as a unit by ClusterRoleBindings.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterRole {
    /// AggregationRule is an optional field that describes how to build the Rules for this ClusterRole. If AggregationRule is set, then the Rules are controller managed and direct changes to Rules will be stomped by the controller.
    pub aggregation_rule: Option<k8s_openapi::api::rbac::v1::AggregationRule>,

    /// Standard object's metadata.
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Rules holds all the PolicyRules for this ClusterRole
    pub rules: Vec<crate::api::authorization::v1::PolicyRule>,
}

// Begin authorization.openshift.io/v1/ClusterRole

// Generated from operation createAuthorizationOpenshiftIoV1ClusterRole

impl ClusterRole {
    /// create a ClusterRole
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
    pub fn create_cluster_role(
        body: &crate::api::authorization::v1::ClusterRole,
        optional: k8s_openapi::CreateOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::CreateResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/authorization.openshift.io/v1/clusterroles?".to_owned();
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteAuthorizationOpenshiftIoV1ClusterRole

impl ClusterRole {
    /// delete a ClusterRole
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::DeleteResponse`]`<Self>>` constructor, or [`k8s_openapi::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_cluster_role(
        name: &str,
        optional: k8s_openapi::DeleteOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::DeleteResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = format!(
            "/apis/authorization.openshift.io/v1/clusterroles/{name}",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
        );

        let __request = http::Request::delete(__url);
        let __body = serde_json::to_vec(&optional).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation listAuthorizationOpenshiftIoV1ClusterRole

impl ClusterRole {
    /// list objects of kind ClusterRole
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
    pub fn list_cluster_role(
        optional: k8s_openapi::ListOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ListResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/authorization.openshift.io/v1/clusterroles?".to_owned();
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

// Generated from operation patchAuthorizationOpenshiftIoV1ClusterRole

impl ClusterRole {
    /// partially update the specified ClusterRole
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::PatchResponse`]`<Self>>` constructor, or [`k8s_openapi::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_cluster_role(
        name: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: k8s_openapi::PatchOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::PatchResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = format!(
            "/apis/authorization.openshift.io/v1/clusterroles/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static(match body {
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => {
                    "application/json-patch+json"
                }
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => {
                    "application/merge-patch+json"
                }
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => {
                    "application/strategic-merge-patch+json"
                }
            }),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation readAuthorizationOpenshiftIoV1ClusterRole

impl ClusterRole {
    /// read the specified ClusterRole
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`ReadClusterRoleResponse`]`>` constructor, or [`ReadClusterRoleResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_cluster_role(
        name: &str,
        optional: ReadClusterRoleOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<ReadClusterRoleResponse>,
        ),
        k8s_openapi::RequestError,
    > {
        let ReadClusterRoleOptional { pretty } = optional;
        let __url = format!(
            "/apis/authorization.openshift.io/v1/clusterroles/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
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

/// Optional parameters of [`ClusterRole::read_cluster_role`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadClusterRoleOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadClusterRoleResponse as Response>::try_from_parts` to parse the HTTP response body of [`ClusterRole::read_cluster_role`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadClusterRoleResponse {
    Ok(crate::api::authorization::v1::ClusterRole),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl k8s_openapi::Response for ReadClusterRoleResponse {
    fn try_from_parts(
        status_code: http::StatusCode,
        buf: &[u8],
    ) -> Result<(Self, usize), k8s_openapi::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => {
                        return Err(k8s_openapi::ResponseError::NeedMoreData)
                    }
                    Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
                };
                Ok((ReadClusterRoleResponse::Ok(result), buf.len()))
            }
            _ => {
                let (result, read) = if buf.is_empty() {
                    (Ok(None), 0)
                } else {
                    match serde_json::from_slice(buf) {
                        Ok(value) => (Ok(Some(value)), buf.len()),
                        Err(ref err) if err.is_eof() => {
                            return Err(k8s_openapi::ResponseError::NeedMoreData)
                        }
                        Err(err) => (Err(err), 0),
                    }
                };
                Ok((ReadClusterRoleResponse::Other(result), read))
            }
        }
    }
}

// Generated from operation replaceAuthorizationOpenshiftIoV1ClusterRole

impl ClusterRole {
    /// replace the specified ClusterRole
    ///
    /// Use the returned [`k8s_openapi::ResponseBody`]`<`[`k8s_openapi::ReplaceResponse`]`<Self>>` constructor, or [`k8s_openapi::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRole
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_cluster_role(
        name: &str,
        body: &crate::api::authorization::v1::ClusterRole,
        optional: k8s_openapi::ReplaceOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::ReplaceResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = format!(
            "/apis/authorization.openshift.io/v1/clusterroles/{name}?",
            name = k8s_openapi::percent_encoding::percent_encode(
                name.as_bytes(),
                k8s_openapi::percent_encoding2::PATH_SEGMENT_ENCODE_SET
            ),
        );
        let mut __query_pairs = k8s_openapi::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::put(__url);
        let __body = serde_json::to_vec(body).map_err(k8s_openapi::RequestError::Json)?;
        let __request = __request.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/json"),
        );
        match __request.body(__body) {
            Ok(request) => Ok((request, k8s_openapi::ResponseBody::new)),
            Err(err) => Err(k8s_openapi::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchAuthorizationOpenshiftIoV1ClusterRole

impl ClusterRole {
    /// list objects of kind ClusterRole
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
    pub fn watch_cluster_role(
        optional: k8s_openapi::WatchOptional<'_>,
    ) -> Result<
        (
            http::Request<Vec<u8>>,
            fn(http::StatusCode) -> k8s_openapi::ResponseBody<k8s_openapi::WatchResponse<Self>>,
        ),
        k8s_openapi::RequestError,
    > {
        let __url = "/apis/authorization.openshift.io/v1/clusterroles?".to_owned();
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

// End authorization.openshift.io/v1/ClusterRole

impl k8s_openapi::Resource for ClusterRole {
    const API_VERSION: &'static str = "authorization.openshift.io/v1";
    const GROUP: &'static str = "authorization.openshift.io";
    const KIND: &'static str = "ClusterRole";
    const VERSION: &'static str = "v1";
    // fixed `Resource` impl
    const URL_PATH_SEGMENT: &'static str = "clusterroles";
    type Scope = k8s_openapi::ClusterResourceScope;
}

impl k8s_openapi::ListableResource for ClusterRole {
    const LIST_KIND: &'static str = concat!("ClusterRole", "List");
}

impl k8s_openapi::Metadata for ClusterRole {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> serde::Deserialize<'de> for ClusterRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_aggregation_rule,
            Key_metadata,
            Key_rules,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "aggregationRule" => Field::Key_aggregation_rule,
                            "metadata" => Field::Key_metadata,
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterRole;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value_aggregation_rule: Option<
                    k8s_openapi::api::rbac::v1::AggregationRule,
                > = None;
                let mut value_metadata: Option<
                    k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                > = None;
                let mut value_rules: Option<Vec<crate::api::authorization::v1::PolicyRule>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String =
                                serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version
                                != <Self::Value as k8s_openapi::Resource>::API_VERSION
                            {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value_api_version),
                                    &<Self::Value as k8s_openapi::Resource>::API_VERSION,
                                ));
                            }
                        }
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as k8s_openapi::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value_kind),
                                    &<Self::Value as k8s_openapi::Resource>::KIND,
                                ));
                            }
                        }
                        Field::Key_aggregation_rule => {
                            value_aggregation_rule = serde::de::MapAccess::next_value(&mut map)?
                        }
                        Field::Key_metadata => {
                            value_metadata = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Key_rules => {
                            value_rules = Some(serde::de::MapAccess::next_value(&mut map)?)
                        }
                        Field::Other => {
                            let _: serde::de::IgnoredAny =
                                serde::de::MapAccess::next_value(&mut map)?;
                        }
                    }
                }

                Ok(ClusterRole {
                    aggregation_rule: value_aggregation_rule,
                    metadata: value_metadata
                        .ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    rules: value_rules.ok_or_else(|| serde::de::Error::missing_field("rules"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &["apiVersion", "kind", "aggregationRule", "metadata", "rules"],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            4 + self.aggregation_rule.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "apiVersion",
            <Self as k8s_openapi::Resource>::API_VERSION,
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut state,
            "kind",
            <Self as k8s_openapi::Resource>::KIND,
        )?;
        if let Some(value) = &self.aggregation_rule {
            serde::ser::SerializeStruct::serialize_field(&mut state, "aggregationRule", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        serde::ser::SerializeStruct::end(state)
    }
}
