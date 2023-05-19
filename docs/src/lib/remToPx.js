export function remToPx(remValue) {
  let rootFontSize =
    typeof window === 'undefined'
      ? 16
      : parseFloat(window.getComputedStyle(document.documentElement).fontSize)

  return parseFloat(remValue) * rootFontSize
}
