import { DMMF } from "@prisma/generator-helper";
import { toCapital, toPascelCase } from "../utils/stringFormats";

export const genEnum = ({ name, values }: DMMF.DatamodelEnum) => {
  const fieldRows = values.map((field) => {
    return `    ${toCapital(field.name)},`;
  });

  return `pub enum ${toPascelCase(name)} {
${fieldRows.join("\n")}
}`;
};
