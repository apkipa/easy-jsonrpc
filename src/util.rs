use crate::serde_json;

#[doc(hidden)]
pub fn from_serde_json_value_ref<'de, T>(value: &'de serde_json::Value) -> Result<T, serde_json::Error>
where
    T: serde::de::Deserialize<'de>,
{
    T::deserialize(value)
}
