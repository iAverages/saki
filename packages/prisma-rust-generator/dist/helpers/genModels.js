"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.genModel = void 0;
const stringFormats_1 = require("../utils/stringFormats");
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
        return `    pub ${(0, stringFormats_1.toSnakeCase)(field.name)}: ${convertToRustType(field.type)},`;
    });
    return `#[derive(sqlx::FromRow)]
pub struct DB${(0, stringFormats_1.toPascelCase)(name)} {
${fieldRows.join("\n")}
}`;
};
exports.genModel = genModel;
//# sourceMappingURL=genModels.js.map