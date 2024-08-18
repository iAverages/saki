export const toCapital = (name: string) => {
  return name.charAt(0).toUpperCase() + name.substring(1);
};

export const toPascelCase = (name: string) => {
  return name
    .split(/\s+|_|-+/)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join("");
};

export const toSnakeCase = (name: string) => {
  return name
    .split(/\s+|_|-+/)
    .map((word) => word.toLowerCase())
    .join("_");
};
