use crate::{routes::Id, AstelResource};
use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    Serialize,
};
use std::{collections::HashMap, fmt::Display};

pub(crate) fn to_table<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    t: &[T],
    path: &str,
) -> String {
    let headers = struct_fields::<T>()
        .iter()
        .map(|s| wrap("th", s))
        .collect::<String>();
    let headers = wrap("tr", wrap("th", "edit") + &wrap("th", "delete") + &headers);

    let values = t
        .iter()
        .map(|t| {
            // TODO something less wasteful
            let s = serde_json::to_string(t).unwrap();
            let map: HashMap<String, serde_json::Value> = serde_json::from_str(&s).unwrap();

            // use struct_fields as sorting
            let v = struct_fields::<T>()
                .iter()
                .map(|k| {
                    let v = map.get(*k).unwrap();
                    wrap("td", v)
                })
                .collect::<String>();

            // NOTE: i think this is wrong cause this is &ID, which might have a different serialization from ID
            let id = &serde_urlencoded::to_string(Id { id: t.id() }).unwrap();
            let edit = wrap("td", link("edit", format!(".{path}/edit?{id}")));
            let delete = wrap("td", link("delete", format!(".{path}/delete?{id}")));
            wrap("tr", edit + &delete + &v)
        })
        .collect::<String>();

    wrap("table", headers + &values)
}

fn wrap(c: &str, d: impl Display) -> String {
    format!("<{c}>{d}</{c}>")
}
fn link(d: impl Display, link: impl Display) -> String {
    format!("<a href=\"{link}\">{d}</a>")
}

pub fn struct_fields<'de, T>() -> &'static [&'static str]
where
    T: Deserialize<'de>,
{
    struct StructFieldsDeserializer<'a> {
        fields: &'a mut Option<&'static [&'static str]>,
    }

    impl<'de, 'a> Deserializer<'de> for StructFieldsDeserializer<'a> {
        type Error = serde::de::value::Error;

        fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            Err(de::Error::custom("I'm just here for the fields"))
        }

        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            *self.fields = Some(fields);
            self.deserialize_any(visitor)
        }

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
            byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map enum identifier ignored_any
        }
    }

    let mut fields = None;
    let _ = T::deserialize(StructFieldsDeserializer {
        fields: &mut fields,
    });
    fields.unwrap()
}
