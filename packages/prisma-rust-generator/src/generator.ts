import { generatorHandler, GeneratorOptions } from "@prisma/generator-helper";
import path from "path";
import { genModel } from "./helpers/genModels";
import { writeFileSafely } from "./utils/writeFileSafely";
import { toSnakeCase } from "./utils/stringFormats";
import { genEnum } from "./helpers/genEnums";

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

    // Generate models
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

    // Generate Enums
    await Promise.all(
      options.dmmf.datamodel.enums.map(async (enumInfo) => {
        const enumStr = genEnum(enumInfo);
        const writeLocation = path.join(
          output,
          `${toSnakeCase(enumInfo.name)}.rs`
        );
        await writeFileSafely(writeLocation, enumStr);
      })
    );

    // Export models
    const mod = options.dmmf.datamodel.models.map((modelInfo) => {
      return `pub mod ${toSnakeCase(modelInfo.name)};`;
    });

    await writeFileSafely(path.join(output, "mod.rs"), mod.join("\n"));
  },
});
