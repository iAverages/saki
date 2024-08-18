"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.genModel = exports.genEnum = void 0;
const genEnum = ({ name, values }) => {
    const enumValues = values.map(({ name }) => `${name}="${name}"`).join(",\n");
    console.log(enumValues);
    return `enum ${name} { \n${enumValues}\n }`;
};
exports.genEnum = genEnum;
const toPascelCase = (name) => {
    return name
        .split(/\s+|_|-+/)
        .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
        .join("");
};
const toSnakeCase = (name) => {
    return name
        .split(/\s+|_|-+/)
        .map((word) => word.toLowerCase())
        .join("_");
};
const convertToRustType = (fieldType) => {
    switch (fieldType) {
        case "String":
            return "String";
        case "BigInt":
            return "u64";
        case "Boolean":
            return "booln";
        case "Bytes":
            return "Vec<u8>";
        case "DateTime":
            return "TODO:";
        case "Decimal":
            return "TODO:";
        case "Int":
            return "u64";
        case "JSON":
            return "TOOD:";
        default:
            return null;
    }
};
const genModel = ({ name, fields }) => {
    const fieldRows = fields.map((field) => {
        return `    pub ${toSnakeCase(field.name)}: ${convertToRustType(field.type)},`;
    });
    return `#[derive(sqlx::FromRow)]
pub struct DB${toPascelCase(name)} {
${fieldRows.join("\n")}
}`;
};
exports.genModel = genModel;
//# sourceMappingURL=genEnum.js.map