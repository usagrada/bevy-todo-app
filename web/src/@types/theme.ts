import { theme } from '../view/theme/theme';

export interface MyTheme {
  colors: typeof theme.colors;
}

export interface MyThemeOptions {
  colors?: typeof theme.colors;
}
