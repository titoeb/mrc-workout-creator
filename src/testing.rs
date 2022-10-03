use serde::de::DeserializeOwned;
use serde::Serialize;

/// Serialize an object to string via serde, then directly deserialize.
/// This can be used in testing to ensured the derived or implemented
/// traits `Serialize`  and `Deserialize` work as expected.
pub fn serialize_deserialize<Serializable>(object: &Serializable) -> Serializable
where
    Serializable: Serialize + DeserializeOwned,
{
    let string_representation: String = serde_json::to_string(object).unwrap();
    serde_json::from_str(&string_representation).unwrap()
}
