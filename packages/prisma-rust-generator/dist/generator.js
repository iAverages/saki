"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const generator_helper_1 = require("@prisma/generator-helper");
const path_1 = __importDefault(require("path"));
const genModels_1 = require("./helpers/genModels");
const writeFileSafely_1 = require("./utils/writeFileSafely");
const stringFormats_1 = require("./utils/stringFormats");
(0, generator_helper_1.generatorHandler)({
    onManifest() {
        return {
            version: "0.0.0",
            defaultOutput: "../generated",
            prettyName: "prisma-rust-generator",
        };
    },
    onGenerate: async (options) => {
        var _a;
        const output = (_a = options.generator.output) === null || _a === void 0 ? void 0 : _a.value;
        await Promise.all(options.dmmf.datamodel.models.map(async (modelInfo) => {
            const model = (0, genModels_1.genModel)(modelInfo);
            const writeLocation = path_1.default.join(output, `${(0, stringFormats_1.toSnakeCase)(modelInfo.name)}.rs`);
            await (0, writeFileSafely_1.writeFileSafely)(writeLocation, model);
        }));
    },
});
//# sourceMappingURL=generator.js.map