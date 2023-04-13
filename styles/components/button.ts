// interface Button {
//     background: StateIntensities
//     label: {
//         text: string
//         color: StateIntensities
//     }
//     icon: {
//         intensity: StateIntensities
//     }
// }

// function contrastRatio(intensity1: number, intensity2: number): number {
//     const [intensityLighter, intensityDarker] =
//         intensity1 > intensity2
//             ? [intensity1, intensity2]
//             : [intensity2, intensity1]
//     return (intensityLighter + 0.5) / (intensityDarker + 0.5)
// }

// function hasSufficientContrast(
//     intensity1: number,
//     intensity2: number,
//     minContrast: number
// ): boolean {
//     return contrastRatio(intensity1, intensity2) >= minContrast
// }

// function createButton(
//     themeConfig: Theme,
//     labelText: string,
//     backgroundIntensity: number,
//     labelIntensity: number,
//     iconIntensity: number
// ): Button | null {
//     const backgroundStates = buildStateIntensities(
//         themeConfig,
//         backgroundIntensity
//     )
//     const labelStates = buildStateIntensities(themeConfig, labelIntensity)
//     const iconStates = buildStateIntensities(themeConfig, iconIntensity)

//     // Ensure sufficient contrast for all states
//     const minContrast = 3
//     const states = ["default", "hovered", "pressed", "active"] as const
//     for (const state of states) {
//         if (
//             !hasSufficientContrast(
//                 backgroundStates[state],
//                 labelStates[state],
//                 minContrast
//             ) ||
//             !hasSufficientContrast(
//                 backgroundStates[state],
//                 iconStates[state],
//                 minContrast
//             )
//         ) {
//             console.warn(
//                 `Insufficient contrast for state "${state}". Please adjust intensities.`
//             )
//             return null
//         }
//     }

//     const button: Button = {
//         background: backgroundStates,
//         label: {
//             text: labelText,
//             color: labelStates,
//         },
//         icon: {
//             intensity: iconStates,
//         },
//     }

//     return button
// }

// const lightButton = createButton(lightTheme, "Click me!", 50, 100, 100)
// console.log(lightButton)
// // {
// //   background: { default: 50, hovered: 53, pressed: 56, active: 59 },
// //   label: { text: 'Click me!', color: { default: 100, hovered: 100, pressed: 100, active: 100 } },
// //   icon: { intensity: { default: 100, hovered: 100, pressed: 100, active: 100 } }
// // }

// const darkButton = createButton(darkTheme, "Click me!", 50, 1, 1)
// console.log(darkButton)
// // {
// //   background: { default: 50, hovered: 65, pressed: 70, active: 75 },
// //   label: { text: 'Click me!', color: { default: 1, hovered: 1, pressed: 1, active: 1 } },
// //   icon: { intensity: { default: 1, hovered: 1, pressed: 1, active: 1 } }
// // }
