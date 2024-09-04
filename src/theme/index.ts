// theme.js
import { extendTheme } from "@chakra-ui/react";
import BadgeTheme from "./badge.theme.ts";
import ButtonTheme from "./button.theme.ts"; // Customizing the Button component
import "@fontsource/ibm-plex-mono"; // Customizing the Button component
import "@fontsource/fira-mono";
import { linkTheme } from "./link.theme.ts";

export const theme = extendTheme({
  fonts: {
    heading: "'IBM Plex Mono', monospace",
    body: "'Open sans', sans-serif",
  },
  colors: {},
  components: {
    Button: ButtonTheme,
    Badge: BadgeTheme,
    Link: linkTheme,
  },
});
