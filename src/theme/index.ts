// theme.js
import {extendTheme} from "@chakra-ui/react";
import BadgeTheme from "./badge.theme.ts";
import ButtonTheme from "./button.theme.ts"; // Customizing the Button component
// Customizing the Button component

export const theme = extendTheme({
  components: {
    Button: ButtonTheme,
    Badge: BadgeTheme,
  },
});
