// Generated from definition com.github.openshift.api.build.v1.StageInfo

/// StageInfo contains details about a build stage.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StageInfo {
    /// durationMilliseconds identifies how long the stage took to complete in milliseconds. Note: the duration of a stage can exceed the sum of the duration of the steps within the stage as not all actions are accounted for in explicit build steps.
    pub duration_milliseconds: Option<i64>,

    /// name is a unique identifier for each build stage that occurs.
    pub name: Option<String>,

    /// startTime is a timestamp representing the server time when this Stage started. It is represented in RFC3339 form and is in UTC.
    pub start_time: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time>,

    /// steps contains details about each step that occurs during a build stage including start time and duration in milliseconds.
    pub steps: Option<Vec<crate::api::build::v1::StepInfo>>,
}

impl<'de> serde::Deserialize<'de> for StageInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_duration_milliseconds,
            Key_name,
            Key_start_time,
            Key_steps,
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
                            "durationMilliseconds" => Field::Key_duration_milliseconds,
                            "name" => Field::Key_name,
                            "startTime" => Field::Key_start_time,
                            "steps" => Field::Key_steps,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StageInfo;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StageInfo")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_duration_milliseconds: Option<i64> = None;
                let mut value_name: Option<String> = None;
                let mut value_start_time: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_steps: Option<Vec<crate::api::build::v1::StepInfo>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_duration_milliseconds => value_duration_milliseconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_time => value_start_time = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_steps => value_steps = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StageInfo {
                    duration_milliseconds: value_duration_milliseconds,
                    name: value_name,
                    start_time: value_start_time,
                    steps: value_steps,
                })
            }
        }

        deserializer.deserialize_struct(
            "StageInfo",
            &[
                "durationMilliseconds",
                "name",
                "startTime",
                "steps",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for StageInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StageInfo",
            self.duration_milliseconds.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.start_time.as_ref().map_or(0, |_| 1) +
            self.steps.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.duration_milliseconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "durationMilliseconds", value)?;
        }
        if let Some(value) = &self.name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.start_time {
            serde::ser::SerializeStruct::serialize_field(&mut state, "startTime", value)?;
        }
        if let Some(value) = &self.steps {
            serde::ser::SerializeStruct::serialize_field(&mut state, "steps", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
