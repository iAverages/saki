import { DMMF } from "@prisma/generator-helper";
import { toPascelCase, toSnakeCase } from "../utils/stringFormats";

// BigInt, Boolean, Bytes, DateTime, Decimal, Float, Int, JSON, String, $ModelName
const convertToRustType = (fieldType: DMMF.Field["type"]) => {
  switch (fieldType) {
    case "String":
      return "String";
    case "BigInt":
      return "i64";
    case "Boolean":
      return "booln";
    case "Bytes":
      return "Vec<u8>";
    case "DateTime":
      return "sqlx::types::chrono::NaiveDateTime";
    case "Decimal":
      return "TODO:";
    case "Int":
      return "i32";
    case "JSON":
      return "TODO:";
    default:
      return null;
  }
};

export const genModel = ({ name, fields }: DMMF.Model) => {
  const fieldRows = fields.map((field) => {
    return `    pub ${toSnakeCase(field.name)}: ${convertToRustType(
      field.type
    )},`;
  });

  return `#[derive(sqlx::FromRow)]
pub struct DB${toPascelCase(name)} {
${fieldRows.join("\n")}
}`;
};
