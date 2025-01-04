export function getCssVariable(variableName: string): string {
  const rootStyles = getComputedStyle(document.documentElement);
  return rootStyles.getPropertyValue(`--${variableName}`).trim();
}
