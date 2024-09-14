const regex = /^((\d+'?)(\/\d+'?)*)$/;

export function isValidDerivationPath(path: string): boolean {
  return regex.test(path);
}

export function removeRef(data: object) {
  return JSON.parse(JSON.stringify(data));
}
