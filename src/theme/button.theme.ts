// theme/components/button.js
import { mode } from "@chakra-ui/theme-tools";

const ButtonTheme = {
  baseStyle: {
    lineHeight: "1.2",
    borderRadius: "md",
    fontWeight: "semibold",
    _focus: {
      boxShadow: "outline",
    },
    _disabled: {
      opacity: 0.4,
      cursor: "not-allowed",
      boxShadow: "none",
    },
  },
  sizes: {
    lg: {
      h: 12,
      minW: 12,
      fontSize: "lg",
      px: 6,
    },
    md: {
      h: 10,
      minW: 10,
      fontSize: "md",
      px: 4,
    },
    sm: {
      h: 8,
      minW: 8,
      fontSize: "sm",
      px: 3,
    },
    xs: {
      h: 6,
      minW: 6,
      fontSize: "xs",
      px: 2,
    },
  },
  variants: {
    ghost: (props) => ({
      color: mode(
        `${props.colorScheme}.500`,
        `${props.colorScheme}.200`,
      )(props),
      _hover: {
        bg: mode(`${props.colorScheme}.50`, `${props.colorScheme}.800`)(props),
      },
      _active: {
        bg: mode(`${props.colorScheme}.100`, `${props.colorScheme}.700`)(props),
      },
    }),
    outline: (props) => ({
      border: "1px solid",
      borderColor: mode(
        `${props.colorScheme}.500`,
        `${props.colorScheme}.200`,
      )(props),
      color: mode(
        `${props.colorScheme}.500`,
        `${props.colorScheme}.200`,
      )(props),
      _hover: {
        bg: mode(`${props.colorScheme}.50`, `${props.colorScheme}.800`)(props),
      },
      _active: {
        bg: mode(`${props.colorScheme}.100`, `${props.colorScheme}.700`)(props),
      },
    }),
    solid: (props) => ({
      bg: mode(`${props.colorScheme}.500`, `${props.colorScheme}.200`)(props),
      color: mode("white", "gray.800")(props),
      _hover: {
        bg: mode(`${props.colorScheme}.600`, `${props.colorScheme}.300`)(props),
      },
      _active: {
        bg: mode(`${props.colorScheme}.700`, `${props.colorScheme}.400`)(props),
      },
    }),
    link: (props) => ({
      padding: 0,
      height: "auto",
      lineHeight: "normal",
      verticalAlign: "baseline",
      color: mode(
        `${props.colorScheme}.500`,
        `${props.colorScheme}.200`,
      )(props),
      _hover: {
        textDecoration: "underline",
      },
      _active: {
        color: mode(
          `${props.colorScheme}.700`,
          `${props.colorScheme}.500`,
        )(props),
      },
    }),
  },
  defaultProps: {
    variant: "solid",
    size: "md",
    colorScheme: "gray",
  },
};

export default ButtonTheme;
