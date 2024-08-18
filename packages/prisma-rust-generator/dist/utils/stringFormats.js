"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.toSnakeCase = exports.toPascelCase = void 0;
const toPascelCase = (name) => {
    return name
        .split(/\s+|_|-+/)
        .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
        .join("");
};
exports.toPascelCase = toPascelCase;
const toSnakeCase = (name) => {
    return name
        .split(/\s+|_|-+/)
        .map((word) => word.toLowerCase())
        .join("_");
};
exports.toSnakeCase = toSnakeCase;
//# sourceMappingURL=stringFormats.js.map