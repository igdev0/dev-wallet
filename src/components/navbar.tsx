import {
  Flex,
  Heading,
  IconButton,
  Switch,
  useColorMode,
  useToken,
} from "@chakra-ui/react";
import { ChangeEvent, useCallback } from "react";
import Logo from "./logo.tsx";
import { Plus } from "@untitled-ui/icons-react";

export default function Navbar({ text = "" }) {
  const { colorMode, setColorMode } = useColorMode();
  const [bdcDark, bdcLight] = useToken("colors", ["white", "black"]);
  const handleColorChange = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => {
      setColorMode(event.currentTarget.checked ? "dark" : "light");
    },
    [colorMode],
  );
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
        <IconButton isRound={true} size="sm">
          <Plus />
        </IconButton>
        <Switch
          colorScheme="orange"
          onChange={handleColorChange}
          defaultChecked={colorMode === "dark"}
        />
      </Flex>
    </Flex>
  );
}