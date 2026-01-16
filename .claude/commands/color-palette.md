# StoryBud Color Palette

Complete color system defined in `constants/theme.ts`

## Brand Colors (BrandColors)

```typescript
primary: '#7C3AED'        // StoryBud Purple - Main brand
primaryLight: '#A78BFA'   // Lighter purple for dark mode
secondary: '#6BA3DB'      // StoryBud Light Blue
secondaryLight: '#93C5FD' // Lighter blue for dark mode
accent: '#FFBF00'         // StoryBud Amber
accentLight: '#FCD34D'    // Brighter amber for dark mode
```

**Usage**: Tab icons, buttons, highlights, active states

## Neutral Colors

```typescript
white: '#FFFFFF'
offWhite: '#FFFDF7'       // Warm off-white
beigeLight: '#FFFDF7'     // Light beige background
beigeDark: '#F5F2E8'      // Warm beige
lightGray: '#E9ECEF'      // Borders, dividers
mediumGray: '#6B7280'     // Secondary text, icons
darkGray: '#333333'       // Primary text (light mode)
nearlyBlack: '#0F172A'    // Dark backgrounds
darkBg: '#1A1C23'         // Very dark background
```

## Interactive Colors

```typescript
link: '#5C7CFA'           // Bright blue for links
linkHover: '#4263EB'      // Darker blue on hover
linkHoverDark: '#748FFC'  // Lighter blue (dark mode)
highlight: '#FFD43B'      // Yellow highlight
highlightActive: '#FFD60A' // Bright golden (active word)
error: '#EF4444'          // Red for errors
errorDark: '#7F1D1D'      // Darker red (dark mode)
success: '#059669'        // Green for success
```

## Story Reader Colors

**Light Mode:**
```typescript
background: '#FFFFFF'
text: '#333333'
muted: '#777777'
paper: '#FFFDF7'
accent: '#5C7CFA'
accentHover: '#4263EB'
progress: '#5C7CFA'
cardBg: '#FFFFFF'
cardHover: '#F8F9FA'
overlay: 'rgba(0, 0, 0, 0.7)'
```

**Dark Mode:**
```typescript
background: '#1A1C23'
text: '#E5E7EB'
muted: '#9CA3AF'
paper: '#2C2E3E'
accent: '#5C7CFA'
accentHover: '#748FFC'
progress: '#5C7CFA'
cardBg: '#252836'
cardHover: '#2D303E'
overlay: 'rgba(0, 0, 0, 0.8)'
```

## Parchment Colors (Book Aesthetic)

```typescript
background: '#F8F1DF'
warmBrown: '#8B6F47'
darkBrown: '#5C4033'
tan: '#C4A882'
lightTan: '#D4A574'
textDark: '#2C2416'
errorBrown: '#8B4513'
```

## Usage

```tsx
import { Colors, BrandColors } from '@/constants/theme';
import { useColorScheme } from 'react-native';

const colorScheme = useColorScheme();
const colors = Colors[colorScheme ?? 'light'];

// Use colors
<Text style={{ color: colors.text }}>Hello</Text>
<View style={{ backgroundColor: BrandColors.primary }} />
```

## Storybook Components (Auto-Theme)

```tsx
import { useColorScheme } from 'react-native';
import { createStyles } from './styles';

const colorScheme = useColorScheme();
const styles = useMemo(() => createStyles(colorScheme), [colorScheme]);
```

## Guidelines

1. **Never hardcode colors** - Always use theme system
2. **Support dark mode** - Use `useColorScheme()`
3. **Maintain WCAG AA contrast** - Test text readability
4. **Consistent states** - Purple for active/selected
5. **Reader adapts** - Parchment (light) / dark paper (dark)
