import { defineStyle, defineStyleConfig } from "@chakra-ui/react";

const teal = defineStyle({
  textDecoration: "none",
  color: "black",
  backgroundColor: "teal.200",
  px: 4,
  py: 2,
  borderRadius: 5,
  // let's also provide dark mode alternatives
  _dark: {
    backgroundColor: "teal.900",
    color: "white",
  },
});

export const linkTheme = defineStyleConfig({
  variants: { teal },
});