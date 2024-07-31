// theme/components/badge.js
import { mode } from "@chakra-ui/theme-tools";

const BadgeTheme = {
  baseStyle: {
    display: "inline-block",
    textTransform: "unset",
    fontSize: "xs",
    fontWeight: "bold",
    borderRadius: "full",
    px: 2,
    py: 1,
  },
  variants: {
    solid: (props) => ({
      bg: mode(`${props.colorScheme}.500`, `${props.colorScheme}.200`)(props),
      color: mode(`white`, `gray.800`)(props),
    }),
    subtle: (props) => ({
      bg: mode(`${props.colorScheme}.100`, `${props.colorScheme}.700`)(props),
      color: mode(`${props.colorScheme}.800`, `white`)(props),
    }),
    outline: (props) => ({
      color: mode(
        `${props.colorScheme}.500`,
        `${props.colorScheme}.200`,
      )(props),
      boxShadow: "inset 0 0 0px 1px",
      borderColor: mode(
        `${props.colorScheme}.500`,
        `${props.colorScheme}.200`,
      )(props),
    }),
  },
  defaultProps: {
    variant: "subtle",
    colorScheme: "gray",
  },
};

export default BadgeTheme;
