use prisma_value::PrismaValue;
use serde_json::json;

pub fn enclose(input: &str, with: &str) -> String {
    format!("{}{}{}", with, input, with)
}

pub fn enclose_all<T>(input: Vec<T>, with: &str) -> Vec<String>
where
    T: AsRef<str>,
{
    input.into_iter().map(|el| enclose(el.as_ref(), with)).collect()
}

pub fn stringify<T>(input: Vec<T>) -> Vec<String>
where
    T: ToString,
{
    input.iter().map(ToString::to_string).collect()
}

pub const TROUBLE_CHARS: &str = "ยฅเธฟ๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐ ๐ก๐ข๐ฃ๐ค๐ฅ๐ฆ๐ง๐จ๐ฉ๐ช๐ซ๐ฌ๐ญ๐ฎ๐ฏ๐ฐ๐ฑ๐ฒ๐ณ๐ด๐ต๐ถ๐ท๐ธ๐น๐บ๐ป๐ผ๐ฝ๐พ๐ฟ๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐๐เคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเคเค เคกเคขเคฃเคคเคฅเคฆเคงเคจเคฉเคชเคซเคฌเคญเคฎเคฏเคฐโฌโญโฎโฏโฐโฑโฒโณโดโตโถโทโธโนโบโปโผโฝโพโฟโ";

pub fn fmt_query_raw(query: &str, params: Vec<PrismaValue>) -> String {
    let params: Vec<serde_json::Value> = params
        .into_iter()
        .map(serde_json::to_value)
        .collect::<std::result::Result<_, _>>()
        .unwrap();

    let params = serde_json::to_string(&params).unwrap();

    format!(
        r#"mutation {{ queryRaw(query: "{}", parameters: "{}") }}"#,
        query.replace("\"", "\\\""),
        params.replace("\"", "\\\"")
    )
}

pub fn fmt_execute_raw(query: &str, params: Vec<PrismaValue>) -> String {
    let params: Vec<serde_json::Value> = params
        .into_iter()
        .map(|v| match v {
            PrismaValue::DateTime(dt) => json!({
                "prisma__type": "date",
                "prisma__value": dt.to_rfc3339(),
            }),
            _ => serde_json::to_value(v).unwrap(),
        })
        .collect();

    let params = serde_json::to_string(&params).unwrap();

    format!(
        r#"mutation {{ executeRaw(query: "{}", parameters: "{}") }}"#,
        query.replace("\"", "\\\""),
        params.replace("\"", "\\\"")
    )
}
