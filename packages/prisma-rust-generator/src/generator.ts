import { generatorHandler, GeneratorOptions } from "@prisma/generator-helper";
import path from "path";
import { genModel } from "./helpers/genModels";
import { writeFileSafely } from "./utils/writeFileSafely";
import { toSnakeCase } from "./utils/stringFormats";

generatorHandler({
  onManifest() {
    return {
      version: "0.0.0",
      defaultOutput: "../generated",
      prettyName: "prisma-rust-generator",
    };
  },
  onGenerate: async (options: GeneratorOptions) => {
    const output = options.generator.output?.value!;

    await Promise.all(
      options.dmmf.datamodel.models.map(async (modelInfo) => {
        const model = genModel(modelInfo);
        const writeLocation = path.join(
          output,
          `${toSnakeCase(modelInfo.name)}.rs`
        );
        await writeFileSafely(writeLocation, model);
      })
    );

    options.dmmf.datamodel.models.map((modelInfo) => {
      return `pub mod ${modelInfo.name}`;
    });
  },
});
