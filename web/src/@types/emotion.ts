import '@emotion/react';
import { MyTheme, MyThemeOptions } from './theme';

declare module '@emotion/react' {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface Theme extends MyTheme {}
}
