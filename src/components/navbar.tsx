import { Flex, Heading, IconButton, useColorMode } from "@chakra-ui/react";
import { useCallback } from "react";
import Logo from "./logo.tsx";
import { Moon01, Sun } from "@untitled-ui/icons-react";

export default function Navbar({ text = "" }) {
  const { colorMode, setColorMode } = useColorMode();
  const handleColorChange = useCallback(() => {
    setColorMode(colorMode === "light" ? "dark" : "light");
  }, [colorMode]);
  return (
    <Flex
      h="10"
      position="sticky"
      top="0"
      alignItems="center"
      borderBottomColor=""
    >
      <div>
        <Logo />
      </div>
      <Heading w="100%" textAlign="center" lineHeight="1" size="sm">
        {text}
      </Heading>
      <Flex grow={1} alignItems="center" gap={2}>
        <IconButton aria-label="Theme" onClick={handleColorChange}>
          {colorMode === "dark" ? <Sun /> : <Moon01 />}
        </IconButton>
      </Flex>
    </Flex>
  );
}