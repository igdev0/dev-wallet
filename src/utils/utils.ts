const regex = /^((\d+'?)(\/\d+'?)*)$/;

export function isValidDerivationPath(path: string): boolean {
  return regex.test(path);
}
